use chumsky::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum SfToken<'a> {
    Name(&'a str),
    Filter {
        property: &'a str,
        condition: &'a str,
        value: &'a str,
    },
    Or,
    Nest(Vec<Self>),
}

pub fn lex_sf<'a>() -> impl Parser<'a, &'a str, Vec<SfToken<'a>>> {
    recursive(|token| {
        // 'or' is the only keyword that appears in the grammar on its own
        // scryfall.com accepts any capitalization of it from testing
        // there is no 'and', because filters are combined by default
        let or = text::ascii::ident::<&str, extra::Default>()
            .filter(|i| match i.to_lowercase().as_str() {
                "or" => true,
                _ => false,
            })
            .to(SfToken::Or)
            .labelled("or clause parser");

        // this matches all possible filter expression, e.g. "c=rg pow<=1 t:enchantment"
        // would find Gruul creatures with power 0 or 1 and the enchantment type.
        //
        // chaining .then() like this creates the clunky tuple-in-a-tuple type for `i`,
        // but it's only used here, so does it really matter?
        let filter = text::ascii::ident()
            .then(choice((
                just(":"),
                just("="),
                just(">"),
                just("<"),
                just(">="),
                just("<="),
                just("!="),
            )))
            .then(text::ident().or(text::int(10)))
            .map(|i: ((&'a str, &'a str), &'a str)| SfToken::Filter {
                // unwrapping is safe here because of the exactly(2) parser.
                property: i.0.0,
                condition: i.0.1,
                value: i.1,
            })
            .labelled("filter parser");

        // TODO: update this to match all non-filter non-'or' UTF-8 sequences, as there are cards with digits & punctuation in the game
        let name = text::unicode::ident().map(SfToken::Name);

        // nested conditions handling, just simplifies down to lots more filters
        let paren = token
            .repeated()
            .collect()
            .delimited_by(just('('), just(')'))
            .map(SfToken::Nest)
            .labelled("nested condition parser");

        choice((or, filter, paren, name)).padded()
    })
    .repeated()
    .collect()
}
