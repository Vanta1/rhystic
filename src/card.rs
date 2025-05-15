use std::{
    borrow::Cow,
    hash::{Hash, Hasher},
};

use serde::Deserialize;

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct SfCard<'a> {
    // enforce zero-copy deserialization, necessary b.c. these types can contain escaped characters that need to be modified and therefore owned by serde
    #[serde(borrow)]
    pub oracle_text: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub name: Cow<'a, str>,

    pub cmc: f32,
    pub game_changer: Option<bool>,
    pub id: &'a str,
    pub loyalty: Option<&'a str>,
    pub mana_cost: Option<&'a str>,
    pub power: Option<&'a str>,
    pub rarity: &'a str,
    pub reprint: bool,
    pub set_name: &'a str,
    pub set: &'a str,
    pub tougness: Option<&'a str>,
    pub type_line: &'a str,
    pub reserved: bool,
}

// names are unique identifiers, therefore hashing & comparing them are valid
impl PartialEq for SfCard<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for SfCard<'_> {}

impl Hash for SfCard<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
