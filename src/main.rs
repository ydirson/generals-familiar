use gloo_net::http::Request;
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
        web_sys::console::log_1(&format!( $( $t )* ).into())
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
    unit_selection: RwSignal<Option<Rc<opr::UnitGroup>>>,
    army_data: Resource<String, Result<Rc<opr::Army>, String>>,
}

impl Army {
    fn new(army_id: Signal<String>) -> Army
    {
        let unit_selection = create_rw_signal(None::<Rc<opr::UnitGroup>>);
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
        <Title formatter=|text| format!("{APP_NAME} — {text}")/>
// <ltn::Box style="min-height: 100vh;">
        <ltr::Router fallback=|| view! {
            <thaw::Alert title="Bad URL" variant=thaw::AlertVariant::Error>
                "route not matched."
            </thaw::Alert>
        }.into_view() >
            <App/>
        </ltr::Router>
// </ltn::Box>
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
                               Result<Rc<opr::CommonRules>, String>> =
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
        <thaw::Layout class="app color-scheme--light">
            <thaw::LayoutHeader class="app-bar">
                <thaw::Flex justify=thaw::FlexJustify::SpaceBetween align=thaw::FlexAlign::Center>
                    <h1>
                        {APP_NAME}
                        {move || match game_system.get() {
                            Some(game_system) => format!(" - {game_system}"),
                            None => "".to_string(),
                        }}
                    </h1>
                </thaw::Flex>
            </thaw::LayoutHeader>
            <thaw::Space class="app-contents" justify=thaw::SpaceJustify::Center>
                <Show when=move || { army_ids.with(Result::is_ok) }
                      fallback=move || view! {
                          <SelectView alert_type={thaw::AlertVariant::Warning}
                                      message=army_ids.get().err().unwrap() />
                      } >
                    <Show when=move || { ! army_ids.get().unwrap().is_empty() }
                          fallback=|| view! {
                              <SelectView alert_type={thaw::AlertVariant::Warning} // FIXME: Info
                                          message="no army selected".to_string() />
                          } >
                        <ArmiesView army_ids=Signal::derive(move || army_ids.get().unwrap().clone()) />
                    </Show>
                </Show>
            </thaw::Space>
        </thaw::Layout>
    }
}

#[component]
fn SelectView(message: String, alert_type: thaw::AlertVariant) -> impl IntoView {
    // reset global game_system so a different can be enabled by new armies
    let app_game_system = expect_context::<RwSignal<Option<opr::GameSystem>>>();
    app_game_system.set(None);

    view! {
        <Title text="select armies"/>
        <thaw::Alert variant=alert_type>
            {message}
        </thaw::Alert>

        <SampleMatchups/>
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
            <thaw::Table> // FIXME: bordered=true hoverable=true>
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
            </thaw::Table>
        </table-wrapper>
    }
}

/// the main view, showing multiple armies and providing detail
/// drawers for selections
#[component]
fn ArmiesView(army_ids: Signal<Vec<String>>) -> impl IntoView {
    provide_context(army_ids);
    view! {
        <Title text="view armies" />
        <thaw::Flex class="army_view" >
            <For each=move || army_ids.with(|ids| ids.iter()
                                            .map(String::clone)
                                            .enumerate().collect::<Vec<(usize, String)>>())
                 key=|k: &(usize, String)| k.clone()
                 children=move |(i, id)| view! {
                     "Army" //<ArmyContainer army={Army::new(Signal::derive(move || id.clone()))}
                            //        side={if i == 0 {ltn::DrawerSide::Left}
                            //              else {ltn::DrawerSide::Right}} />
                 }
             />
        </thaw::Flex>
    }
}

// #[derive(Clone)]
// struct DrawerControl {
//     shown: RwSignal<bool>,
// }
// impl Default for DrawerControl {
//     fn default() -> Self {
//         DrawerControl {
//             shown: create_rw_signal(false),
//         }
//     }
// }

