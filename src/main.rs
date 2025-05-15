mod card;
mod filter;

use std::{collections::HashSet, sync::LazyLock};

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
        SfExpr::And(l, r) => match (*l, *r) {
            (SfExpr::Title(t1), SfExpr::Title(t2)) => (t1, t2),
            _ => panic!(""),
        },
        _ => panic!(""),
    };

    let filtered1: Vec<&'static SfCard> = CARDS.iter().filter(|c| c.name.contains(t.0)).collect();
    let filtered2: Vec<&'static SfCard> = CARDS.iter().filter(|c| c.name.contains(t.1)).collect();

    let hs1: HashSet<&&SfCard<'_>> = HashSet::from_iter(&filtered1);
    let hs2: HashSet<&&SfCard<'_>> = HashSet::from_iter(&filtered2);

    let intersection: Vec<&&&SfCard<'_>> = hs1.intersection(&hs2).collect();

    dbg!(
        &filtered1.first(),
        &filtered1.len(),
        &filtered2.first(),
        &filtered2.len(),
        intersection
    );
}
