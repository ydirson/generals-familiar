use gloo_net::http::Request;
use leptonic::prelude as ltn;
use leptos::*;
use std::rc::Rc;

const APP_NAME: &str = "General's Familiar";
const PLAYER_NAMES: [&str; 2] = ["Player 1", "Player 2"];
const ARMY_IDS: [&str; 2] = [
//    "nLBrzTpB1TTJ",
//    "ybjR2-7kHUNY",
    "Mlwpoh1AGLC2",
    "p2KIbSBOYpSB",
];

fn main() {
    #[cfg(feature = "panic_hook")]
    console_error_panic_hook::set_once();

    set_app_name(APP_NAME);
    mount_to_body(|| view! { <App/> })
}

fn set_app_name(app_name: &str) {
    let doc = web_sys::window()
        .expect("should have a window")
        .document()
        .expect("window should have a document");
    let title = doc.create_element("title").expect("should create title");
    title.set_text_content(Some(app_name));
    doc
        .head()
        .expect("document should have a head")
        .append_child(&title)
        .expect("should set document title");
}

#[component]
fn App() -> impl IntoView {
    let (army_id0, set_army_id0) = create_signal(ARMY_IDS[0].to_string());
    let (army_id1, set_army_id1) = create_signal(ARMY_IDS[1].to_string());
    let (unitsel0, set_unitsel0) = create_signal(None::<Rc<opr::Unit>>);
    let (unitsel1, set_unitsel1) = create_signal(None::<Rc<opr::Unit>>);
    view! {
        <ltn::Root default_theme=ltn::LeptonicTheme::default()>
            <ltn::Box style="min-height: 100vh;">
                <ltn::AppBar>
                    <h1>{APP_NAME}</h1>
                    <ltn::ThemeToggle off=ltn::LeptonicTheme::Light on=ltn::LeptonicTheme::Dark/>
                </ltn::AppBar>
                <DetailsDrawer side=ltn::DrawerSide::Left unit_selection=unitsel0 />
                <ltn::Stack orientation=ltn::StackOrientation::Horizontal
                       spacing=ltn::Size::Em(1.0)
                       style="margin-right: 1em; align-items: flex-start;">
                    <ArmyList army_id=army_id0
                              player_name=PLAYER_NAMES[0].to_string()
                              select_unit=set_unitsel0
                    />
                    <ArmyList army_id=army_id1
                              player_name=PLAYER_NAMES[1].to_string()
                              select_unit=set_unitsel1
                     />
                </ltn::Stack>
                <DetailsDrawer side=ltn::DrawerSide::Right unit_selection=unitsel1 />
            </ltn::Box>
        </ltn::Root>
    }
}

#[component]
fn ArmyList(player_name: String,
            army_id: ReadSignal<String>,
            select_unit: WriteSignal<Option<Rc<opr::Unit>>>,
) -> impl IntoView {
    let army_data = create_resource(
        move || army_id.get(),
        |army_id_value| async move { load_data(&army_id_value).await });

    view! {
        <ltn::Stack spacing=ltn::Size::Em(0.5)>
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
                            <UnitsList units={units.clone()} select_unit />
                        }
                    }.into_view()
                }
            )}
        </ltn::Stack>
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
fn UnitsList(units: Vec<Rc<opr::Unit>>,
             select_unit: WriteSignal<Option<Rc<opr::Unit>>>,
) -> impl IntoView {
    view! {
        <ltn::TableContainer>
            <ltn::Table bordered=true hoverable=true>
                <ltn::TableBody>
                    {move || {
                        units
                            .clone()
                            .into_iter()
                            .map(|unit| {
                                let unit_name = (*unit).formatted_name();
                                //let opr::Unit{..} = *unit;
                                view! {
                                    <ltn::TableRow on:click=move |_| {
                                        select_unit.set(Some(unit.clone()));
                                    }>
                                        <ltn::TableCell> {unit_name} </ltn::TableCell>
                                    </ltn::TableRow>
                                }
                            })
                            .collect_view()
                    }}
                </ltn::TableBody>
            </ltn::Table>
        </ltn::TableContainer>
    }
}

#[component]
fn DetailsDrawer(side: ltn::DrawerSide,
                 unit_selection: ReadSignal<Option<Rc<opr::Unit>>>) -> impl IntoView {
    let pos_class = match side {
        ltn::DrawerSide::Left => "left",
        ltn::DrawerSide::Right => "right",
    };

    // the `shown` status can be changed by eg. selecting in the army
    // list, or using close button in the drawer itself
    let (shown, set_shown) = create_signal(false);
    create_effect(move |_| {
        set_shown.set(unit_selection.with(Option::is_some));
    });

    view! {
        <ltn::Drawer side shown class={format!("army_details {pos_class}")}>
            <Show when={move || shown.get()}>
                <UnitDetails side set_shown unit=unit_selection.get().unwrap() />
            </Show>
        </ltn::Drawer>
    }
}

