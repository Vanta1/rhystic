use chumsky::{
    error,
    input::{Stream, ValueInput},
    prelude::*,
};
use logos::Logos;

/// the parsed expression
#[derive(Debug)]
pub enum SfExpr<'a> {
    And(Box<(Self, Self)>),
    Or(Box<(Self, Self)>),
    // property & condition are optional as if they aren't present the filter goes by card title
    Filter {
        property: Option<&'a str>,
        condition: Option<&'a str>,
        value: &'a str,
    },
}

// TODO: add negation (-) and exact (!) operators, for name, filter & nest exprs
#[derive(Debug, Clone, Logos, PartialEq)]
#[logos(error = String)]
pub enum SfToken<'a> {
    Error,
    #[token("or", ignore(case))]
    Or,
    // this regex is a little clunky, so here's a quick explanation:
    // [A-Za-z]+ all filter properties are only these characters, and in this implementation they're case insensitive
    // (:|!=|=|>=|>|<=|<) this matches all possible comparisons/'condition's, note that the order **does** matter otherwise regex will match ">" and then not ">="
    // the above 2 filters are optional, as a filter w/o a property & condition filters by card title
    // ('[^']*'|[A-Za-z0-9]*) the value of the filter can either be any quoted string (to allow for spaces in namse) or an alphanumeric
    #[regex(r#"([A-Za-z]+(:|!=|=|>=|>|<=|<))?('[^']*'|"[^"]*"|[A-Za-z0-9{}\/]*)"#)]
    Filter(&'a str),
    #[regex(r"[ \t\f\n]+", logos::skip)]
    Whitespace,
}

// idea: parse_sf only parses beginnings of quotes, filters or nests
// bc these are the only valid identifiers to start a query off w
// and then pattern match from there (and use default cases as syntax errors)
pub fn parse_sf<'a>(src: &'a str) -> Result<SfExpr<'a>, Vec<error::Rich<'a, SfToken<'a>>>> {
    let lexer = SfToken::lexer(src).spanned().map(|(tok, span)| match tok {
        Ok(tok) => (tok, span.into()),
        Err(_) => (SfToken::Error, span.into()),
    });

    let tokens = Stream::from_iter(lexer).map((0..src.len()).into(), |(t, s): (_, _)| (t, s));

    parser().parse(tokens).into_result()
}

fn parser<'src, I>() -> impl Parser<'src, I, SfExpr<'src>, extra::Err<Rich<'src, SfToken<'src>>>>
where
    I: ValueInput<'src, Token = SfToken<'src>, Span = SimpleSpan>,
{
    let filter = select! {
        SfToken::Filter(t) => SfExpr::Filter {
            property: None,
            condition: None,
            value: t,
        }
    };

    let ops = filter.foldl(
        just(SfToken::Or).or_not().then(filter).repeated(),
        |lhs, (or, rhs)| match or {
            Some(_) => SfExpr::Or(Box::new((lhs, rhs))),
            None => SfExpr::And(Box::new((lhs, rhs))),
        },
    );

    ops.or(filter)
}