// /// A component container for the army list and the drawer, so they
// /// can share a common context
// #[component]
// fn ArmyContainer(army: Army, side: ltn::DrawerSide) -> impl IntoView {
//     // the `shown` status can be changed by eg. selecting in the army
//     // list, or using close button in the drawer itself
//     let drawer_control = DrawerControl::default();
//     let shown = drawer_control.shown.clone();
//     create_effect(move |_| {
//         shown.set(army.unit_selection.with(Option::is_some));
//     });

//     view! {
//         <Provider value=drawer_control >
//             <DetailsDrawer side army=army.clone() />
//             <ArmyList army />
//         </Provider>
//     }
// }

// #[component]
// fn ArmyList(army: Army,
// ) -> impl IntoView {
//     let app_game_system = expect_context::<RwSignal<Option<opr::GameSystem>>>();
//     view! {
//         <ltn::Stack spacing=ltn::Size::Em(0.5) class="army_list" >
//         { move || {
//             army.army_data.with(
//                 |army_data| match army_data {
//                     None => view! { <h2>"Loading..."</h2> }.into_view(),
//                     Some(Err(message)) => {
//                         let message = message.clone();
//                         let army_id = army.army_id.get();
//                         view! {
//                             <ltn::Stack spacing=ltn::Size::Em(0.5)
//                                         orientation=ltn::StackOrientation::Horizontal >
//                                 <ltn::Alert variant=ltn::AlertVariant::Danger>
//                                     <AlertContent slot>{message}</AlertContent>
//                                 </ltn::Alert>
//                                 // FIXME: hack to have the button on errors have the same
//                                 // size as the one on titles
//                                 <span style="font-size: var(--typography-h2-font-size);">
//                                     <RemoveArmyButton army_id />
//                                 </span>
//                             </ltn::Stack>
//                         }.into_view()
//                     },
//                     Some(Ok(army_data)) => {
//                         let opr::Army{ref game_system, ref name, ref unit_groups, ..} = **army_data;
//                         let game_system = game_system.clone();
//                         let name = Rc::clone(name);
//                         let unit_groups = unit_groups.clone();

//                         // since we are recreating an ArmyList, the Army must have changed
//                         // (or this is an over-refresh bug), drawer is outdated so close it
//                         let shown = use_context::<DrawerControl>().unwrap().shown;
//                         shown.set(false);

//                         let check_inconsistency = move || {
//                             match game_system.clone() {
//                                 Err(e) => Some(e),
//                                 Ok(game_system) => match app_game_system.get() {
//                                     // not yet set: set it, ok
//                                     None => {
//                                         app_game_system.set(Some(game_system));
//                                         None },
//                                     // already set and matching this army: ok
//                                     Some(app_game_system) if game_system == app_game_system
//                                         => None,
//                                     // already set and not matching: KO
//                                     Some(_)
//                                         => Some(format!("game system mismatch: {game_system}")),
//                                 }
//                             }
//                         };
//                         view! {
//                             {move || {
//                                 let name = Rc::clone(&name);
//                                 let army_id = army.army_id.get();
//                                 let af_url = format!("{}?id={army_id}", opr::ARMYFORGE_SHARE_URL);
//                                 view! {
//                                     <h2>
//                                         <a target="_blank" href={af_url}>{name}</a>
//                                         <RemoveArmyButton army_id />
//                                     </h2>
//                                 }
//                             }}
//                             {move || {
//                                 let check_inconsistency = check_inconsistency();
//                                 if check_inconsistency.is_none() {
//                                     view! {
//                                         <UnitsList unit_groups={unit_groups.clone()}
//                                                    select_unit=army.unit_selection />
//                                     }.into_view()
//                                 } else {
//                                     view! {
//                                         <ltn::Alert variant=ltn::AlertVariant::Danger>
//                                             <AlertContent slot>
//                                                 {check_inconsistency.unwrap()}
//                                             </AlertContent>
//                                         </ltn::Alert>
//                                     }.into_view()
//                                 }
//                             }}
//                         }.into_view()
//                     },
//                 }
//             )
//         }}
//         </ltn::Stack>
//     }
// }

