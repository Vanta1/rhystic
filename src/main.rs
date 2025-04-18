#![allow(unused)] // TODO: REMOVE

use chumsky::prelude::*;
use serde::Deserialize;

#[derive(Debug)]
enum SfFilter<'a> {
    Filter(&'a str),
}

fn _parse_sf<'a>() /*-> impl Parser<'a, &'a str, SfFilter<'a>> */
{
    // the plan so far:
    // try and parse each space-seperated argument
    // first check if it's a non-title based filter,
    // then if it's a nested filter,
    // and if these fail add it to the title filter

    //let sf_filter = text::ident();
}

#[derive(Deserialize, Debug)]
struct ScCard {
    cmc: f32,
    game_changer: Option<bool>,
    id: String,
    loyalty: Option<String>,
    mana_cost: Option<String>,
    name: String,
    oracle_text: Option<String>,
    power: Option<String>,
    rarity: String,
    reprint: bool,
    set_name: String,
    set: String,
    tougness: Option<String>,
    type_line: String,
}

#[derive(Deserialize)]
struct ScCardPool {
    pub cards: Vec<ScCard>,
}

fn main() {
    //println!("{:?}", parse_sf().parse("lightning t:instant"));

    let now = std::time::Instant::now();

    let cards: Vec<ScCard> =
        serde_json::from_str(include_str!("../res/bulk/oracle-cards-20250417210525.json")).unwrap();

    let elapsed_time = now.elapsed();
    println!("deserialized in {} ms", elapsed_time.as_millis());

    let now = std::time::Instant::now();

    let filtered: Vec<ScCard> = cards
        .into_iter()
        .filter(|c| match &c.oracle_text {
            Some(ot) => ot.contains("enters"),
            None => false,
        })
        .filter(|c| match &c.power {
            Some(pow) => match pow.parse::<f32>().ok() {
                Some(p) => true,
                None => false,
            },
            None => false,
        })
        .filter(|c| &c.name == "Spirited Companion")
        .collect();

    let elapsed_time = now.elapsed();
    println!("filtered in {} ms", elapsed_time.as_millis());

    dbg!(filtered.first(), filtered.len());
}
