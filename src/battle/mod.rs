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
struct HealthText;

#[derive(Component)]
struct HealthBar;

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
    player_stats: Query<&Stats, With<Player>>,
) {
    let player_stats = player_stats.single();
    let hp_perc = player_stats.hp as f32 / player_stats.hp_max as f32 * 100.;
    let mp_perc = player_stats.mp as f32 / player_stats.mp_max as f32 * 100.;

    // This will set BattleState::Initialization to BattleState::Idle in 1 second.
    commands.insert_resource(Timer::from_seconds(TEXT_DURATION, false));

    // TODO: use actual enemy stats
    commands.spawn().insert(Enemy).insert(Stats {
        hp: 1000,
        mp: 234,
        strength: 531,
        ..Stats::new()
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
                                    p.spawn_bundle(styled_player_hp_text(&font_assets, player_stats.hp, player_stats.hp_max))
                                        .insert(HealthText);
                                    p.spawn_bundle(styled_player_hp_bar_container())
                                        .with_children(|p| {
                                            p.spawn_bundle(styled_player_hp_bar(hp_perc))
                                                .insert(HealthBar);
                                        });
                                });

                            p.spawn_bundle(styled_player_stats_child_container())
                                .with_children(|p| {
                                    p.spawn_bundle(styled_player_mp_text(&font_assets, player_stats.mp, player_stats.mp_max));
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
    mut set: ParamSet<(
        Query<&Stats, With<Player>>,
        Query<&mut Stats, With<Enemy>>,
        Query<&mut Text, With<Announcement>>,
    )>
) {
    // TODO: update player MP if needed...
    // TODO: calculate damage
    let mut damage = 0;
    for player_stats in set.p0().iter() {
        damage = player_stats.strength;
    }

    for mut enemy_stats in set.p1().iter_mut() {
        enemy_stats.hp -= damage;
    }

    for mut announcement_text in set.p2().iter_mut() {
        announcement_text.sections[0].value =
            format!("You did {} damage to the enemy!", damage);
    }

    commands.insert_resource(Timer::from_seconds(TEXT_DURATION, false));
}

fn enemy_attack_setup(
    mut commands: Commands,
    mut set: ParamSet<(
        Query<&mut Stats, With<Player>>,
        Query<&Stats, With<Enemy>>,
        Query<&mut Text, With<Announcement>>,
        Query<&mut Text, With<HealthText>>,
        Query<&mut Style, With<HealthBar>>,
    )>
) {
    // TODO: calculate damage
    let mut damage = 0;
    for enemy_stats in set.p1().iter() {
        damage = enemy_stats.strength;
    }
    
    let mut player_hp = 0;
    let mut player_max_hp = 0;
    for mut player_stats in set.p0().iter_mut() {
        player_stats.hp -= damage;
        player_hp = player_stats.hp;
        player_max_hp = player_stats.hp_max;
    }

    for mut announcement_text in set.p2().iter_mut() {
        announcement_text.sections[0].value =
            format!("They did {} damage to you!", damage);
    }

    // TODO: put these into another system with Changed<> query filter... maybe
    for mut health_text in set.p3().iter_mut() {
        health_text.sections[1].value = format!("{} / {}", player_hp, player_max_hp);
    }

    let player_hp_perc = player_hp as f32 / player_max_hp as f32 * 100.;
    for mut health_bar in set.p4().iter_mut() {
        health_bar.size = Size::new(Val::Percent(player_hp_perc), Val::Percent(100.));
    }

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
