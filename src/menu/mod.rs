use crate::{button_system, despawn_children, despawn_screen, global, FontAssets};

mod styles;
pub use styles::*;

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(MenuState::Active)
            .add_state(SubPanelState::Inactive)
            .add_system_set(SystemSet::on_enter(global::GameState::Menu).with_system(menu_setup))
            .add_system_set(
                SystemSet::on_update(global::GameState::Menu)
                    .with_system(side_panel_action)
                    .with_system(item_button_action)
                    .with_system(item_list_scroll)
                    .with_system(button_system),
            )
            .add_system_set(SystemSet::on_enter(SubPanelState::Item).with_system(spawn_item_menu))
            .add_system_set(
                SystemSet::on_exit(SubPanelState::Item).with_system(despawn_children::<SubPanel>),
            )
            .add_system_set(
                SystemSet::on_exit(global::GameState::Menu)
                    .with_system(despawn_screen::<MenuScreen>),
            );
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum MenuState {
    Active,
    Inactive,
}

#[derive(Component, Debug, Clone)]
enum SidePanelButtonAction {
    Item,
    Equip,
    Magic,
    Exit,
    // TODO: maybe settings?
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum SubPanelState {
    Inactive,
    Item,
    Equip,
    Magic,
}

#[derive(Component)]
struct MenuScreen;

#[derive(Component)]
struct HPText;

#[derive(Component)]
struct HPBar;

#[derive(Component)]
struct MPText;

#[derive(Component)]
struct MPBar;

#[derive(Component)]
struct LimitBar;

#[derive(Component)]
struct StrengthText;

#[derive(Component)]
struct WisdomText;

#[derive(Component)]
struct DefenseText;

#[derive(Component)]
struct SubPanel;

#[derive(Component, Default)]
struct ItemList {
    position: f32,
}

#[derive(Component, Deref)]
struct ItemButton(usize); // holds item id

#[derive(Component)]
struct SubPanelDescContainer;

#[derive(Component)]
struct SubPanelDesc;

fn menu_setup(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    mut menu_state: ResMut<State<MenuState>>,
    mut subpanel_state: ResMut<State<SubPanelState>>,
    player: Res<global::Player>,
) {
    // Reset state for recurring visit to this page.
    if *menu_state.current() == MenuState::Inactive {
        menu_state.set(MenuState::Active).unwrap();
    }
    if *subpanel_state.current() != SubPanelState::Inactive {
        subpanel_state.set(SubPanelState::Inactive).unwrap();
    }

    let hp_perc = player.stats.hp as f32 / player.stats.hp_max as f32 * 100.;
    let mp_perc = player.stats.mp as f32 / player.stats.mp_max as f32 * 100.;
    let max_xp = global::XP_TABLE[player.stats.level as usize - 1];
    let xp_perc = player.stats.experience as f32 / max_xp as f32;

    commands
        .spawn_bundle(styled_menu_container())
        .insert(MenuScreen)
        .with_children(|p| {
            p.spawn_bundle(styled_side_panel()).with_children(|p| {
                for side_panel_button in [
                    SidePanelButtonAction::Item,
                    SidePanelButtonAction::Equip,
                    SidePanelButtonAction::Magic,
                    SidePanelButtonAction::Exit,
                ] {
                    p.spawn_bundle(styled_button())
                        .insert(side_panel_button.clone())
                        .with_children(|p| {
                            p.spawn_bundle(styled_text_bundle(
                                format!("{:?}", &side_panel_button),
                                &font_assets,
                            ));
                        });
                }
            });
            p.spawn_bundle(styled_main_panel()).with_children(|p| {
                p.spawn_bundle(styled_stats_panel()).with_children(|p| {
                    p.spawn_bundle(styled_stat_group_container())
                        .with_children(|p| {
                            p.spawn_bundle(styled_stat_with_bar_container())
                                .with_children(|p| {
                                    p.spawn_bundle(styled_stat_out_of_text(
                                        &font_assets,
                                        "HP: ".to_string(),
                                        player.stats.hp,
                                        player.stats.hp_max,
                                    ))
                                    .insert(HPText);
                                    p.spawn_bundle(styled_stat_bar_container()).with_children(
                                        |p| {
                                            p.spawn_bundle(styled_stat_bar(hp_perc, Color::GREEN))
                                                .insert(HPBar);
                                        },
                                    );
                                });
                            p.spawn_bundle(styled_stat_with_bar_container())
                                .with_children(|p| {
                                    p.spawn_bundle(styled_stat_out_of_text(
                                        &font_assets,
                                        "MP: ".to_string(),
                                        player.stats.mp,
                                        player.stats.mp_max,
                                    ))
                                    .insert(MPText);
                                    p.spawn_bundle(styled_stat_bar_container()).with_children(
                                        |p| {
                                            p.spawn_bundle(styled_stat_bar(mp_perc, Color::BLUE))
                                                .insert(MPBar);
                                        },
                                    );
                                });
                            p.spawn_bundle(styled_stat_with_bar_container())
                                .with_children(|p| {
                                    p.spawn_bundle(styled_text_bundle(
                                        format!("Limit: {} %", player.limit),
                                        &font_assets,
                                    ));
                                    p.spawn_bundle(styled_stat_bar_container()).with_children(
                                        |p| {
                                            p.spawn_bundle(styled_stat_bar(
                                                player.limit as f32,
                                                if player.limit < 100 {
                                                    Color::ORANGE
                                                } else {
                                                    Color::RED
                                                },
                                            ))
                                            .insert(LimitBar);
                                        },
                                    );
                                });
                            p.spawn_bundle(styled_stat_with_bar_container())
                                .with_children(|p| {
                                    p.spawn_bundle(styled_stat_out_of_text(
                                        &font_assets,
                                        "XP: ".to_string(),
                                        player.stats.experience,
                                        max_xp,
                                    ));
                                    p.spawn_bundle(styled_stat_bar_container()).with_children(
                                        |p| {
                                            p.spawn_bundle(styled_stat_bar(xp_perc, Color::YELLOW));
                                        },
                                    );
                                });
                        });

                    p.spawn_bundle(styled_stat_group_container())
                        .with_children(|p| {
                            p.spawn_bundle(styled_text_bundle(
                                format!("Strength: {}", player.stats.strength),
                                &font_assets,
                            ))
                            .insert(StrengthText);
                            p.spawn_bundle(styled_text_bundle(
                                format!("Wisdom: {}", player.stats.wisdom),
                                &font_assets,
                            ))
                            .insert(WisdomText);
                            p.spawn_bundle(styled_text_bundle(
                                format!("Defense: {}", player.stats.defense),
                                &font_assets,
                            ))
                            .insert(DefenseText);
                        });
                });

                p.spawn_bundle(styled_sub_panel()).insert(SubPanel);
            });
        });
}

fn side_panel_action(
    interaction_query: Query<
        (&Interaction, &SidePanelButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu_state: ResMut<State<MenuState>>,
    mut subpanel_state: ResMut<State<SubPanelState>>,
    mut game_state: ResMut<State<global::GameState>>,
) {
    for (interaction, menu_button_action) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                SidePanelButtonAction::Item => {
                    // Switch subpanel state.
                    match *subpanel_state.current() {
                        SubPanelState::Inactive => subpanel_state.set(SubPanelState::Item).unwrap(),
                        SubPanelState::Item => subpanel_state.set(SubPanelState::Inactive).unwrap(),
                        _ => (),
                    }
                }
                SidePanelButtonAction::Exit => {
                    game_state.set(global::GameState::Overworld).unwrap();
                    menu_state.set(MenuState::Inactive).unwrap();
                    if *subpanel_state.current() != SubPanelState::Inactive {
                        subpanel_state.set(SubPanelState::Inactive).unwrap();
                    }
                }
                _ => todo!("Unhandled menu button action!!"), // TODO
            }
        }
    }
}

