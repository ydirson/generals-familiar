use gloo_net::http::Request;
use itertools::Itertools; // sorted_by, join
use leptos::prelude::*;
use leptos::either::{Either, EitherOf3};
use leptos_meta::{provide_meta_context, Title};
use leptos_router as ltr;
use leptos_router::components as ltrc;
use leptos_router::params::Params; // derive(ltr::Params) won't work ?!
use std::iter::once;
use std::sync::Arc;

const APP_NAME: &str = "General's Familiar";

// `println!(..)`-style syntax for debugging in browser console
#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

fn open_in_new_tab_and(url: &str, action: impl FnOnce() -> ()) {
    window().open_with_url_and_target(url, "_blank")
        .expect("should open a window");
    action();
}

fn main() {
    #[cfg(feature = "dev")]
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <AppBoilerplate/> })
}

#[derive(Clone)]
struct Army {
    army_id: Signal<String>,
    unit_selection: RwSignal<Option<Arc<opr::UnitGroup>>>,
    army_data: AsyncDerived<Result<Arc<opr::Army>, String>, LocalStorage>,
}

impl Army {
    fn new(army_id: Signal<String>) -> Army
    {
        let unit_selection = RwSignal::new(None::<Arc<opr::UnitGroup>>);
        let army_data = AsyncDerived::new_unsync(
            move || {
                let army_id_value = army_id.get();
                let url = opr::get_army_url(&army_id_value);
                async move { load_json_from_url::<Arc<opr::Army>>(&url).await }
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
    provide_meta_context();
    let theme = RwSignal::new(thaw::Theme::light());
    let theme_class = Memo::new(move |_| {
        theme.with(|theme| format!("color-scheme--{}", theme.color.color_scheme()))
    });
    // get app URL to workaround Router assuming by default path
    // components are a route
    let path: &'static str = document()
        .location().expect("document should have a location")
        .pathname().expect("document location should have a pathname")
        .leak();
    log!("location path: {path:?}");
    view! {
        <Title formatter=|text| format!("{APP_NAME} — {text}")/>
        <thaw::ConfigProvider theme class=theme_class>
// <ltn::Box style="min-height: 100vh;">
            <ltrc::Router>
                <App/>
            </ltrc::Router>
// </ltn::Box>
        </thaw::ConfigProvider>
    }
}

#[derive(Params,PartialEq)]
struct UrlQuery {
    armies: Option<String>,
}

#[component]
fn ThemeToggle() -> impl IntoView {
    let theme = thaw::Theme::use_rw_theme();
    let checked = RwSignal::new(false);
    Effect::new(move |_| {
        theme.set(if checked.get() {thaw::Theme::dark()} else {thaw::Theme::light()});
    });

    view! {
        <thaw::Switch checked/>
    }
}

/// the main application component
#[component]
fn App() -> impl IntoView {
    let query = ltr::hooks::use_query::<UrlQuery>();
    let game_system = RwSignal::new(None::<opr::GameSystem>);
    provide_context(game_system);

    let common_rules = AsyncDerived::new_unsync(
        move || async move {
            match game_system.get() {
                Some(game_system) => {
                    let url = opr::get_common_rules_url(game_system);
                    load_json_from_url::<Arc<opr::CommonRules>>(&url).await
                },
                None => Err("no common-rules, game system not known yet".to_string()),
            }
        });
    provide_context(common_rules);

    let army_ids = Signal::derive(move || {
        query.with(|params| {
            match params.as_ref().map(|params| params.armies.clone()) {
                Ok(None) => Ok(vec!()),
                Err(err) => Err(err.to_string()),
                Ok(Some(armies_string)) => {
                    let v = armies_string
                        .split(',')
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>();
                    Ok(v)
                },
            }
        })
    });
    view! {
        <thaw::Layout class="app"
                      content_style="min-height: 100vh; display: flex; flex-direction: column">
            <thaw::LayoutHeader class="app-bar">
                <thaw::Flex justify=thaw::FlexJustify::SpaceBetween align=thaw::FlexAlign::Center>
                    <h1>
                        {APP_NAME}
                        {move || match game_system.get() {
                            Some(game_system) => format!(" - {game_system}"),
                            None => "".to_string(),
                        }}
                    </h1>
                    <ThemeToggle/>
                </thaw::Flex>
            </thaw::LayoutHeader>
            <thaw::Flex class="app-contents" justify=thaw::FlexJustify::Center
                        style="flex: 1">
                <Show when=move || { army_ids.with(Result::is_ok) }
                      fallback=move || view! {
                          <SelectView alert_type={thaw::MessageBarIntent::Warning}
                                      message=army_ids.get().err().unwrap() />
                      } >
                    <Show when=move || { ! army_ids.get().unwrap().is_empty() }
                          fallback=|| view! {
                              <SelectView alert_type={thaw::MessageBarIntent::Info}
                                          message="no army selected".to_string() />
                          } >
                        <ArmiesView army_ids=Signal::derive(move || army_ids.get().unwrap().clone()) />
                    </Show>
                </Show>
            </thaw::Flex>
        </thaw::Layout>
    }
}

#[component]
fn SelectView(message: String, alert_type: thaw::MessageBarIntent) -> impl IntoView {
    // reset global game_system so a different can be enabled by new armies
    let app_game_system = expect_context::<RwSignal<Option<opr::GameSystem>>>();
    app_game_system.set(None);

    view! {
        <Title text="select armies"/>
        <thaw::Flex vertical=true class="sample-matchups">
            <thaw::MessageBar intent=alert_type>
                <thaw::MessageBarBody>
                    {message}
                </thaw::MessageBarBody>
            </thaw::MessageBar>

            <SampleMatchups/>
        </thaw::Flex>
    }
}

#[component]
fn SampleMatchups() -> impl IntoView {
    view! {
        <h3> "Notice" </h3>
        <p>
            "This app is a technical preview. "
            "Remember to use smartphones in landscape mode."
        </p>
        <h3> "Sample matchups" </h3>
        <table-wrapper>
            <table> // FIXME: bordered=true hoverable=true>
                <tbody>
                    <tr>
                        <td>
                            <em>"Grimdark Future — "</em>
                            <a href="./?armies=Rrlct39EGuct,p2KIbSBOYpSB">
                            "Robots Legion vs. Prime Brothers" </a>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <em>"Grimdark Future — "</em>
                            <a href="./?armies=Mlwpoh1AGLC2">
                            "Robots Legion (single list)" </a>
                        </td>
                    </tr>
                    <tr>
                        <td>
                            <em>"Age of Fantasy: Skirmish — "</em>
                            <a href="./?armies=zhz5uajqHdt5,ZTgIvcYABynP">
                            "War Disciples vs. Eternal Wardens" </a>
                        </td>
                    </tr>
                </tbody>
            </table>
        </table-wrapper>
    }
}

/// the main view, showing multiple armies and providing detail
/// drawers for selections
#[component]
fn ArmiesView(army_ids: Signal<Vec<String>>) -> impl IntoView {
    let dialog_shown = RwSignal::new(false);
    provide_context(army_ids);
    view! {
        <Title text="view armies" />
        <thaw::Flex class="army_view" >
            <For each=move || army_ids.with(|ids| ids.iter()
                                            .map(String::clone)
                                            .enumerate().collect::<Vec<(usize, String)>>())
                 key=|k: &(usize, String)| k.clone()
                 children=move |(i, id)| view! {
                     <ArmyContainer army={Army::new(Signal::derive(move || id.clone()))}
                                    side={if i == 0 {thaw::DrawerPosition::Left}
                                          else {thaw::DrawerPosition::Right}} />
                 }
             />
            <thaw::Button appearance=thaw::ButtonAppearance::Primary
                          class="new_army"
                          on_click=move |_| dialog_shown.set(true) >
                "+"
            </thaw::Button>
        </thaw::Flex>
        <NewArmyDialog show_when=dialog_shown />
    }
}

#[component]
fn NewArmyDialog(#[prop(into)] show_when: RwSignal<bool>) -> impl IntoView {
    let new_id = RwSignal::new("".to_string());
    view! {
        <thaw::Dialog open=show_when ><thaw::DialogSurface><thaw::DialogBody>
            <thaw::DialogTitle>"Load Army"</thaw::DialogTitle>
            <thaw::DialogContent>
                <thaw::Input placeholder="AF Share ID"
                             // should_be_focused=|| true
                             value=new_id
                 />
            </thaw::DialogContent>
            <thaw::DialogActions>
                <thaw::Button on_click=move |_| show_when.set(false)>
                    "Cancel"
                </thaw::Button>
                {move || {
                    let ids =
                        use_context::<Signal<Vec<String>>>()
                        .expect("should find army_ids in context")
                        .get();
                    let armies = ids.into_iter()
                        .chain(once(new_id.get()))
                        .join(",");
                    let url = format!("./?armies={armies}");
                    let navigate = ltr::hooks::use_navigate();
                    view! {
                        <thaw::Button appearance=thaw::ButtonAppearance::Primary
                                      on_click=move |_| {
                                          show_when.set(false);
                                          navigate(url.as_str(), Default::default());
                                          new_id.set("".to_string());
                                      } >
                            "Load"
                        </thaw::Button>
                    }
                }}
            </thaw::DialogActions>
        </thaw::DialogBody></thaw::DialogSurface></thaw::Dialog>
    }
}

#[derive(Clone)]
struct DrawerControl {
    shown: RwSignal<bool>,
}
impl Default for DrawerControl {
    fn default() -> Self {
        DrawerControl {
            shown: RwSignal::new(false),
        }
    }
}

/// A component container for the army list and the drawer, so they
/// can share a common context
#[component]
fn ArmyContainer(army: Army, side: thaw::DrawerPosition) -> impl IntoView {
    // the `shown` status can be changed by eg. selecting in the army
    // list, or using close button in the drawer itself
    let drawer_control = DrawerControl::default();
    let shown = drawer_control.shown.clone();
    Effect::new(move |_| {
        shown.set(army.unit_selection.with(Option::is_some));
    });

    view! {
        <leptos::context::Provider value=drawer_control >
            <DetailsDrawer side army=army.clone() />
            <ArmyList army />
        </leptos::context::Provider>
    }
}

#[component]
fn ArmyList(army: Army,
) -> impl IntoView {
    let app_game_system = expect_context::<RwSignal<Option<opr::GameSystem>>>();
    view! {
        <thaw::Flex class="army_list" vertical={true} >
        { move || {
            army.army_data.with(
                |army_data| match army_data {
                    None => EitherOf3::A(view! { <h2>"Loading..."</h2> }),
                    Some(Err(message)) => {
                        let message = message.clone();
                        let army_id = army.army_id.get();
                        EitherOf3::B(view! {
                            <thaw::Flex gap=thaw::FlexGap::Small >
                                <thaw::MessageBar intent=thaw::MessageBarIntent::Error>
                                    <thaw::MessageBarBody>
                                        {message}
                                    </thaw::MessageBarBody>
                                </thaw::MessageBar>
                                // FIXME: hack to have the button on errors have the same
                                // size as the one on titles
                                <span style="font-size: var(--typography-h2-font-size);">
                                    <RemoveArmyButton army_id />
                                </span>
                            </thaw::Flex>
                        })
                    },
                    Some(Ok(army_data)) => {
                        let opr::Army{ref game_system, ref name, ref unit_groups, ..} = **army_data;
                        let army_data = Arc::clone(army_data);
                        let game_system = game_system.clone();
                        let name = Arc::clone(name);
                        let unit_groups = unit_groups.clone();

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
                        EitherOf3::C(view! {
                            {move || {
                                let army_id = army.army_id.get();
                                let dialog_shown = RwSignal::new(false);
                                view! {
                                    <h2>
                                        <span on:click=move |_| dialog_shown.set(true)>
                                            {name.clone()}</span>
                                        <RemoveArmyButton army_id />
                                    </h2>
                                    <ArmyDialog army={Arc::clone(&army_data)}
                                                show_when=dialog_shown />
                                }
                            }}
                            {move || {
                                let check_inconsistency = check_inconsistency();
                                if check_inconsistency.is_none() {
                                    Either::Left(view! {
                                        <UnitsList unit_groups={unit_groups.clone()}
                                                   select_unit=army.unit_selection />
                                    })
                                } else {
                                    Either::Right(view! {
                                        <thaw::MessageBar intent=thaw::MessageBarIntent::Error>
                                            <thaw::MessageBarBody>
                                                {check_inconsistency.unwrap()}
                                            </thaw::MessageBarBody>
                                        </thaw::MessageBar>
                                    })
                                }
                            }}
                        })
                    },
                }
            )
        }}
        </thaw::Flex>
    }
}

#[component]
fn ArmyDialog(
    army: Arc<opr::Army>,
    #[prop(into)] show_when: RwSignal<bool>,
) -> impl IntoView {
    provide_context(army);

    view! {
        <thaw::Dialog class="army_details" open=show_when close_on_esc={true} >
            <thaw::DialogSurface><thaw::DialogBody>
            {
                let army = use_context::<Arc<opr::Army>>()
                    .expect("should find my Army");
                let af_url = format!("{}?id={}", opr::ARMYFORGE_SHARE_URL, army.id);
                view! {
                    <thaw::DialogTitle>{Arc::clone(&army.name)}</thaw::DialogTitle>
                    <thaw::DialogContent>
                        <thaw::Button
                            appearance=thaw::ButtonAppearance::Primary
                            on_click=move |_| {
                                open_in_new_tab_and(&af_url, || show_when.set(false))
                            }
                        >
                            "Open in ArmyForge"
                        </thaw::Button>
                    </thaw::DialogContent>
                    <thaw::DialogActions>
                        <thaw::Button on_click=move |_| show_when.set(false)>
                            "Close"
                        </thaw::Button>
                    </thaw::DialogActions>
                }
            }
            </thaw::DialogBody></thaw::DialogSurface>
        </thaw::Dialog>
    }
}

#[component]
fn UnitsList(unit_groups: Vec<Arc<opr::UnitGroup>>,
             select_unit: RwSignal<Option<Arc<opr::UnitGroup>>>, // FIXME only need WriteSignal here
) -> impl IntoView {
    let (selected_row_num, set_selected_row_num) = signal(None::<usize>);
    view! {
        <table-wrapper>
            <table> // FIXME bordered=true hoverable=true>
                <tbody>
                    {move || {
                        unit_groups
                            .clone()
                            .into_iter()
                            .sorted_by(|g1, g2| g1.display_cmp(&g2))
                            .enumerate()
                            .map(|(i, group)| {
                                let group_name = (*group).formatted_name();
                                //let opr::Unit{..} = *unit;
                                view! {
                                    <tr
                                         class:selected=move || {
                                             matches!(selected_row_num.get(),
                                                      Some(index) if index == i)
                                         }
                                         on:click=move |_| {
                                             select_unit.set(Some(group.clone()));
                                             set_selected_row_num.set(Some(i));
                                    }>
                                        <td> {group_name} </td>
                                    </tr>
                                }
                            })
                            .collect_view()
                    }}
                </tbody>
            </table>
        </table-wrapper>
    }
}

#[component]
fn RemoveArmyButton(army_id: String) -> impl IntoView {
    let mut ids =
        use_context::<Signal<Vec<String>>>()
        .expect("should find army_ids in context")
        .get();
    ids.remove(ids.iter().position(|x| *x == army_id)
               .expect("id should be in the list to remove it"));
    let remove_army_url = if ids.is_empty() {
        "./".to_string()
    } else {
        format!("./?armies={}", ids.join(","))
    };

    view! {
        <a href=remove_army_url >
            <button class="rm_army">
                <thaw::Icon icon=icondata::IoCloseCircleOutline />
            </button>
        </a>
    }
}

#[component]
fn DetailsDrawer(army: Army,
                 side: thaw::DrawerPosition) -> impl IntoView {
    let pos_class = match side {
        thaw::DrawerPosition::Left => "left",
        thaw::DrawerPosition::Right => "right",
        _ => unreachable!(),
    };

    let shown = use_context::<DrawerControl>().unwrap().shown;
    view! {
        <thaw::OverlayDrawer position=side open=shown modal_type=thaw::DrawerModalType::NonModal
                             class={format!("army_details {pos_class} color-scheme--light")}>
            // FIXME use DrawerHeader?
            <thaw::DrawerBody>
                <Show when=move || shown.get() >
                    <GroupDetails army=army.clone() side
                                  group=army.unit_selection.get().unwrap() />
                </Show>
            </thaw::DrawerBody>
        </thaw::OverlayDrawer>
    }
}

#[component]
fn GroupDetails(group: Arc<opr::UnitGroup>,
                army: Army,
                side: thaw::DrawerPosition,
) -> impl IntoView
{
    let opr::UnitGroup{full_cost, ..} = *group;
    let shown = use_context::<DrawerControl>().unwrap().shown;
    let group_name = group.formatted_name();
    let close_button = |glyph| view! {
        <thaw::Button on_click=move |_| shown.set(false)> {glyph} </thaw::Button>
    };
    let (left_button, right_button) = match side {
        thaw::DrawerPosition::Left  => ( Some(close_button("<")), None ),
        thaw::DrawerPosition::Right => ( None, Some(close_button(">")) ),
        _ => unreachable!(),
    };

    view! {
        <h3>
            <thaw::Space justify=thaw::SpaceJustify::SpaceBetween >
                <thaw::Space>
                    {left_button}
                    {group_name}
                </thaw::Space>
                <thaw::Space>
                    {format!("{full_cost} pts")}
                    {right_button}
                </thaw::Space>
            </thaw::Space>
        </h3>

        {
            let single = group.units.len() == 1;
            group.units.iter()
                .map(|unit| view! {<UnitDetails unit=Arc::clone(unit) single />})
                .collect_view()
        }

        <SpecialRulesDefList group=Arc::clone(&group) army />
    }
}

#[component]
fn UnitDetails(unit: Arc<opr::Unit>,
               single: bool,
) -> impl IntoView
{
    let unit_name = unit.formatted_name();
    let opr::Unit{quality, defense, full_cost, ref loadout, ref special_rules, ..} = *unit;
    let special_rules = special_rules.clone();

    view! {
        <p>
            <thaw::Space justify=thaw::SpaceJustify::SpaceBetween>
                <span>
                    <span class="unit">
                        {(!single).then(|| format!("{unit_name}: "))}
                        {format!("Q{quality} D{defense}")}
                    </span>
                    " "
                    <SpecialRulesList special_rules={special_rules.clone()} />
                </span>
                {format!("{full_cost} pts")}
            </thaw::Space>
        </p>
        <p><UnitUpgradesList loadout_list={loadout.clone()} /></p>
        <EquipmentList loadout_list={loadout.clone()} />
    }
}

#[component]
fn UnitUpgradesList(loadout_list: Vec<Arc<opr::UnitLoadout>>) -> impl IntoView {
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
                            {Arc::clone(name)}
                            " (" <SpecialRulesList special_rules={content.clone()} /> ")"
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
fn EquipmentList(loadout_list: Vec<Arc<opr::UnitLoadout>>) -> impl IntoView {
    view! {
        <table-wrapper>
            <table> // FIXME bordered=true hoverable=true>
                <tbody>
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
                </tbody>
            </table>
        </table-wrapper>
    }
}

#[component]
fn EquipmentItem(loadout: Arc<opr::UnitLoadout>) -> impl IntoView {
    if let opr::UnitLoadout::Equipment(ref equipment) = *loadout {
        let name = Arc::clone(&equipment.name);
        let special_rules = equipment.special_rules.clone();
        let opr::Equipment{count, range, attacks, ..} = *equipment;
        view! {
            <tr>
                <td>
                    {if count != 1
                        {format!("{}x ", count)} else {"".to_string()}}
                    {name}
                </td>
                <td>
                    {if range != 0
                        {format!(r#"{}""#, range )}
                        else {"-".to_string()}}
                </td>
                <td>
                    {format!("A{}", attacks)}
                </td>
                <td>
                    <SpecialRulesList special_rules={special_rules.clone()} />
                </td>
            </tr>
        }
    } else {
        panic!("EquipmentItem must be used on Equipment only");
    }
}

#[component]
fn SpecialRulesList(special_rules: Vec<Arc<opr::SpecialRule>>) -> impl IntoView {
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
                    {Arc::clone(&special_rule.name)}
                </special-rule>
                {rating}
            }
        })
        .collect_view()
}

#[component]
fn SpecialRulesDefList(group: Arc<opr::UnitGroup>,
                       army: Army,) -> impl IntoView {
    view! {
        <specialrules-def-list>
            {move || {
                let group = Arc::clone(&group);
                army.army_data.with(
                    move |army_data| {
                        if let Some(Ok(army_data)) = army_data {
                            // the rules def
                            let opr::Army{ref special_rules, ..} = **army_data;
                            let special_rules_def = special_rules;

                            Either::Left(view!{
                                {match common_rules_def() {
                                    Ok(common_rules_def) => Either::Left(view! {
                                        {rules_descriptions_from_list_for_group(
                                            Arc::clone(&group), &common_rules_def.clone(),
                                            RuleType::Common)}
                                    }),
                                    Err(message) => Either::Right(view! {
                                        <thaw::MessageBar intent=thaw::MessageBarIntent::Error>
                                            <thaw::MessageBarBody>
                                                {message}
                                            </thaw::MessageBarBody>
                                        </thaw::MessageBar>
                                    }),
                                }}
                                {rules_descriptions_from_list_for_group(
                                    Arc::clone(&group), special_rules_def, RuleType::Army)}
                            })
                        } else {
                            // cannot happen - FIXME should pass opr::Army directly instead?
                            Either::Right(view!{})
                        }
                    }
                )
            }}
        </specialrules-def-list>
    }
}

/// extract common-rules definitions from the Context Resource, or an
/// error string to display
fn common_rules_def() -> Result<Vec<Arc<opr::SpecialRuleDef>>, String> {
    let common_rules_def = use_context::<
            AsyncDerived<Result<Arc<opr::CommonRules>, String>, LocalStorage>
            >();

    if let Some(common_rules_def) = common_rules_def {
        let common_rules_def = common_rules_def.get();
        match common_rules_def {
            Some(Ok(common_rules_def)) => Ok(common_rules_def.rules.clone()),
            Some(Err(message)) => Err(format!("common rules not found: {message}")),
            // FIXME that one is not really an error
            None => Err("(common rules still loading)".to_string()),
        }
    } else {
        Err("(internal error, common rules resource not found in context)".to_string())
    }
}

enum RuleType {
    Common,
    Army,
}

fn rules_descriptions_from_list_for_group(group: Arc<opr::UnitGroup>,
                                          rules_def: &[Arc<opr::SpecialRuleDef>],
                                          rule_type: RuleType,
) -> impl IntoView {
    rules_def
        .iter()
        .map(|rule_def| {
            if group_uses_rule(Arc::clone(&group), rule_def) {
                let opr::SpecialRuleDef{ref name, ref description} = **rule_def;
                Either::Left(view!{
                    <p class={match &rule_type {RuleType::Common => "common",
                                                RuleType::Army => "army"}} >
                        <rule-name>{Arc::clone(name)}</rule-name> ": "
                        {Arc::clone(description)}
                    </p>
                })
            } else {Either::Right(view!{})}})
        .collect_view()
}

fn group_uses_rule(group: Arc<opr::UnitGroup>,
                   rule_def: &Arc<opr::SpecialRuleDef>) -> bool {
    for unit in group.units.iter() {
        let opr::Unit{ref special_rules, ref loadout, ..} = **unit;
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
        { return true; }
    }
    return false;
}
