mod opr_data;

use leptonic::prelude::*;
use leptos::*;

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
        <AppBar>
            <h1>{APP_NAME}</h1>
        </AppBar>
        <opr_data::ArmyList army_id player_name=PLAYER_NAMES[0].to_string() />
    }
}
