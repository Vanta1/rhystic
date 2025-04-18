use chumsky::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum SfToken<'a> {
    Name(&'a str),
    Filter { property: &'a str, value: &'a str },
    Or,
    Nest(Vec<Self>),
}

pub fn lex_sf<'a>() -> impl Parser<'a, &'a str, Vec<SfToken<'a>>> {
    // the plan so far:
    // try and parse each space-seperated argument
    // first check if it's a non-title based filter,
    // then if it's a nested filter,
    // and if these fail add it to the title filter

    recursive(|token| {
        choice((
            choice((just("or"), just("oR"), just("Or"), just("OR"))).to(SfToken::Or),
            text::ident()
                .or(text::int(10))
                .separated_by(just(':'))
                .exactly(2)
                .collect::<Vec<&'a str>>()
                .map(|i| SfToken::Filter {
                    property: i.first().unwrap(),
                    value: i.last().unwrap(),
                }),
            text::ident().map(SfToken::Name),
            token
                .repeated()
                .collect()
                .delimited_by(just('('), just(')'))
                .map(SfToken::Nest),
        ))
        .padded()
    })
    .repeated()
    .collect()
}
