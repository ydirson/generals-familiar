use gloo_net::http::Request;
use leptonic::prelude as ltn;
use leptonic::prelude::AlertContent;
use leptos::*;
use leptos_router as ltr;
use leptos_router::Params; // derive(ltr::Params) won't work ?!
use std::rc::Rc;

const APP_NAME: &str = "General's Familiar";
const PLAYER_NAMES: [&str; 2] = ["Player 1", "Player 2"];

// `println!(..)`-style syntax for debugging in browser console
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

fn main() {
    #[cfg(feature = "dev")]
    console_error_panic_hook::set_once();

    set_app_name(APP_NAME);
    mount_to_body(|| view! { <AppBoilerplate/> })
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

/// component dedicated to boilerplate not really part of the app per
/// se, and mandatory parents of the app
#[component]
fn AppBoilerplate() -> impl IntoView {
    view! {
        <ltn::Root default_theme=ltn::LeptonicTheme::default()>
            <ltn::Box style="min-height: 100vh;">
                <ltr::Router fallback=|| view! {
                    <ltn::Alert variant=ltn::AlertVariant::Danger>
                        <AlertContent slot>"Bad URL (route not matched)"</AlertContent>
                    </ltn::Alert>
                }.into_view() >
                    <App/>
                </ltr::Router>
            </ltn::Box>
        </ltn::Root>
    }
}

#[derive(Params,PartialEq)]
struct UrlQuery {
    armies: Option<String>,
}

/// the main application component
#[component]
fn App() -> impl IntoView {
    let query = ltr::use_query::<UrlQuery>();
    let army_ids = move || {
        query.with(|params| {
            match params.as_ref().map(|params| params.armies.clone()) {
                Ok(None) => Ok(None),
                Err(err) => Err(err.to_string()),
                Ok(Some(armies_string)) => {
                    let v = armies_string
                        .split(",")
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>();
                    if v.len() >= 2 {Ok(Some(v))} else {Err("not enough armies".to_string())}
                },
            }
        })
    };
    // army_id's are only read if OK(Some(Vec of length 2)), unwrap() is safe.
    let army_id0 = Signal::derive(move || army_ids().unwrap().unwrap()[0].clone());
    let army_id1 = Signal::derive(move || army_ids().unwrap().unwrap()[1].clone());
    view! {
        <ltn::AppBar>
            <h1>{APP_NAME}</h1>
            <ltn::ThemeToggle off=ltn::LeptonicTheme::Light on=ltn::LeptonicTheme::Dark/>
        </ltn::AppBar>
        <ltn::Box style="padding: 0 1em 1em 1em;">
            <Show when=move || { army_ids().is_ok() }
                  fallback=move || view! { <SelectView message=army_ids().err().unwrap() /> } >
                <Show when=move || { army_ids().unwrap().is_some() }
                      fallback=|| view! { <SelectView message="no army selected".to_string() /> } >
                     <ArmiesView army_id0 army_id1 />
                </Show>
            </Show>
        </ltn::Box>
    }
}

#[component]
fn SelectView(message: String) -> impl IntoView {
    view! {
        <ltn::Alert variant=ltn::AlertVariant::Info>
            <AlertContent slot>{message}</AlertContent>
        </ltn::Alert>

        <h3> "Sample matchups" </h3>
        <ltn::TableContainer>
            <ltn::Table bordered=true hoverable=true>
                <ltn::TableBody>
                    <ltn::TableRow>
                        <ltn::TableCell><a href="./?armies=Mlwpoh1AGLC2,p2KIbSBOYpSB">
                            <em>"Grimdark Future"</em>
                            " — Robots Legion vs. Prime Brothers"
                        </a></ltn::TableCell>
                    </ltn::TableRow>
                    <ltn::TableRow>
                        <ltn::TableCell><a href="./?armies=zhz5uajqHdt5,ZTgIvcYABynP">
                            <em>"Age of Fantasy – Skirmish"</em>
                            " — War Disciples vs. Eternal Wardens"
                        </a></ltn::TableCell>
                    </ltn::TableRow>
                </ltn::TableBody>
            </ltn::Table>
        </ltn::TableContainer>
    }
}

/// the main view, showing multiple armies and providing detail
/// drawers for selections
#[component]
fn ArmiesView(army_id0: Signal<String>,
              army_id1: Signal<String>,
) -> impl IntoView {
    let (unitsel0, set_unitsel0) = create_signal(None::<Rc<opr::Unit>>);
    let (unitsel1, set_unitsel1) = create_signal(None::<Rc<opr::Unit>>);
    view! {
        <DetailsDrawer side=ltn::DrawerSide::Left unit_selection=unitsel0 />
        <ltn::Stack orientation=ltn::StackOrientation::Horizontal
               spacing=ltn::Size::Em(1.0)
               style="align-items: flex-start;">
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
    }
}

#[component]
fn ArmyList(player_name: String,
            army_id: Signal<String>,
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
    let (selected_row_num, set_selected_row_num) = create_signal(None::<usize>);
    view! {
        <ltn::TableContainer>
            <ltn::Table bordered=true hoverable=true>
                <ltn::TableBody>
                    {move || {
                        units
                            .clone()
                            .into_iter()
                            .enumerate()
                            .map(|(i, unit)| {
                                let unit_name = (*unit).formatted_name();
                                //let opr::Unit{..} = *unit;
                                view! {
                                    // FIXME this ought to be a ltn::TableRow, which does
                                    // not allow for dynamic classes or even for class
                                    <leptonic-table-row
                                         class:selected=move || {
                                             match selected_row_num.get() {
                                                 Some(index) if index == i => true,
                                                 _ => false,
                                             }
                                         }
                                         on:click=move |_| {
                                             select_unit.set(Some(unit.clone()));
                                             set_selected_row_num.set(Some(i));
                                    }>
                                        <ltn::TableCell> {unit_name} </ltn::TableCell>
                                    </leptonic-table-row>
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
    let opr::Unit{quality, defense, cost, ref loadout, ref special_rules, ..} = *unit;
    let close_button = |glyph|
        view!{ <ltn::Button color=ltn::ButtonColor::Secondary
                            on_click=move |_| set_shown.set(false)> {glyph} </ltn::Button> };
    let (left_button, right_button) = match side {
        ltn::DrawerSide::Left  => ( Some(close_button("<")), None ),
        ltn::DrawerSide::Right => ( None, Some(close_button(">")) ),
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
                <ltn::Stack orientation=ltn::StackOrientation::Horizontal
                            spacing=ltn::Size::Em(1.0)>
                    {format!("{cost} pts")}
                    {right_button}
                </ltn::Stack>
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
        .enumerate()
        .map(|(i, special_rule)| {
            let separator = if i == 0 { "" } else { ", " };
            let rating = match special_rule.rating.as_str() {
                "" => { "".to_string() },
                rating => { format!("({})", rating) },
            };
            view! {
                {separator}
                <special-rule>
                    {special_rule.name.clone()}
                </special-rule>
                {rating}
            }
        })
        .collect_view()
}
