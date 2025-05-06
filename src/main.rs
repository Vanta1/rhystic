mod card;
mod filter;

use card::ScCard;
use filter::{SfExpr, parse_sf};

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>().join(" ");
    dbg!(&args);

    let filter = parse_sf(args.as_str()).expect("parsing failed");
    dbg!(&filter);

    test_filter(filter);
}

fn test_filter(filter: SfExpr) {
    let now = std::time::Instant::now();

    let cards: Vec<ScCard> =
        serde_json::from_str(include_str!("../res/bulk/oracle-cards-20250417210525.json")).unwrap();

    let elapsed_time = now.elapsed();
    println!("deserialized in {} ms", elapsed_time.as_millis());

    let now = std::time::Instant::now();

    let t = match filter {
        SfExpr::Filter {
            property: _,
            condition: _,
            value,
        } => value,
        _ => panic!(""),
    };

    let filtered: Vec<ScCard> = cards
        .into_iter()
        .filter(|c| c.type_line.contains(t))
        .collect();

    dbg!(filtered.first(), filtered.len());

    let elapsed_time = now.elapsed();
    println!("filtered in {} ms", elapsed_time.as_millis());
}