fn spawn_item_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    subpanel: Query<Entity, With<SubPanel>>,
    items: ResMut<global::PlayerItemInventory>,
    item_table: Res<global::ItemTable>,
) {
    commands.entity(subpanel.single()).with_children(|p| {
        p.spawn_bundle(styled_sub_sub_panel()).with_children(|p| {
            p.spawn_bundle(styled_item_list())
                .insert(ItemList::default())
                .with_children(|p| {
                    for (item_id, _) in items.iter() {
                        p.spawn_bundle(styled_subpanel_button())
                            .insert(ItemButton(*item_id))
                            .with_children(|p| {
                                p.spawn_bundle(styled_text_bundle(
                                    format!("{}", item_table.get(&item_id).unwrap().name),
                                    &font_assets,
                                ));
                            });
                    }
                });
        });
        p.spawn_bundle(styled_sub_sub_panel())
            .insert(SubPanelDescContainer)
            .with_children(|p| {
                p.spawn_bundle(styled_text_bundle("", &font_assets))
                    .insert(SubPanelDesc);
            });
    });
}

fn item_button_action(
    mut commands: Commands,
    children_query: Query<&Children>,
    mut interaction_query: Query<(&Interaction, &ItemButton), (Changed<Interaction>, With<Button>)>,
    mut desc_entity: Query<Entity, With<SubPanelDesc>>,
    mut items: ResMut<global::PlayerItemInventory>,
    mut player: ResMut<global::Player>,
    item_table: Res<global::ItemTable>,
    font_assets: Res<FontAssets>,
) {
    for (interaction, button_action) in interaction_query.iter_mut() {
        let item = item_table.get(&button_action.0).unwrap().clone();

        if let Ok(children) = children_query.get(desc_entity.single()) {
            for child in children.iter() {
                commands.entity(*child).despawn_recursive();
            }
        }
        if *interaction == Interaction::Clicked {
            if let Some(_) = items.get_mut(&item.id) {
                if item.stats.hp > 0 {
                    player.stats.hp =
                        std::cmp::min(player.stats.hp + item.stats.hp, player.stats.hp_max);
                }
                if item.stats.mp > 0 {
                    player.stats.mp =
                        std::cmp::min(player.stats.mp + item.stats.mp, player.stats.mp_max);
                }

                *items.get_mut(&item.id).unwrap() -= 1;

                // Remove item if reach 0.
                if *items.get_mut(&item.id).unwrap() == 0 {
                    items.remove(&item.id);
                }
            }
        } else if *interaction == Interaction::Hovered {
            commands
                .entity(desc_entity.single_mut())
                .with_children(|p| {
                    let mut desc_text = if item.stats.hp > 0 {
                        format!("Heals {} HP", item.stats.hp)
                    } else {
                        format!("Heals {} MP", item.stats.mp)
                    };
                    let quantity = if let Some(q) = items.get_mut(&item.id) {
                        *q
                    } else {
                        0
                    };
                    desc_text.push_str(&format!("\nQuantity: {}", quantity));
                    p.spawn_bundle(styled_text_bundle(desc_text, &font_assets));
                });
        } else {
            if let Ok(children) = children_query.get(desc_entity.single()) {
                for child in children.iter() {
                    commands.entity(*child).despawn_recursive();
                }
            }
        }
    }
}

fn item_list_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ItemList, &mut Style, &Children, &Node)>,
    query_item: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, children, uinode) in query_list.iter_mut() {
            let items_height: f32 = children
                .iter()
                .map(|entity| query_item.get(*entity).unwrap().size.y)
                .sum();
            let panel_height = uinode.size.y;
            let max_scroll = (items_height - panel_height).max(0.);
            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };
            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.position.top = Val::Px(scrolling_list.position);
        }
    }
}
