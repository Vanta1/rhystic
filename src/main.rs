mod card;
mod filter;

use card::ScCard;
use chumsky::Parser;
use filter::lex_sf;

fn main() {
    dbg!(lex_sf().parse("input or (pow:1 or enchantment)"));
}

fn _test_filter() {
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
        .filter(|c| c.type_line.contains("Enchantment"))
        .collect();

    let elapsed_time = now.elapsed();
    println!("filtered in {} ms", elapsed_time.as_millis());

    dbg!(filtered.first(), filtered.len());
}
