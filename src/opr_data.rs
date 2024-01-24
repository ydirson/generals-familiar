use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::{deserialize_number_from_string,
                                  deserialize_string_from_number,
};
use std::rc::Rc;

pub const GET_ARMY_BASE_URL: &str = "https://army-forge.onepagerules.com/api/tts";

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Army {
    pub id: String,
    pub name: String,
    pub game_system: String,
    pub points: usize,
    pub points_limit: usize,
    //pub special_rules: Vec<Rc<SpecialRuleDef>>,
    pub units: Vec<Rc<Unit>>,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Unit {
    pub id: String,
    pub name: String,
    pub cost: usize,
    #[serde(default)]
    pub custom_name: String,
    pub size: usize,
    pub quality: usize,
    pub defense: usize,
    pub special_rules: Vec<Rc<SpecialRule>>,
    // FIXME loadout instead?
    pub equipment: Vec<Rc<Equipment>>,
    // FIXME army_id for regrouping
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct SpecialRule {
    pub name: String,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub rating: String,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Equipment {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub range: usize,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub attacks: usize,
    pub count: usize,
    pub special_rules: Vec<Rc<SpecialRule>>,
}

mod static_data;
#[test]
fn test_load_parse() {
    for (i, json_string) in static_data::JSON_DATA.iter().enumerate() {
        let _army_list: Rc<Army> = serde_json::from_str(json_string)
            .expect(format!("should parse data {i} as json").as_str());
    }
}
