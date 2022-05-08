use crate::{button_system, despawn_screen, global, FontAssets, ImageAssets};

mod styles;
pub use styles::*;

use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(MenuState::Active)
            .add_system_set(SystemSet::on_enter(global::GameState::Menu).with_system(menu_setup))
            .add_system_set(
                SystemSet::on_update(global::GameState::Menu)
                    .with_system(side_panel_action)
                    .with_system(button_system),
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

fn menu_setup(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    mut menu_state: ResMut<State<MenuState>>,
    player: Res<global::Player>,
) {
    // Reset state for recurring visit to this page.
    if *menu_state.current() == MenuState::Inactive {
        menu_state.set(MenuState::Active).unwrap();
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
            });
        });
}

fn side_panel_action(
    interaction_query: Query<
        (&Interaction, &SidePanelButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu_state: ResMut<State<MenuState>>,
    mut game_state: ResMut<State<global::GameState>>,
) {
    for (interaction, menu_button_action) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                SidePanelButtonAction::Exit => {
                    game_state.set(global::GameState::Overworld).unwrap();
                    menu_state.set(MenuState::Inactive).unwrap();
                }
                _ => todo!("Unhandled menu button action!!"), // TODO
            }
        }
    }
}