// #[component]
// fn UnitsList(unit_groups: Vec<Rc<opr::UnitGroup>>,
//              select_unit: RwSignal<Option<Rc<opr::UnitGroup>>>, // FIXME only need WriteSignal here
// ) -> impl IntoView {
//     let (selected_row_num, set_selected_row_num) = create_signal(None::<usize>);
//     view! {
//         <ltn::TableContainer>
//             <ltn::Table bordered=true hoverable=true>
//                 <ltn::TableBody>
//                     {move || {
//                         unit_groups
//                             .clone()
//                             .into_iter()
//                             .enumerate()
//                             .map(|(i, group)| {
//                                 let group_name = (*group).formatted_name();
//                                 //let opr::Unit{..} = *unit;
//                                 view! {
//                                     // FIXME this ought to be a ltn::TableRow, which does
//                                     // not allow for dynamic classes or even for class
//                                     <leptonic-table-row
//                                          class:selected=move || {
//                                              matches!(selected_row_num.get(),
//                                                       Some(index) if index == i)
//                                          }
//                                          on:click=move |_| {
//                                              select_unit.set(Some(group.clone()));
//                                              set_selected_row_num.set(Some(i));
//                                     }>
//                                         <ltn::TableCell> {group_name} </ltn::TableCell>
//                                     </leptonic-table-row>
//                                 }
//                             })
//                             .collect_view()
//                     }}
//                 </ltn::TableBody>
//             </ltn::Table>
//         </ltn::TableContainer>
//     }
// }

// #[component]
// fn RemoveArmyButton(army_id: String) -> impl IntoView {
//     let mut ids =
//         use_context::<Signal<Vec<String>>>()
//         .expect("should find army_ids in context")
//         .get();
//     ids.remove(ids.iter().position(|x| *x == army_id)
//                .expect("id should be in the list to remove it"));
//     let remove_army_url = if ids.is_empty() {
//         "./".to_string()
//     } else {
//         format!("./?armies={}", ids.join(","))
//     };

//     view! {
//         <a href=remove_army_url >
//             <button class="rm_army">
//                 <ltn::Icon icon=ltn::icondata::IoCloseCircleOutline />
//             </button>
//         </a>
//     }
// }

// #[component]
// fn DetailsDrawer(army: Army,
//                  side: ltn::DrawerSide) -> impl IntoView {
//     let pos_class = match side {
//         ltn::DrawerSide::Left => "left",
//         ltn::DrawerSide::Right => "right",
//     };

//     let shown = use_context::<DrawerControl>().unwrap().shown;
//     view! {
//         <ltn::Drawer side shown class={format!("army_details {pos_class}")}>
//             <Show when=move || shown.get() >
//                 <GroupDetails army=army.clone() side
//                               group=army.unit_selection.get().unwrap() />
//             </Show>
//         </ltn::Drawer>
//     }
// }

// #[component]
// fn GroupDetails(group: Rc<opr::UnitGroup>,
//                 army: Army,
//                 side: ltn::DrawerSide,
// ) -> impl IntoView
// {
//     let opr::UnitGroup{full_cost, ..} = *group;
//     let shown = use_context::<DrawerControl>().unwrap().shown;
//     let group_name = group.formatted_name();
//     let close_button = |glyph| view! {
//         <ltn::Button color=ltn::ButtonColor::Secondary
//                      on_click=move |_| shown.set(false)> {glyph} </ltn::Button>
//     };
//     let (left_button, right_button) = match side {
//         ltn::DrawerSide::Left  => ( Some(close_button("<")), None ),
//         ltn::DrawerSide::Right => ( None, Some(close_button(">")) ),
//     };