#[component]
fn UnitDetails(unit: Rc<opr::Unit>,
               side: ltn::DrawerSide,
               set_shown: WriteSignal<bool>) -> impl IntoView
{
    let unit_name = unit.formatted_name();
    let opr::Unit{quality, defense, ref loadout, ref special_rules, ..} = *unit;
    let (left_button, right_button) = match side {
        ltn::DrawerSide::Left => (
            Some(view!{ <ltn::Button
                             color=ltn::ButtonColor::Secondary
                             on_click=move |_| set_shown.set(false)> "<" </ltn::Button> }),
            None),
        ltn::DrawerSide::Right => (
            None,
            Some( view!{ <ltn::Button
                              color=ltn::ButtonColor::Secondary
                              on_click=move |_| set_shown.set(false)> ">" </ltn::Button> })),
    };
    view! {
        <h3>
            <ltn::Stack orientation=ltn::StackOrientation::Horizontal
                        style="width: 100%; justify-content: space-between;"
                        spacing=ltn::Size::Em(0.0)>
                <ltn::Stack orientation=ltn::StackOrientation::Horizontal
                            spacing=ltn::Size::Em(1.0)>
                    {left_button}
                    {format!("{unit_name}: Q{quality} D{defense}")}
                </ltn::Stack>
                {right_button}
            </ltn::Stack>
        </h3>
        <p><SpecialRulesList special_rules={special_rules.clone()} /></p>
        <p><UnitUpgradesList loadout_list={loadout.clone()} /></p>
        <EquipmentList loadout_list={loadout.clone()} />
    }
}

#[component]
fn UnitUpgradesList(loadout_list: Vec<Rc<opr::UnitLoadout>>) -> impl IntoView {
    view! {
        {move || {
            loadout_list
                .clone()
                .into_iter()
                .filter(|loadout|
                        if let opr::UnitLoadout::Upgrade{..} = **loadout
                        { true } else { false })
                .enumerate()
                .map(|(i, loadout)| {
                    if let opr::UnitLoadout::Upgrade(ref upgrade) = *loadout {
                        let opr::UnitUpgrade{name, ref content, ..} = upgrade;
                        view! {
                            {move || if i > 0 { ", " } else { "" }}
                            {name} " (" <SpecialRulesList special_rules={content.clone()} /> ")"
                        }
                    } else {
                        panic!();
                    }
                })
                .collect_view()
        }}
    }
}

#[component]
fn EquipmentList(loadout_list: Vec<Rc<opr::UnitLoadout>>) -> impl IntoView {
    view! {
        <ltn::TableContainer>
            <ltn::Table bordered=true hoverable=true>
                <ltn::TableBody>
                    {move || {
                        loadout_list
                            .clone()
                            .into_iter()
                            .filter(|loadout|
                                    if let opr::UnitLoadout::Equipment{..} = **loadout
                                    { true } else { false })
                            .map(|loadout| {
                                view! {
                                    <EquipmentItem loadout />
                                }
                            })
                            .collect_view()
                    }}
                </ltn::TableBody>
            </ltn::Table>
        </ltn::TableContainer>
    }
}

#[component]
fn EquipmentItem(loadout: Rc<opr::UnitLoadout>) -> impl IntoView {
    if let opr::UnitLoadout::Equipment(ref equipment) = *loadout {
        let name = equipment.name.clone();
        let special_rules = equipment.special_rules.clone();
        let opr::Equipment{count, range, attacks, ..} = *equipment;
        view! {
            <ltn::TableRow>
                <ltn::TableCell>
                    {if count != 1
                        {format!("{}x ", count)} else {"".to_string()}}
                    {name}
                </ltn::TableCell>
                <ltn::TableCell>
                    {if range != 0
                        {format!(r#"{}""#, range )}
                        else {"-".to_string()}}
                </ltn::TableCell>
                <ltn::TableCell>
                    {format!("A{}", attacks)}
                </ltn::TableCell>
                <ltn::TableCell>
                    <SpecialRulesList special_rules={special_rules.clone()} />
                </ltn::TableCell>
            </ltn::TableRow>
        }
    } else {
        panic!("EquipmentItem must be used on Equipment only");
    }
}

#[component]
fn SpecialRulesList(special_rules: Vec<Rc<opr::SpecialRule>>) -> impl IntoView {
    special_rules.iter()
        // render each rule
        .map(|special_rule| {
            let rating = match special_rule.rating.as_str() {
                "" => { "".to_string() },
                rating => { format!("({})", rating) },
            };
            view! {
                {format!("{name}{rating}", name=special_rule.name, rating=rating)}
            }
        })
        .collect::<Vec<String>>()
        .join(", ")
}
