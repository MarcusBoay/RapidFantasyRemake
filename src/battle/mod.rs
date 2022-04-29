use crate::{
    button_system, despawn_screen, set_visible_recursive, FontAssets, ImageAssets, Player, Stats,
};

mod styles;
pub use styles::*;

use super::GameState;
use bevy::prelude::*;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(BattleState::Initialization)
            .add_system_set(SystemSet::on_enter(GameState::Battle).with_system(battle_setup))
            .add_system_set(SystemSet::on_update(GameState::Battle).with_system(battle_init))
            .add_system_set(SystemSet::on_enter(BattleState::Idle).with_system(battle_idle_setup))
            .add_system_set(
                SystemSet::on_update(BattleState::Idle)
                    .with_system(show_player_action_container)
                    .with_system(battle_action)
                    .with_system(button_system),
            )
            .add_system_set(
                SystemSet::on_exit(BattleState::Idle).with_system(hide_player_action_container),
            )
            .add_system_set(
                SystemSet::on_enter(BattleState::PlayerAttack).with_system(player_attack_setup),
            )
            .add_system_set(
                SystemSet::on_update(BattleState::PlayerAttack).with_system(battle_update),
            )
            .add_system_set(
                SystemSet::on_enter(BattleState::EnemyAttack).with_system(enemy_attack_setup),
            )
            .add_system_set(
                SystemSet::on_update(BattleState::EnemyAttack).with_system(battle_update),
            )
            // When exiting the state, despawn everything that was spawned for this screen.
            .add_system_set(
                SystemSet::on_exit(GameState::Battle).with_system(despawn_screen::<BattleScreen>),
            );
    }
}

#[derive(Component)]
struct BattleScreen;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum BattleState {
    Initialization,
    Idle,
    PlayerAttack,
    EnemyAttack,
    Win,
    Lose,
    // maybe magic menu state?
}
#[derive(Component, Debug, Clone)]
pub enum PlayerButtonAction {
    Attack,
    Magic,
    Block,
    Item,
}

#[derive(Component)]
struct PlayerActionContainer;

#[derive(Component)]
struct Announcement;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct EnemySprite; // TODO: when we need to change boss sprite due to phases

const TEXT_DURATION: f32 = 1.5;

fn battle_setup(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    font_assets: Res<FontAssets>,
) {
    // This will set BattleState::Initialization to BattleState::Idle in 1 second.
    commands.insert_resource(Timer::from_seconds(TEXT_DURATION, false));

    // TODO: use actual enemy stats
    commands.spawn().insert(Enemy).insert(Stats {
        hp: 500,
        mp: 234,
        attack: 531,
    });

    commands
        .spawn_bundle(styled_battle_screen())
        .insert(BattleScreen)
        .with_children(|p| {
            p.spawn_bundle(styled_bottom_container())
                .with_children(|p| {
                    p.spawn_bundle(styled_player_stats_container())
                        .with_children(|p| {
                            p.spawn_bundle(styled_player_stats_child_container())
                                .with_children(|p| {
                                    p.spawn_bundle(styled_player_hp_text(&font_assets));
                                    p.spawn_bundle(styled_player_hp_bar_container())
                                        .with_children(|p| {
                                            p.spawn_bundle(styled_player_hp_bar());
                                        });
                                });

                            p.spawn_bundle(styled_player_stats_child_container())
                                .with_children(|p| {
                                    p.spawn_bundle(styled_player_mp_text(&font_assets));
                                    p.spawn_bundle(styled_player_mp_bar_container())
                                        .with_children(|p| {
                                            p.spawn_bundle(styled_player_mp_bar());
                                        });
                                });

                            p.spawn_bundle(styled_player_stats_child_container())
                                .with_children(|p| {
                                    p.spawn_bundle(styled_player_limit_break_text(&font_assets));
                                    p.spawn_bundle(styled_player_limit_break_bar_container())
                                        .with_children(|p| {
                                            p.spawn_bundle(styled_player_limit_break_bar());
                                        });
                                });
                        });

                    p.spawn_bundle(styled_player_action_container())
                        .insert(PlayerActionContainer)
                        .with_children(|p| {
                            for player_button_action in [
                                PlayerButtonAction::Attack,
                                PlayerButtonAction::Magic,
                                PlayerButtonAction::Block,
                                PlayerButtonAction::Item,
                            ] {
                                p.spawn_bundle(styled_player_action_button())
                                    .insert(player_button_action.clone())
                                    .with_children(|p| {
                                        p.spawn_bundle(styled_player_action_button_text(
                                            &player_button_action,
                                            &font_assets,
                                        ));
                                    });
                            }
                        });
                });

            p.spawn_bundle(styled_announcement_container())
                .with_children(|p| {
                    p.spawn_bundle(styled_announcement_text(&font_assets))
                        .insert(Announcement);
                });

            p.spawn_bundle(styled_battle_images_container())
                .with_children(|p| {
                    p.spawn_bundle(styled_battle_portrait(image_assets.player_battle.clone()));
                    p.spawn_bundle(styled_battle_portrait(image_assets.enemy1.clone()))
                        .insert(EnemySprite);
                });
        });
}