//     view! {
//         <h3>
//             <ltn::Stack orientation=ltn::StackOrientation::Horizontal
//                         style="width: 100%; justify-content: space-between;"
//                         spacing=ltn::Size::Em(0.0)>
//                 <ltn::Stack orientation=ltn::StackOrientation::Horizontal
//                             spacing=ltn::Size::Em(1.0)>
//                     {left_button}
//                     {group_name}
//                 </ltn::Stack>
//                 <ltn::Stack orientation=ltn::StackOrientation::Horizontal
//                             spacing=ltn::Size::Em(1.0)>
//                     {format!("{full_cost} pts")}
//                     {right_button}
//                 </ltn::Stack>
//             </ltn::Stack>
//         </h3>

//         {
//             let single = group.units.len() == 1;
//             group.units.iter()
//                 .map(|unit| view! {<UnitDetails unit=Rc::clone(unit) single />})
//                 .collect_view()
//         }

//         <SpecialRulesDefList group=Rc::clone(&group) army />
//     }
// }

// #[component]
// fn UnitDetails(unit: Rc<opr::Unit>,
//                single: bool,
// ) -> impl IntoView
// {
//     let unit_name = unit.formatted_name();
//     let opr::Unit{quality, defense, full_cost, ref loadout, ref special_rules, ..} = *unit;
//     let special_rules = special_rules.clone();

//     view! {
//         <p>
//             <ltn::Stack orientation=ltn::StackOrientation::Horizontal
//                         style="width: 100%; justify-content: space-between;"
//                         spacing=ltn::Size::Em(0.0)>
//                 <span>
//                     <span class="unit">
//                         {(!single).then(|| format!("{unit_name}: "))}
//                         {format!("Q{quality} D{defense}")}
//                     </span>
//                     " "
//                     <SpecialRulesList special_rules={special_rules.clone()} />
//                 </span>
//                 {format!("{full_cost} pts")}
//             </ltn::Stack>
//         </p>
//         <p><UnitUpgradesList loadout_list={loadout.clone()} /></p>
//         <EquipmentList loadout_list={loadout.clone()} />
//     }
// }

// #[component]
// fn UnitUpgradesList(loadout_list: Vec<Rc<opr::UnitLoadout>>) -> impl IntoView {
//     view! {
//         {move || {
//             loadout_list
//                 .clone()
//                 .into_iter()
//                 .filter(|loadout| matches!(**loadout, opr::UnitLoadout::Upgrade{..}))
//                 .enumerate()
//                 .map(|(i, loadout)| {
//                     if let opr::UnitLoadout::Upgrade(ref upgrade) = *loadout {
//                         let opr::UnitUpgrade{name, ref content, ..} = upgrade;
//                         view! {
//                             {move || if i > 0 { ", " } else { "" }}
//                             {Rc::clone(name)} " (" <SpecialRulesList special_rules={content.clone()} /> ")"
//                         }
//                     } else {
//                         panic!();
//                     }
//                 })
//                 .collect_view()
//         }}
//     }
// }

// #[component]
// fn EquipmentList(loadout_list: Vec<Rc<opr::UnitLoadout>>) -> impl IntoView {
//     view! {
//         <ltn::TableContainer>
//             <ltn::Table bordered=true hoverable=true>
//                 <ltn::TableBody>
//                     {move || {
//                         loadout_list
//                             .clone()
//                             .into_iter()
//                             .filter(|loadout| matches!(**loadout, opr::UnitLoadout::Equipment{..}))
//                             .map(|loadout| {
//                                 view! {
//                                     <EquipmentItem loadout />
//                                 }
//                             })
//                             .collect_view()
//                     }}
//                 </ltn::TableBody>
//             </ltn::Table>
//         </ltn::TableContainer>
//     }
// }

