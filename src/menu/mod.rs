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
                    .with_system(close_menu)
                    .with_system(side_panel_action)
                    .with_system(scroll_list_scroll)
                    .with_system(button_system),
            )
            .add_system_set(
                SystemSet::on_enter(SubPanelState::Item).with_system(item_menu::spawn_item_menu),
            )
            .add_system_set(
                SystemSet::on_update(SubPanelState::Item)
                    .with_system(item_menu::item_button_action),
            )
            .add_system_set(
                SystemSet::on_exit(SubPanelState::Item).with_system(despawn_children::<SubPanel>),
            )
            .add_system_set(
                SystemSet::on_enter(SubPanelState::Magic).with_system(magic_menu::spawn_magic_menu),
            )
            .add_system_set(
                SystemSet::on_update(SubPanelState::Magic)
                    .with_system(magic_menu::magic_slot_button_action)
                    .with_system(magic_menu::magic_button_action),
            )
            .add_system_set(
                SystemSet::on_exit(SubPanelState::Magic).with_system(despawn_children::<SubPanel>),
            )
            .add_system_set(
                SystemSet::on_enter(SubPanelState::Equip).with_system(equip_menu::spawn_equip_menu),
            )
            .add_system_set(
                SystemSet::on_update(SubPanelState::Equip)
                    .with_system(equip_menu::equip_slot_button_action)
                    .with_system(equip_menu::equip_button_action),
            )
            .add_system_set(
                SystemSet::on_exit(SubPanelState::Equip).with_system(despawn_children::<SubPanel>),
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
struct ScrollList {
    position: f32,
}

#[derive(Component, Deref)]
struct ItemButton(usize); // holds item id

#[derive(Component)]
struct SubPanelDescContainer;

#[derive(Component)]
struct SubPanelDesc;

#[derive(Component)]
struct MagicListContainer;

#[derive(Component, Deref)]
struct MagicSlotText(usize);

#[derive(Default, Deref)]
struct MagicSlotSelected(usize);

#[derive(Component, Deref)]
struct MagicSlotButton(usize);

#[derive(Component, Deref)]
struct MagicButton(global::PlayerAttack);

#[derive(Component)]
struct EquipListContainer;

#[derive(Component, Deref)]
struct EquipSlotText(global::ItemType);

#[derive(Component, Deref, Debug)]
struct EquipSlotSelected(global::ItemType);

#[derive(Component, Deref)]
struct EquipSlotButton(global::ItemType);

#[derive(Component, Deref)]
struct EquipButton(usize); // item id

fn menu_setup(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    mut menu_state: ResMut<State<MenuState>>,
    mut subpanel_state: ResMut<State<SubPanelState>>,
    player: Res<global::Player>,
) {
    // Ensures close_menu() doesn't conflict with open_menu() from overworld.rs.
    commands.insert_resource(Timer::from_seconds(global::MENU_TOGGLE_DURATION, false));

    // Initialize selected slots resources.
    commands.insert_resource(MagicSlotSelected(0));
    commands.insert_resource(EquipSlotSelected(global::ItemType::Weapon));

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
    let xp_perc = player.stats.experience as f32 / max_xp as f32 * 100.;

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
                        SubPanelState::Item => subpanel_state.set(SubPanelState::Inactive).unwrap(),
                        _ => subpanel_state.set(SubPanelState::Item).unwrap(),
                    }
                }
                SidePanelButtonAction::Magic => {
                    // Switch subpanel state.
                    match *subpanel_state.current() {
                        SubPanelState::Magic => {
                            subpanel_state.set(SubPanelState::Inactive).unwrap()
                        }
                        _ => subpanel_state.set(SubPanelState::Magic).unwrap(),
                    }
                }
                SidePanelButtonAction::Equip => {
                    // Switch subpanel state.
                    match *subpanel_state.current() {
                        SubPanelState::Equip => {
                            subpanel_state.set(SubPanelState::Inactive).unwrap()
                        }
                        _ => subpanel_state.set(SubPanelState::Equip).unwrap(),
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

fn scroll_list_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollList, &mut Style, &Children, &Node)>,
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

fn close_menu(
    time: Res<Time>,
    mut timer: ResMut<Timer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<global::GameState>>,
    mut menu_state: ResMut<State<MenuState>>,
    mut subpanel_state: ResMut<State<SubPanelState>>,
) {
    if timer.tick(time.delta()).finished() && keyboard_input.just_pressed(KeyCode::P) {
        game_state.set(global::GameState::Overworld).unwrap();
        if *menu_state.current() == MenuState::Active {
            menu_state.set(MenuState::Inactive).unwrap();
        }
        if *subpanel_state.current() != SubPanelState::Inactive {
            subpanel_state.set(SubPanelState::Inactive).unwrap();
        }
    }
}

//==============================================================================
// Item menu
//==============================================================================
mod item_menu {
    use super::*;

    pub(super) fn spawn_item_menu(
        mut commands: Commands,
        font_assets: Res<FontAssets>,
        subpanel: Query<Entity, With<SubPanel>>,
        items: Res<global::PlayerItemInventory>,
        item_table: Res<global::ItemTable>,
    ) {
        commands.entity(subpanel.single()).with_children(|p| {
            p.spawn_bundle(styled_sub_sub_panel()).with_children(|p| {
                p.spawn_bundle(styled_scroll_list())
                    .insert(ScrollList::default())
                    .with_children(|p| {
                        for (item_id, _) in items.iter() {
                            p.spawn_bundle(styled_subpanel_button())
                                .insert(ItemButton(*item_id))
                                .with_children(|p| {
                                    p.spawn_bundle(styled_text_bundle(
                                        item_table.get(item_id).unwrap().name.to_string(),
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

    pub(super) fn item_button_action(
        mut commands: Commands,
        children_query: Query<&Children>,
        mut interaction_query: Query<
            (&Interaction, &ItemButton),
            (Changed<Interaction>, With<Button>),
        >,
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
                if items.get_mut(&item.id).is_some() {
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
                    if *items.get(&item.id).unwrap() == 0 {
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
            } else if let Ok(children) = children_query.get(desc_entity.single()) {
                for child in children.iter() {
                    commands.entity(*child).despawn_recursive();
                }
            }
        }
    }
}

//==============================================================================
// Magic menu
//==============================================================================
mod magic_menu {
    use super::*;

    // FIXME: why is the magic menu laggy?
    pub(super) fn spawn_magic_menu(
        mut commands: Commands,
        font_assets: Res<FontAssets>,
        subpanel: Query<Entity, With<SubPanel>>,
        magic_equipped: Res<global::PlayerMagicEquipped>,
    ) {
        commands.entity(subpanel.single()).with_children(|p| {
            p.spawn_bundle(styled_sub_sub_panel()).with_children(|p| {
                p.spawn_bundle(styled_magic_slots_container())
                    .with_children(|p| {
                        for (i, magic) in magic_equipped.iter().enumerate() {
                            p.spawn_bundle(styled_magic_equipped_container())
                                .with_children(|p| {
                                    let magic_name = if let Some(magic) = magic {
                                        magic.name.clone()
                                    } else {
                                        "None".to_string()
                                    };
                                    let slot_text = format!("Slot {}: {}", i, magic_name);
                                    p.spawn_bundle(styled_magic_equipped_text_container())
                                        .with_children(|p| {
                                            p.spawn_bundle(styled_text_bundle(
                                                slot_text,
                                                &font_assets,
                                            ))
                                            .insert(MagicSlotText(i));
                                        });
                                    p.spawn_bundle(styled_button())
                                        .insert(MagicSlotButton(i))
                                        .with_children(|p| {
                                            p.spawn_bundle(styled_text_bundle(
                                                "Change",
                                                &font_assets,
                                            ));
                                        });
                                });
                        }
                    });

                p.spawn_bundle(styled_magic_panel_desc_container())
                    .insert(SubPanelDescContainer)
                    .with_children(|p| {
                        p.spawn_bundle(styled_text_bundle("", &font_assets))
                            .insert(SubPanelDesc);
                    });
            });

            p.spawn_bundle(styled_sub_sub_panel())
                .insert(MagicListContainer);
        });
    }

    pub(super) fn magic_slot_button_action(
        mut commands: Commands,
        children_query: Query<&Children>,
        mut interaction_query: Query<
            (&Interaction, &MagicSlotButton),
            (Changed<Interaction>, With<Button>),
        >,
        magic_list_container: Query<Entity, With<MagicListContainer>>,
        attack_inv: Res<global::PlayerAttackInventory>,
        font_assets: Res<FontAssets>,
        mut magic_slot_selected: ResMut<MagicSlotSelected>,
    ) {
        for (interaction, button_action) in interaction_query.iter_mut() {
            if *interaction == Interaction::Clicked {
                // Despawn magic list.
                if let Ok(children) = children_query.get(magic_list_container.single()) {
                    for child in children.iter() {
                        commands.entity(*child).despawn_recursive();
                    }
                }

                magic_slot_selected.0 = button_action.0;
                commands
                    .entity(magic_list_container.single())
                    .with_children(|p| {
                        p.spawn_bundle(styled_scroll_list())
                            .insert(ScrollList::default())
                            .with_children(|p| {
                                for attack in attack_inv.iter() {
                                    if let Some(atk_type) = &attack.attack_type {
                                        if *atk_type == global::PlayerAttackType::Magic {
                                            p.spawn_bundle(styled_subpanel_button())
                                                .insert(MagicButton(attack.clone()))
                                                .with_children(|p| {
                                                    p.spawn_bundle(styled_text_bundle(
                                                        attack.name.to_string(),
                                                        &font_assets,
                                                    ));
                                                });
                                        }
                                    }
                                }
                            });
                    });
            }
        }
    }

    pub(super) fn magic_button_action(
        mut commands: Commands,
        children_query: Query<&Children>,
        mut interaction_query: Query<
            (&Interaction, &MagicButton),
            (Changed<Interaction>, With<Button>),
        >,
        mut desc_entity: Query<Entity, With<SubPanelDesc>>,
        magic_list_container: Query<Entity, With<MagicListContainer>>,
        mut magic_slot_query: Query<(Entity, &MagicSlotText)>,
        mut magic_slot_text_query: Query<&mut Text>,
        font_assets: Res<FontAssets>,
        magic_slot_selected: Res<MagicSlotSelected>,
        mut magic_equipped: ResMut<global::PlayerMagicEquipped>,
    ) {
        for (interaction, button_action) in interaction_query.iter_mut() {
            // Despawn description menu.
            if let Ok(children) = children_query.get(desc_entity.single()) {
                for child in children.iter() {
                    commands.entity(*child).despawn_recursive();
                }
            }

            if *interaction == Interaction::Clicked {
                // Set selected magic to slot.
                magic_equipped[magic_slot_selected.0] = Some(button_action.0.clone());

                // Despawn magic list.
                if let Ok(children) = children_query.get(magic_list_container.single()) {
                    for child in children.iter() {
                        commands.entity(*child).despawn_recursive();
                    }
                }

                // Update slot magic text.
                let slot_text = format!("Slot {}: {}", magic_slot_selected.0, button_action.name);
                for (magic_slot_entity, magic_slot_text) in magic_slot_query.iter_mut() {
                    if magic_slot_text.0 == magic_slot_selected.0 {
                        *magic_slot_text_query.get_mut(magic_slot_entity).unwrap() =
                            styled_text(slot_text.clone(), &font_assets);
                        break;
                    }
                }
            } else if *interaction == Interaction::Hovered {
                // Show magic description.
                commands
                    .entity(desc_entity.single_mut())
                    .with_children(|p| {
                        let element = if let Some(e) = &button_action.element {
                            format!("{:?}", e)
                        } else {
                            "Normal".to_string()
                        };
                        let desc_text = format!(
                            "Deals Tier {} {} damage.\nCosts {} MP",
                            button_action.tier, element, button_action.mp_use
                        );
                        p.spawn_bundle(styled_text_bundle(desc_text, &font_assets));
                    });
            }
        }
    }
}

//==============================================================================
// Equip menu
//==============================================================================
mod equip_menu {
    use super::*;

    // FIXME: why is the equip menu laggy?
    pub(super) fn spawn_equip_menu(
        mut commands: Commands,
        font_assets: Res<FontAssets>,
        subpanel: Query<Entity, With<SubPanel>>,
        equipment_equipped: Res<global::PlayerEquipmentEquipped>,
    ) {
        commands.entity(subpanel.single()).with_children(|p| {
            p.spawn_bundle(styled_sub_sub_panel()).with_children(|p| {
                p.spawn_bundle(styled_magic_slots_container())
                    .with_children(|p| {
                        for (equip, equip_type) in [
                            (&equipment_equipped.weapon, global::ItemType::Weapon),
                            (&equipment_equipped.armor, global::ItemType::Armor),
                            (&equipment_equipped.accessory, global::ItemType::Accessory),
                        ] {
                            p.spawn_bundle(styled_magic_equipped_container())
                                .with_children(|p| {
                                    let equip_name = if let Some(equip) = equip {
                                        equip.name.clone()
                                    } else {
                                        "None".to_string()
                                    };
                                    let slot_text =
                                        format!("{:?} equipped: {}", equip_type, equip_name);
                                    p.spawn_bundle(styled_magic_equipped_text_container())
                                        .with_children(|p| {
                                            p.spawn_bundle(styled_text_bundle(
                                                slot_text,
                                                &font_assets,
                                            ))
                                            .insert(EquipSlotText(equip_type.clone()));
                                        });
                                    p.spawn_bundle(styled_button())
                                        .insert(EquipSlotButton(equip_type.clone()))
                                        .with_children(|p| {
                                            p.spawn_bundle(styled_text_bundle(
                                                "Change",
                                                &font_assets,
                                            ));
                                        });
                                });
                        }
                    });

                p.spawn_bundle(styled_magic_panel_desc_container())
                    .insert(SubPanelDescContainer)
                    .with_children(|p| {
                        p.spawn_bundle(styled_text_bundle("", &font_assets))
                            .insert(SubPanelDesc);
                    });
            });

            p.spawn_bundle(styled_sub_sub_panel())
                .insert(EquipListContainer);
        });
    }

    pub(super) fn equip_slot_button_action(
        mut commands: Commands,
        children_query: Query<&Children>,
        mut interaction_query: Query<
            (&Interaction, &EquipSlotButton),
            (Changed<Interaction>, With<Button>),
        >,
        equip_list_container: Query<Entity, With<EquipListContainer>>,
        item_inv: Res<global::PlayerItemInventory>,
        item_table: Res<global::ItemTable>,
        font_assets: Res<FontAssets>,
        mut equip_slot_selected: ResMut<EquipSlotSelected>,
    ) {
        for (interaction, button_action) in interaction_query.iter_mut() {
            if *interaction == Interaction::Clicked {
                // Despawn equipment list.
                if let Ok(children) = children_query.get(equip_list_container.single()) {
                    for child in children.iter() {
                        commands.entity(*child).despawn_recursive();
                    }
                }

                equip_slot_selected.0 = button_action.0.clone();
                commands
                    .entity(equip_list_container.single())
                    .with_children(|p| {
                        p.spawn_bundle(styled_scroll_list())
                            .insert(ScrollList::default())
                            .with_children(|p| {
                                for (item_id, _) in item_inv.iter() {
                                    if button_action.0 == item_table.get(item_id).unwrap().item_type
                                    {
                                        p.spawn_bundle(styled_subpanel_button())
                                            .insert(EquipButton(*item_id))
                                            .with_children(|p| {
                                                p.spawn_bundle(styled_text_bundle(
                                                    item_table
                                                        .get(item_id)
                                                        .unwrap()
                                                        .name
                                                        .to_string(),
                                                    &font_assets,
                                                ));
                                            });
                                    }
                                }
                            });
                    });
            }
        }
    }

    // TODO:
    // - update stats panel
    pub(super) fn equip_button_action(
        mut commands: Commands,
        children_query: Query<&Children>,
        mut interaction_query: Query<
            (&Interaction, &EquipButton),
            (Changed<Interaction>, With<Button>),
        >,
        mut desc_entity: Query<Entity, With<SubPanelDesc>>,
        equip_list_container: Query<Entity, With<EquipListContainer>>,
        mut equip_slot_query: Query<(Entity, &EquipSlotText)>,
        mut equip_slot_text_query: Query<&mut Text>,
        mut player: ResMut<global::Player>,
        font_assets: Res<FontAssets>,
        item_table: Res<global::ItemTable>,
        equip_slot_selected: Res<EquipSlotSelected>,
        mut equipment_equipped: ResMut<global::PlayerEquipmentEquipped>,
    ) {
        for (interaction, button_action) in interaction_query.iter_mut() {
            // Despawn description menu.
            if let Ok(children) = children_query.get(desc_entity.single()) {
                for child in children.iter() {
                    commands.entity(*child).despawn_recursive();
                }
            }

            if *interaction == Interaction::Clicked {
                // Get currently equipped equipment.
                let selected_equip: &mut Option<global::Item>;
                match equip_slot_selected.0 {
                    global::ItemType::Weapon => selected_equip = &mut equipment_equipped.weapon,
                    global::ItemType::Armor => selected_equip = &mut equipment_equipped.armor,
                    global::ItemType::Accessory | _ => {
                        selected_equip = &mut equipment_equipped.accessory
                    }
                };

                if let Some(equip) = &selected_equip {
                    player.stats.subtract_item_stats(&equip.stats);
                }

                // Set selected equip to slot.
                *selected_equip = Some(item_table.get(&button_action.0).unwrap().clone());

                // Add new weapon stats from player stats.
                player
                    .stats
                    .add_item_stats(&selected_equip.as_ref().unwrap().stats);

                // Despawn equipment list.
                if let Ok(children) = children_query.get(equip_list_container.single()) {
                    for child in children.iter() {
                        commands.entity(*child).despawn_recursive();
                    }
                }

                // Update slot equip text.
                let equip_name = if let Some(equip) = &*selected_equip {
                    equip.name.clone()
                } else {
                    "None".to_string()
                };
                let slot_text = format!("{:?} equipped: {}", equip_slot_selected.0, equip_name);
                for (equip_slot_entity, equip_slot_text) in equip_slot_query.iter_mut() {
                    if equip_slot_text.0 == equip_slot_selected.0 {
                        *equip_slot_text_query.get_mut(equip_slot_entity).unwrap() =
                            styled_text(slot_text.clone(), &font_assets);
                        break;
                    }
                }
            } else if *interaction == Interaction::Hovered {
                // Show equip description.
                commands
                    .entity(desc_entity.single_mut())
                    .with_children(|p| {
                        p.spawn_bundle(styled_text_bundle(
                            item_table
                                .get(&button_action.0)
                                .unwrap()
                                .stats
                                .print_equip_stats(),
                            &font_assets,
                        ));
                    });
            }
        }
    }
}