fn battle_init(
    time: Res<Time>,
    mut timer: ResMut<Timer>,
    mut battle_state: ResMut<State<BattleState>>,
) {
    if timer.tick(time.delta()).finished() {
        match battle_state.as_ref().current() {
            BattleState::Initialization => battle_state.set(BattleState::Idle).unwrap(),
            _ => (),
        }
    }
}

fn battle_idle_setup(mut query: Query<&mut Text, With<Announcement>>) {
    let mut announcement_text = query.single_mut();
    announcement_text.sections[0].value = format!("");
}

fn battle_action(
    interaction_query: Query<
        (&Interaction, &PlayerButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut battle_state: ResMut<State<BattleState>>,
) {
    for (interaction, menu_button_action) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                PlayerButtonAction::Attack => battle_state.set(BattleState::PlayerAttack).unwrap(),
                _ => unimplemented!("Unhandled player action button!!"), // TODO
            }
        }
    }
}

fn player_attack_setup(
    mut commands: Commands,
    mut query: Query<&mut Text, With<Announcement>>,
    player_stats: Query<&Stats, With<Player>>,
) {
    let mut announcement_text = query.single_mut();
    let player_stats = player_stats.single();

    announcement_text.sections[0].value =
        format!("You did {} damage to the enemy!", player_stats.attack);

    commands.insert_resource(Timer::from_seconds(TEXT_DURATION, false));
}

fn enemy_attack_setup(
    mut commands: Commands,
    mut query: Query<&mut Text, With<Announcement>>,
    enemy_stats: Query<&Stats, With<Enemy>>,
) {
    let mut announcement_text = query.single_mut();
    let enemy_stats = enemy_stats.single();

    announcement_text.sections[0].value = format!("They did {} damage to you!", enemy_stats.attack);

    commands.insert_resource(Timer::from_seconds(TEXT_DURATION, false));
}

fn battle_update(
    time: Res<Time>,
    mut timer: ResMut<Timer>,
    mut battle_state: ResMut<State<BattleState>>,
    mut game_state: ResMut<State<GameState>>,
) {
    // TODO: check win/lose
    if timer.tick(time.delta()).finished() {
        match battle_state.as_ref().current() {
            BattleState::Initialization => battle_state.set(BattleState::Idle).unwrap(),
            BattleState::PlayerAttack => {
                battle_state.set(BattleState::EnemyAttack).unwrap();
            }
            BattleState::EnemyAttack => {
                battle_state.set(BattleState::Idle).unwrap();
            }
            _ => (),
        }
    }
}

fn hide_player_action_container(
    children_query: Query<&Children>,
    mut visible_query: Query<&mut Visibility>,
    entity_vis: Query<Entity, With<PlayerActionContainer>>,
) {
    set_visible_recursive(
        false,
        entity_vis.single(),
        &mut visible_query,
        &children_query,
    );
}

fn show_player_action_container(
    children_query: Query<&Children>,
    mut visible_query: Query<&mut Visibility>,
    entity_vis: Query<Entity, With<PlayerActionContainer>>,
) {
    set_visible_recursive(
        true,
        entity_vis.single(),
        &mut visible_query,
        &children_query,
    );
}