// #[component]
// fn EquipmentItem(loadout: Rc<opr::UnitLoadout>) -> impl IntoView {
//     if let opr::UnitLoadout::Equipment(ref equipment) = *loadout {
//         let name = Rc::clone(&equipment.name);
//         let special_rules = equipment.special_rules.clone();
//         let opr::Equipment{count, range, attacks, ..} = *equipment;
//         view! {
//             <ltn::TableRow>
//                 <ltn::TableCell>
//                     {if count != 1
//                         {format!("{}x ", count)} else {"".to_string()}}
//                     {name}
//                 </ltn::TableCell>
//                 <ltn::TableCell>
//                     {if range != 0
//                         {format!(r#"{}""#, range )}
//                         else {"-".to_string()}}
//                 </ltn::TableCell>
//                 <ltn::TableCell>
//                     {format!("A{}", attacks)}
//                 </ltn::TableCell>
//                 <ltn::TableCell>
//                     <SpecialRulesList special_rules={special_rules.clone()} />
//                 </ltn::TableCell>
//             </ltn::TableRow>
//         }
//     } else {
//         panic!("EquipmentItem must be used on Equipment only");
//     }
// }

// #[component]
// fn SpecialRulesList(special_rules: Vec<Rc<opr::SpecialRule>>) -> impl IntoView {
//     special_rules.iter()
//     // render each rule
//         .enumerate()
//         .map(|(i, special_rule)| {
//             let separator = if i == 0 { "" } else { ", " };
//             let rating = match special_rule.rating {
//                 None => { "".to_string() },
//                 Some(rating) => { format!("({})", rating) },
//             };
//             view! {
//                 {separator}
//                 <special-rule>
//                     {Rc::clone(&special_rule.name)}
//                 </special-rule>
//                 {rating}
//             }
//         })
//         .collect_view()
// }

// #[component]
// fn SpecialRulesDefList(group: Rc<opr::UnitGroup>,
//                        army: Army,) -> impl IntoView {
//     view! {
//         <specialrules-def-list>
//             {move || {
//                 let group = Rc::clone(&group);
//                 army.army_data.with(
//                     move |army_data| {
//                         if let Some(Ok(army_data)) = army_data {
//                             // the rules def
//                             let opr::Army{ref special_rules, ..} = **army_data;
//                             let special_rules_def = special_rules;

//                             view!{
//                                 {match common_rules_def() {
//                                     Ok(common_rules_def) => view! {
//                                         {rules_descriptions_from_list_for_group(
//                                             Rc::clone(&group), &common_rules_def.clone())}
//                                     }.into_view(),
//                                     Err(message) => view! {
//                                         <ltn::Alert variant=ltn::AlertVariant::Danger>
//                                             <AlertContent slot>{message}</AlertContent>
//                                         </ltn::Alert>
//                                     }.into_view(),
//                                 }}
//                                 {rules_descriptions_from_list_for_group(
//                                     Rc::clone(&group), special_rules_def)}
//                             }.into_view()
//                         } else {
//                             // cannot happen - FIXME should pass opr::Army directly instead?
//                             view!{}.into_view()
//                         }
//                     }
//                 )
//             }}
//         </specialrules-def-list>
//     }
// }

/// extract common-rules definitions from the Context Resource, or an
/// error string to display
fn common_rules_def() -> Result<Vec<Rc<opr::SpecialRuleDef>>, String> {
    let common_rules_def = use_context::<
            Resource<Option<opr::GameSystem>, Result<Rc<opr::CommonRules>, String>>
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

fn rules_descriptions_from_list_for_group(group: Rc<opr::UnitGroup>,
                                          rules_def: &[Rc<opr::SpecialRuleDef>]
) -> impl IntoView {
    rules_def
        .iter()
        .map(|rule_def| {
            if group_uses_rule(Rc::clone(&group), rule_def) {
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

fn group_uses_rule(group: Rc<opr::UnitGroup>,
                   rule_def: &Rc<opr::SpecialRuleDef>) -> bool {
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
