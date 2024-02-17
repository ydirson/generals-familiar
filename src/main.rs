use gloo_net::http::Request;
use leptonic::prelude as ltn;
use leptonic::prelude::AlertContent;
use leptos::*;
use leptos_meta::{provide_meta_context, Title};
use leptos_router as ltr;
use leptos_router::Params; // derive(ltr::Params) won't work ?!
use std::rc::Rc;

const APP_NAME: &str = "General's Familiar";

// `println!(..)`-style syntax for debugging in browser console
#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

fn main() {
    #[cfg(feature = "dev")]
    console_error_panic_hook::set_once();

    provide_meta_context();
    mount_to_body(|| view! { <AppBoilerplate/> })
}

#[derive(Clone, Debug)]
struct Army {
    army_id: Signal<String>,
    unit_selection: RwSignal<Option<Rc<opr::Unit>>>,
    army_data: Resource<String, Result<Rc<opr::Army>, String>>,
}

impl Army {
    fn new(army_id: Signal<String>) -> Army
    {
        let unit_selection = create_rw_signal(None::<Rc<opr::Unit>>);
        let army_data = create_resource(
            move || army_id.get(),
            |army_id_value| {
                let url = opr::get_army_url(&army_id_value);
                async move { load_json_from_url::<Rc<opr::Army>>(&url).await }
            });
        Army{army_id, unit_selection, army_data}
    }
}

async fn load_json_from_url<T>(url: &str) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    let response = Request::get(url).send().await
        .map_err(|e| format!("communication error: {e}"));
    match response {
        Err(e) => Err(e),
        Ok(response) if ! response.ok() =>
            Err(format!("HTTP error {} - {}", response.status(), response.status_text())),
        Ok(response) =>
            response.json().await
            .map_err(|e| format!("parse error: {e}")),
    }
}

