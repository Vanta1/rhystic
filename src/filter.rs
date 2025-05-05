use chumsky::{
    error,
    input::{Stream, ValueInput},
    prelude::*,
};
use logos::{Logos, Span};

/// the parsed expression
#[derive(Debug)]
pub enum SfExpr<'a> {
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    Filter {
        property: &'a str,
        condition: &'a str,
        value: &'a str,
    },
    Title(&'a str),
}

#[derive(Logos)]
enum SfFilter<'a> {
    Error,
    #[regex("[A-Za-z]+")]
    Property(&'a str),
    #[regex("(:|!=|=|>=|>|<=|<)")]
    Condition(&'a str),
    #[regex(r#"('[^']*'|"[^"]*"|[A-Za-z0-9{}\/]*)"#)]
    Value(&'a str),
}

fn parse_filter<'a>(src: &'a str) -> SfExpr<'a> {
    let mut lexer = SfFilter::lexer(src).spanned().map(|(tok, span)| match tok {
        Ok(tok) => (tok, span),
        Err(_) => (SfFilter::Error, span),
    });

    let property = match lexer.next() {
        Some(token) => match token {
            (SfFilter::Property(s), _) => s,
            _ => panic!("lexer failed for filter"),
        },
        _ => panic!("lexer failed unexpectedly while parsing filter property"),
    };
    let condition = match lexer.next() {
        Some(token) => match token {
            (SfFilter::Condition(s), _) => s,
            _ => panic!("lexer failed for filter"),
        },
        _ => panic!("lexer failed unexpectedly while parsing filter condition"),
    };
    let value = match lexer.next() {
        Some(token) => match token {
            (SfFilter::Value(s), _) => s,
            _ => panic!("lexer failed for filter"),
        },
        _ => panic!("lexer failed unexpectedly while parsing filter value"),
    };

    SfExpr::Filter {
        property,
        condition,
        value,
    }
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
    #[regex(r#"[A-Za-z]+(:|!=|=|>=|>|<=|<)('[^']*'|"[^"]*"|[A-Za-z0-9{}\/]*)"#)]
    Filter(&'a str),
    #[regex(r#"([A-Za-z0-9'^:]*|"[^"]*")"#)]
    Title(&'a str),
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
        SfToken::Filter(t) => parse_filter(t),
        SfToken::Title(t) => SfExpr::Title(t),
    };

    let ops = filter.foldl(
        just(SfToken::Or).or_not().then(filter).repeated(),
        |lhs, (or, rhs)| match or {
            Some(_) => SfExpr::Or(Box::new(lhs), Box::new(rhs)),
            None => SfExpr::And(Box::new(lhs), Box::new(rhs)),
        },
    );

    ops.or(filter)
}
