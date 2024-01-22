mod opr_data;

use gloo_net::http::Request;
use leptonic::prelude as ltn;
use leptos::*;
use opr_data as opr;
use std::rc::Rc;

const APP_NAME: &str = "General's Familiar";
const PLAYER_NAMES: [&str; 2] = ["Player 1", "Player 2"];
const ARMY_IDS: [&str; 2] = ["ybjR2-7kHUNY", "VV8Zy0GIfOUX"];

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (army_id, set_army_id) = create_signal(ARMY_IDS[0].to_string());
    view! {
        <ltn::AppBar>
            <h1>{APP_NAME}</h1>
        </ltn::AppBar>
        <ArmyList army_id player_name=PLAYER_NAMES[0].to_string() />
    }
}

#[component]
pub fn ArmyList(player_name: String, army_id: ReadSignal<String>) -> impl IntoView {
    let army_data = create_resource(
        move || army_id.get(),
        |army_id_value| async move { load_data(&army_id_value).await });

    view! {
        <h2>{player_name} " - "
        {move || army_data.with(
            |army_data| match army_data {
                None => view! { "Loading..." }.into_view(),
                Some(army) => {
                    let opr::Army{ref game_system, ref name, ..} = **army;
                    view! {
                        {game_system.to_uppercase()}
                        " - "
                        {name}
                    }
                }.into_view()
            }
        )}
        </h2>
        {move || army_data.with(
            |army_data| match army_data {
                None => ().into_view(),
                Some(army) => {
                    let opr::Army{ref units, ..} = **army;
                    view! {
                        <UnitsList units={units.clone()} /*on_click={on_click.clone()}*/ />
                    }
                }.into_view()
            }
        )}
    }
}

async fn load_data(army_id: &str) -> Rc<opr::Army> {
    let army_url = format!("{}?id={army_id}", opr::GET_ARMY_BASE_URL);
    Request::get(army_url.as_str())
        .send()
        .await
        .expect("should get an HTTP answer")
        .json()
        .await
        .expect("should deserialize Army from JSON content")
}

#[component]
fn UnitsList(units: Vec<Rc<opr::Unit>>/*, on_click: &Callback<Rc<Unit>> */) -> impl IntoView {
    view! {
        <ul>
        {move || {
            units
                .clone()
                .into_iter()
                .map(|unit| {
                    let opr::Unit{ref name, ..} = *unit;
                    view! { <li> {name} </li> }
                })
                .collect_view()
        }}
        </ul>
    }
}