/// component dedicated to boilerplate not really part of the app per
/// se, and mandatory parents of the app
#[component]
fn AppBoilerplate() -> impl IntoView {
    view! {
        <ltn::Root default_theme=ltn::LeptonicTheme::default()>
            <Title formatter=|text| format!("{APP_NAME} — {text}")/>
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
    let game_system = create_rw_signal(None::<opr::GameSystem>);
    provide_context(game_system);

    let common_rules: Resource<Option<opr::GameSystem>,
                               Result<Vec<Rc<opr::SpecialRuleDef>>, String>> =
        create_resource(
            move || game_system.get(),
            |game_system| async move {
                match game_system {
                    Some(game_system) => {
                        let url = opr::get_common_rules_url(game_system);
                        load_json_from_url(&url).await
                    },
                    None => Err("no common-rules, game system not known yet".to_string()),
                }
            });
    provide_context(common_rules);

    let army_ids = move || {
        query.with(|params| {
            match params.as_ref().map(|params| params.armies.clone()) {
                Ok(None) => Ok(None),
                Err(err) => Err(err.to_string()),
                Ok(Some(armies_string)) => {
                    let v = armies_string
                        .split(',')
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
            <h1>
                {APP_NAME}
                {move || match game_system.get() {
                    Some(game_system) => format!(" - {game_system}"),
                    None => "".to_string(),
                }}
            </h1>
            <ltn::ThemeToggle off=ltn::LeptonicTheme::Light on=ltn::LeptonicTheme::Dark/>
        </ltn::AppBar>
        <ltn::Box style="padding: 0 1em 1em 1em;">
            <Show when=move || { army_ids().is_ok() }
                  fallback=move || view! {
                      <SelectView alert_type={ltn::AlertVariant::Warn}
                                  message=army_ids().err().unwrap() />
                  } >
                <Show when=move || { army_ids().unwrap().is_some() }
                      fallback=|| view! {
                          <SelectView alert_type={ltn::AlertVariant::Info}
                                      message="no army selected".to_string() />
                      } >
                    <ArmiesView army_id0 army_id1 />
                </Show>
            </Show>
        </ltn::Box>
    }
}

#[component]
fn SelectView(message: String, alert_type: ltn::AlertVariant) -> impl IntoView {
    // reset global game_system so a different can be enabled by new armies
    let app_game_system = expect_context::<RwSignal<Option<opr::GameSystem>>>();
    app_game_system.set(None);

    view! {
        <Title text="select armies"/>
        <ltn::Alert variant=alert_type>
            <AlertContent slot>{message}</AlertContent>
        </ltn::Alert>

        <SampleMatchups/>
    }
}

#[component]
fn SampleMatchups() -> impl IntoView {
    view! {
        <h3> "Sample matchups" </h3>
        <ltn::TableContainer>
            <ltn::Table bordered=true hoverable=true>
                <ltn::TableBody>
                    <ltn::TableRow>
                        <ltn::TableCell><a href="./?armies=Rrlct39EGuct,p2KIbSBOYpSB">
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
    let army0 = Army::new(army_id0);
    let army1 = Army::new(army_id1);
    view! {
        <Title text="view armies" />
        <ltn::Stack orientation=ltn::StackOrientation::Horizontal
               spacing=ltn::Size::Em(1.0)
               style="align-items: flex-start;">
            <ArmyContainer army=army0 side=ltn::DrawerSide::Left />
            <ArmyContainer army=army1 side=ltn::DrawerSide::Right />
        </ltn::Stack>
    }
}

#[derive(Clone)]
struct DrawerControl {
    shown: RwSignal<bool>,
}
impl Default for DrawerControl {
    fn default() -> Self {
        DrawerControl {
            shown: create_rw_signal(false),
        }
    }
}

/// A component container for the army list and the drawer, so they
/// can share a common context
#[component]
fn ArmyContainer(army: Army, side: ltn::DrawerSide) -> impl IntoView {
    // the `shown` status can be changed by eg. selecting in the army
    // list, or using close button in the drawer itself
    let drawer_control = DrawerControl::default();
    let shown = drawer_control.shown.clone();
    create_effect(move |_| {
        shown.set(army.unit_selection.with(Option::is_some));
    });

    view! {
        <Provider value=drawer_control >
            <DetailsDrawer side army=army.clone() />
            <ArmyList army />
        </Provider>
    }
}

#[component]
fn ArmyList(army: Army,
) -> impl IntoView {
    let app_game_system = expect_context::<RwSignal<Option<opr::GameSystem>>>();
    view! {
        <ltn::Stack spacing=ltn::Size::Em(0.5) class="army_list" >
        { move || {
            army.army_data.with(
                |army_data| match army_data {
                    None => view! { <h2>"Loading..."</h2> }.into_view(),
                    Some(Err(message)) => {
                        let message = message.clone();
                        view! {
                            <ltn::Alert variant=ltn::AlertVariant::Danger>
                                <AlertContent slot>{message}</AlertContent>
                            </ltn::Alert>
                        }.into_view()
                    },
                    Some(Ok(army_data)) => {
                        let opr::Army{ref game_system, ref name, ref units, ..} = **army_data;
                        let game_system = game_system.clone();
                        let name = Rc::clone(name);
                        let units = units.clone();

                        // since we are recreating an ArmyList, the Army must have changed
                        // (or this is an over-refresh bug), drawer is outdated so close it
                        let shown = use_context::<DrawerControl>().unwrap().shown;
                        shown.set(false);

                        let check_inconsistency = move || {
                            match game_system.clone() {
                                Err(e) => Some(e),
                                Ok(game_system) => match app_game_system.get() {
                                    // not yet set: set it, ok
                                    None => {
                                        app_game_system.set(Some(game_system));
                                        None },
                                    // already set and matching this army: ok
                                    Some(app_game_system) if game_system == app_game_system
                                        => None,
                                    // already set and not matching: KO
                                    Some(_)
                                        => Some(format!("game system mismatch: {game_system}")),
                                }
                            }
                        };
                        view! {
                            {move || {
                                let name = Rc::clone(&name);
                                let url = format!("{}?id={}",
                                                  opr::ARMYFORGE_SHARE_URL,
                                                  army.army_id.get());
                                view! {
                                    <h2><a target="_blank" href={url}>{name}</a></h2>
                                }
                            }}
                            {move || {
                                let check_inconsistency = check_inconsistency();
                                if check_inconsistency.is_none() {
                                    view! {
                                        <UnitsList units={units.clone()}
                                                   select_unit=army.unit_selection />
                                    }.into_view()
                                } else {
                                    view! {
                                        <ltn::Alert variant=ltn::AlertVariant::Danger>
                                            <AlertContent slot>
                                                {check_inconsistency.unwrap()}
                                            </AlertContent>
                                        </ltn::Alert>
                                    }.into_view()
                                }
                            }}
                        }.into_view()
                    },
                }
            )
        }}
        </ltn::Stack>
    }
}

#[component]
fn UnitsList(units: Vec<Rc<opr::Unit>>,
             select_unit: RwSignal<Option<Rc<opr::Unit>>>, // FIXME only need WriteSignal here
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
                                             matches!(selected_row_num.get(),
                                                      Some(index) if index == i)
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
fn DetailsDrawer(army: Army,
                 side: ltn::DrawerSide) -> impl IntoView {
    let pos_class = match side {
        ltn::DrawerSide::Left => "left",
        ltn::DrawerSide::Right => "right",
    };

    let shown = use_context::<DrawerControl>().unwrap().shown;
    view! {
        <ltn::Drawer side shown class={format!("army_details {pos_class}")}>
            <Show when=move || shown.get() >
                <UnitDetails army=army.clone() side
                             unit=army.unit_selection.get().unwrap() />
            </Show>
        </ltn::Drawer>
    }
}

#[component]
fn UnitDetails(unit: Rc<opr::Unit>,
               army: Army,
               side: ltn::DrawerSide,
) -> impl IntoView
{
    let shown = use_context::<DrawerControl>().unwrap().shown;
    let unit_name = unit.formatted_name();
    let opr::Unit{quality, defense, full_cost, ref loadout, ref special_rules, ..} = *unit;
    let close_button = |glyph| view! {
        <ltn::Button color=ltn::ButtonColor::Secondary
                     on_click=move |_| shown.set(false)> {glyph} </ltn::Button>
    };
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
                    {format!("{full_cost} pts")}
                    {right_button}
                </ltn::Stack>
            </ltn::Stack>
        </h3>
        <p><SpecialRulesList special_rules={special_rules.clone()} /></p>
        <p><UnitUpgradesList loadout_list={loadout.clone()} /></p>
        <EquipmentList loadout_list={loadout.clone()} />
        <SpecialRulesDefList unit=Rc::clone(&unit) army />
    }
}

#[component]
fn UnitUpgradesList(loadout_list: Vec<Rc<opr::UnitLoadout>>) -> impl IntoView {
    view! {
        {move || {
            loadout_list
                .clone()
                .into_iter()
                .filter(|loadout| matches!(**loadout, opr::UnitLoadout::Upgrade{..}))
                .enumerate()
                .map(|(i, loadout)| {
                    if let opr::UnitLoadout::Upgrade(ref upgrade) = *loadout {
                        let opr::UnitUpgrade{name, ref content, ..} = upgrade;
                        view! {
                            {move || if i > 0 { ", " } else { "" }}
                            {Rc::clone(name)} " (" <SpecialRulesList special_rules={content.clone()} /> ")"
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
                            .filter(|loadout| matches!(**loadout, opr::UnitLoadout::Equipment{..}))
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
        let name = Rc::clone(&equipment.name);
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
            let rating = match special_rule.rating {
                None => { "".to_string() },
                Some(rating) => { format!("({})", rating) },
            };
            view! {
                {separator}
                <special-rule>
                    {Rc::clone(&special_rule.name)}
                </special-rule>
                {rating}
            }
        })
        .collect_view()
}

#[component]
fn SpecialRulesDefList(unit: Rc<opr::Unit>,
                       army: Army,) -> impl IntoView {
    view! {
        <specialrules-def-list>
            {move || {
                let unit = Rc::clone(&unit);
                army.army_data.with(
                    move |army_data| {
                        if let Some(Ok(army_data)) = army_data {
                            // the rules def
                            let opr::Army{ref special_rules, ..} = **army_data;
                            let special_rules_def = special_rules;

                            view!{
                                {match common_rules_def() {
                                    Ok(common_rules_def) => view! {
                                        {rules_descriptions_from_list_for_unit(
                                            Rc::clone(&unit), &common_rules_def.clone())}
                                    }.into_view(),
                                    Err(message) => view! {
                                        <ltn::Alert variant=ltn::AlertVariant::Danger>
                                            <AlertContent slot>{message}</AlertContent>
                                        </ltn::Alert>
                                    }.into_view(),
                                }}
                                {rules_descriptions_from_list_for_unit(
                                    Rc::clone(&unit), special_rules_def)}
                            }.into_view()
                        } else {
                            // cannot happen - FIXME should pass opr::Army directly instead?
                            view!{}.into_view()
                        }
                    }
                )
            }}
        </specialrules-def-list>
    }
}

/// extract common-rules definitions from the Context Resource, or an
/// error string to display
fn common_rules_def() -> Result<Vec<Rc<opr::SpecialRuleDef>>, String> {
    let common_rules_def = use_context::<
            Resource<Option<opr::GameSystem>, Result<Vec<Rc<opr::SpecialRuleDef>>, String>>
            >();

    if let Some(common_rules_def) = common_rules_def {
        let common_rules_def = common_rules_def.get();
        match common_rules_def {
            Some(Ok(common_rules_def)) => Ok(common_rules_def),
            Some(Err(message)) => Err(format!("common rules not found: {message}")),
            // FIXME that one is not really an error
            None => Err("(common rules still loading)".to_string()),
        }
    } else {
        Err("(internal error, common rules resource not found in context)".to_string())
    }
}

fn rules_descriptions_from_list_for_unit(unit: Rc<opr::Unit>,
                                         rules_def: &[Rc<opr::SpecialRuleDef>]
) -> impl IntoView {
    let opr::Unit{ref special_rules, ref loadout, ..} = *unit;
    rules_def
        .iter()
        .map(|rule_def| {
            if special_rules
                .iter()
                .any(|rule| rule.name == rule_def.name) ||
                loadout
                .iter()
                .any(|item| match &**item {
                    opr::UnitLoadout::Equipment(equipment) =>
                        equipment.special_rules.iter().any(|rule| rule.name == rule_def.name),
                    opr::UnitLoadout::Upgrade(upgrade) =>
                        upgrade.content.iter().any(|rule| rule.name == rule_def.name),
                })
            {
                let opr::SpecialRuleDef{ref name, ref description} = **rule_def;
                view!{
                    <p>
                        <rule-name>{Rc::clone(name)}</rule-name> ": "
                        {Rc::clone(description)}
                    </p>
                }.into_view()
            } else {view!{}.into_view()}})
        .collect_view()
}
