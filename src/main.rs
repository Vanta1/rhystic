mod card;
mod filter;

use std::sync::LazyLock;

use card::SfCard;
use filter::{SfExpr, parse_sf};

pub static CARDS: LazyLock<Vec<SfCard<'static>>> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../res/bulk/oracle-cards-20250417210525.json")).unwrap()
});

fn main() {
    let now = std::time::Instant::now();
    _ = LazyLock::force(&CARDS);
    println!("deserialized in {} ms", now.elapsed().as_millis());

    let args = std::env::args().skip(1).collect::<Vec<String>>().join(" ");
    dbg!(&args);

    let filter = parse_sf(args.as_str()).expect("parsing failed");
    dbg!(&filter);

    let now = std::time::Instant::now();
    test_filter(filter);
    println!("filtered in {} ms", now.elapsed().as_millis());
}

#[allow(unused)]
fn test_filter(filter: SfExpr) {
    let t = match filter {
        SfExpr::Title(value) => value,
        _ => panic!(""),
    };

    let filtered: Vec<&'static SfCard> = CARDS.iter().filter(|c| c.name.eq(t)).collect();

    dbg!(filtered.first(), filtered.len());
}
