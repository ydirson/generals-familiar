use gloo_net::http::Request;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;
use std::rc::Rc;

const GET_ARMY_BASE_URL: &str = "https://army-forge.onepagerules.com/api/tts";

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Army {
    id: String,
    name: String,
    game_system: String,
    points: usize,
    points_limit: usize,
    units: Vec<Rc<Unit>>,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Unit {
    id: String,
    name: String,
    size: usize,
    quality: usize,
    defense: usize,
    special_rules: Vec<Rc<SpecialRule>>,
    equipment: Vec<Rc<Equipment>>,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct SpecialRule {
    name: String,
    #[serde(default)]
    rating: String,
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Equipment {
    id: String,
    name: String,
    range: usize,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    attacks: usize,
    count: usize,
    special_rules: Vec<Rc<SpecialRule>>,
}

async fn load_data(army_id: &str) -> Rc<Army> {
    let army_url = format!("{GET_ARMY_BASE_URL}?id={army_id}");
    Request::get(army_url.as_str())
        .send()
        .await
        .expect("should get an HTTP answer")
        .json()
        .await
        .expect("should deserialize Army from JSON content")
}

#[component]
pub fn ArmyList(player_name: String, army_id: ReadSignal<String>) -> impl IntoView {
    let army_data = create_resource(
        move || army_id.get(),
        |army_id_value| async move { load_data(&army_id_value).await });

    view! {
        <h2>{player_name} " - "
        {move || army_data.with(
            |army_data| match army_data.clone() {
                None => view! { "Loading..." }.into_view(),
                Some(army) => view! {
                    {move || army.name.clone()}
                }.into_view()
            }
        )}
        </h2>
    }
}
