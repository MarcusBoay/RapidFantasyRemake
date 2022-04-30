use crate::{
    button_system, despawn_screen, set_visible_recursive, Enemy, FontAssets, Player, XP_TABLE,
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
            .add_system_set(SystemSet::on_enter(BattleState::Win).with_system(win_setup))
            .add_system_set(SystemSet::on_update(BattleState::Win).with_system(battle_update))
            .add_system_set(SystemSet::on_enter(BattleState::Lose).with_system(lose_setup))
            .add_system_set(SystemSet::on_update(BattleState::Lose).with_system(battle_update))
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
    Deinitialize,
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
struct HealthText;

#[derive(Component)]
struct HealthBar;

#[derive(Component)]
struct Announcement;

const TEXT_DURATION: f32 = 1.5;

fn battle_setup(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    player: Res<Player>,
    enemy: Res<Enemy>,
    mut battle_state: ResMut<State<BattleState>>,
) {
    let player = &player.stats;
    let hp_perc = player.hp as f32 / player.hp_max as f32 * 100.;
    let mp_perc = player.mp as f32 / player.mp_max as f32 * 100.;

    let enemy_sprite = enemy.stats.battle_sprite.clone();
    let enemy_name = enemy.enemy_stats.name.clone();

    // This will set BattleState::Initialization to BattleState::Idle in 1 second.
    commands.insert_resource(Timer::from_seconds(TEXT_DURATION, false));
    if *battle_state.current() == BattleState::Deinitialize {
        battle_state.set(BattleState::Initialization).unwrap();
    }

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
                                    p.spawn_bundle(styled_player_hp_text(
                                        &font_assets,
                                        player.hp,
                                        player.hp_max,
                                    ))
                                    .insert(HealthText);
                                    p.spawn_bundle(styled_player_hp_bar_container())
                                        .with_children(|p| {
                                            p.spawn_bundle(styled_player_hp_bar(hp_perc))
                                                .insert(HealthBar);
                                        });
                                });

                            p.spawn_bundle(styled_player_stats_child_container())
                                .with_children(|p| {
                                    p.spawn_bundle(styled_player_mp_text(
                                        &font_assets,
                                        player.mp,
                                        player.mp_max,
                                    ));
                                    p.spawn_bundle(styled_player_mp_bar_container())
                                        .with_children(|p| {
                                            p.spawn_bundle(styled_player_mp_bar(mp_perc));
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
                    p.spawn_bundle(styled_announcement_text(&font_assets, enemy_name))
                        .insert(Announcement);
                });

            p.spawn_bundle(styled_battle_images_container())
                .with_children(|p| {
                    p.spawn_bundle(styled_battle_portrait(player.battle_sprite.clone()));
                    p.spawn_bundle(styled_battle_portrait(enemy_sprite));
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
    mut announcement_text: Query<&mut Text, With<Announcement>>,
    player: Res<Player>,
    mut enemy: ResMut<Enemy>,
) {
    // TODO: update player MP if needed...
    // TODO: calculate damage
    let damage = player.stats.strength;

    enemy.stats.hp -= damage;
    println!("enemy hp: {}", enemy.stats.hp); // TODO: make enemy hp bar

    announcement_text.single_mut().sections[0].value =
        format!("You did {} damage to the enemy!", damage);

    commands.insert_resource(Timer::from_seconds(TEXT_DURATION, false));
}

fn enemy_attack_setup(
    mut commands: Commands,
    mut set: ParamSet<(
        Query<&mut Text, With<Announcement>>,
        Query<&mut Text, With<HealthText>>,
        Query<&mut Style, With<HealthBar>>,
    )>,
    mut player: ResMut<Player>,
    enemy: Res<Enemy>,
) {
    // TODO: calculate damage
    let damage = enemy.stats.strength;

    let mut player = &mut player.stats;
    player.hp -= damage;

    for mut announcement_text in set.p0().iter_mut() {
        announcement_text.sections[0].value = format!("They did {} damage to you!", damage);
    }

    // TODO: maybe put these into another system with Changed<> query filter...
    for mut health_text in set.p1().iter_mut() {
        health_text.sections[1].value = format!("{} / {}", player.hp, player.hp_max);
    }

    let player_hp_perc = player.hp as f32 / player.hp_max as f32 * 100.;
    for mut health_bar in set.p2().iter_mut() {
        health_bar.size = Size::new(Val::Percent(player_hp_perc), Val::Percent(100.));
    }

    commands.insert_resource(Timer::from_seconds(TEXT_DURATION, false));
}

fn win_setup(
    mut commands: Commands,
    mut set: ParamSet<(Query<&mut Text, With<Announcement>>,)>,
    mut player: ResMut<Player>,
    enemy: Res<Enemy>,
) {
    let mut player = &mut player.stats;
    let enemy_name = enemy.enemy_stats.name.clone();

    for mut announcement_text in set.p0().iter_mut() {
        announcement_text.sections[0].value = format!("You defeated {}!", enemy_name);
    }

    // Level up
    let mut leveled_up = false;
    player.experience += enemy.stats.experience;
    if player.level < 5 && player.experience >= XP_TABLE[player.level as usize - 1] {
        leveled_up = true;
        player.experience %= XP_TABLE[player.level as usize - 1];
        player.level += 1;

        player.hp_max += player.level * 50;
        player.hp = player.hp_max;
        player.strength += player.level * 5;
        player.wisdom += player.level * 5;
        player.mp_max += 40 + player.wisdom * 5;
        player.mp = player.mp_max;
    } else if player.level == 5 {
        player.experience = 1;
    }

    // TODO: gain loot

    commands.insert_resource(Timer::from_seconds(TEXT_DURATION, false));
}

fn lose_setup(
    mut commands: Commands,
    mut set: ParamSet<(Query<&mut Text, With<Announcement>>,)>,
    enemy: Res<Enemy>,
) {
    let enemy_name = enemy.enemy_stats.name.clone();

    for mut announcement_text in set.p0().iter_mut() {
        announcement_text.sections[0].value = format!("{} defeated you!", enemy_name);
    }

    commands.insert_resource(Timer::from_seconds(TEXT_DURATION, false));
}

fn battle_update(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<Timer>,
    mut battle_state: ResMut<State<BattleState>>,
    mut game_state: ResMut<State<GameState>>,
    enemy: Res<Enemy>,
    player: Res<Player>,
) {
    if timer.tick(time.delta()).finished() {
        match battle_state.as_ref().current() {
            BattleState::Initialization => battle_state.set(BattleState::Idle).unwrap(),
            BattleState::PlayerAttack => {
                if enemy.stats.hp <= 0 {
                    battle_state.set(BattleState::Win).unwrap();
                } else {
                    battle_state.set(BattleState::EnemyAttack).unwrap();
                }
            }
            BattleState::EnemyAttack => {
                // TODO: lose state
                if player.stats.hp <= 0 {
                    battle_state.set(BattleState::Lose).unwrap();
                } else {
                    battle_state.set(BattleState::Idle).unwrap();
                }
            }
            BattleState::Win => {
                battle_state.set(BattleState::Deinitialize).unwrap();
                game_state.set(GameState::Overworld).unwrap();
                // TODO: transition to final boss victory screen
            }
            BattleState::Lose => {
                battle_state.set(BattleState::Deinitialize).unwrap();
                game_state.set(GameState::Lose).unwrap();
            }
            BattleState::Deinitialize => {
                if let Some(e) = enemy.entity {
                    commands.entity(e).despawn_recursive();
                }
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
