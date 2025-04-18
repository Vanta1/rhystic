use serde::Deserialize;

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct ScCard {
    pub cmc: f32,
    pub game_changer: Option<bool>,
    pub id: String,
    pub loyalty: Option<String>,
    pub mana_cost: Option<String>,
    pub name: String,
    pub oracle_text: Option<String>,
    pub power: Option<String>,
    pub rarity: String,
    pub reprint: bool,
    pub set_name: String,
    pub set: String,
    pub tougness: Option<String>,
    pub type_line: String,
}
