use crate::{button_system, despawn_screen, global, set_visible_recursive, FontAssets};

mod styles;
use queues::*;
pub use styles::*;

use bevy::prelude::*;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(BattleState::Initialization)
            .init_resource::<Announcement>()
            .init_resource::<PlayerBattleAction>()
            .add_system_set(
                SystemSet::on_enter(global::GameState::Battle).with_system(battle_setup),
            )
            .add_system_set(
                SystemSet::on_update(global::GameState::Battle).with_system(battle_init),
            )
            .add_system_set(SystemSet::on_enter(BattleState::Idle).with_system(idle_init))
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
                SystemSet::on_exit(global::GameState::Battle)
                    .with_system(despawn_screen::<BattleScreen>),
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

#[derive(Default)]
struct PlayerBattleAction {
    attack: Option<global::PlayerAttack>,
    block: bool,
    // item: Option<Item>
}

#[derive(Component)]
struct PlayerActionContainer;

#[derive(Component)]
struct HealthText;

#[derive(Component)]
struct HealthBar;

#[derive(Default)]
struct Announcement {
    entity: Option<Entity>,
    texts: Queue<String>,
}

const TEXT_DURATION: f32 = 2.;

fn battle_setup(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    player: Res<global::Player>,
    enemy: Res<global::Enemy>,
    mut battle_state: ResMut<State<BattleState>>,
    mut announcement: ResMut<Announcement>,
) {
    let player = &player.stats;
    let hp_perc = player.hp as f32 / player.hp_max as f32 * 100.;
    let mp_perc = player.mp as f32 / player.mp_max as f32 * 100.;

    let enemy_sprite = enemy.stats.battle_sprite.clone();
    let enemy_name = enemy.enemy_stats.name.clone();

    // This will set BattleState::Initialization to BattleState::Idle in 1 second.
    commands.insert_resource(Timer::from_seconds(TEXT_DURATION, false));
    // Repeating battles will start on BattleState::Deinitialize... Reset it.
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
                    announcement.entity = Some(
                        p.spawn_bundle(styled_announcement_text(&font_assets, enemy_name))
                            .id(),
                    );
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

fn idle_init(
    font_assets: Res<FontAssets>,
    announcement: Res<Announcement>,
    mut announcement_text: Query<&mut Text>,
    mut player_battle_action: ResMut<PlayerBattleAction>,
) {
    *announcement_text
        .get_mut(announcement.entity.unwrap())
        .unwrap() = Text::with_section("", common_text_style(&font_assets), Default::default());

    // Reset player battle actions.
    player_battle_action.attack = None;
    player_battle_action.block = false;
    // player_battle_action.item = None;

    // TODO: change text 'Attack' to 'Limit Break'
}

fn battle_action(
    interaction_query: Query<
        (&Interaction, &PlayerButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut battle_state: ResMut<State<BattleState>>,
    mut player_battle_action: ResMut<PlayerBattleAction>,
) {
    for (interaction, menu_button_action) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                PlayerButtonAction::Attack => {
                    battle_state.set(BattleState::PlayerAttack).unwrap();

                    // TODO: switch between attack and limit break
                    // TODO: get attack from player attack inventory...
                    player_battle_action.attack = Some(global::PlayerAttack {
                        name: "Tackle".to_string(),
                        attack_type: None,
                        element: None,
                        mp_use: 0,
                        tier: 1,
                    });
                }
                _ => unimplemented!("Unhandled player action button!!"), // TODO
            }
        }
    }
}

fn player_attack_setup(
    mut announcement: ResMut<Announcement>,
    player: Res<global::Player>,
    mut enemy: ResMut<global::Enemy>,
    player_action: Res<PlayerBattleAction>,
) {
    if let Some(attack) = &player_action.attack {
        // Get attack power.
        let mut power = if let Some(global::PlayerAttackType::Limit) = attack.attack_type {
            match attack.tier {
                3 => 120. + 2.5 * player.stats.strength as f32,
                2 => 80. + 2.0 * player.stats.strength as f32,
                1 | _ => 40. + 1.5 * player.stats.strength as f32,
            }
        } else if let Some(global::PlayerAttackType::Magic) = attack.attack_type {
            match attack.tier {
                3 => 45. + 1.2 * player.stats.wisdom as f32,
                2 => 15. + 1.0 * player.stats.wisdom as f32,
                1 | _ => 5. + 0.8 * player.stats.wisdom as f32,
            }
        } else {
            // Normal attack
            1.5 * player.stats.strength as f32
        };

        // TODO: elemental modifier
        // Apply elemental modifier.
        // power = power * elemental_modifier(attack_element, enemy.enemy_stats.element);

        // Get damage reduction.
        let mut damage_reduction =
            enemy.stats.defense as f32 + enemy.stats.defense as f32 / 300. * power as f32;
        if let Some(global::PlayerAttackType::Magic) = attack.attack_type {
            damage_reduction *= 0.2;
        }

        let damage = (power - damage_reduction).round() as i32;

        enemy.stats.hp = std::cmp::min(
            std::cmp::max(0, enemy.stats.hp - damage),
            enemy.stats.hp_max,
        );

        // TODO: check which type of attack is used
        // TODO: deduct limit
        // TODO: deduct magic use
        // TODO: ensure there is mp to use magic...

        println!("enemy hp: {}", enemy.stats.hp); // TODO: make enemy hp bar

        let announcement_text = if damage >= 0 {
            format!("You used {}, dealing {} damage!", attack.name, damage)
        } else {
            format!(
                "You used {}, healing {} to the enemy!",
                attack.name,
                damage * -1
            )
        };

        let _ = announcement.texts.add(announcement_text);
    }

    // TODO: handle block
    // TODO: handle use item
}

fn enemy_attack_setup(
    mut set: ParamSet<(
        Query<&mut Text, With<HealthText>>,
        Query<&mut Style, With<HealthBar>>,
    )>,
    mut announcement: ResMut<Announcement>,
    mut player: ResMut<global::Player>,
    enemy: Res<global::Enemy>,
) {
    // TODO: calculate damage
    let damage = enemy.stats.strength;

    let mut player = &mut player.stats;
    player.hp -= damage;

    let _ = announcement
        .texts
        .add(format!("They did {} damage to you!", damage));

    // TODO: maybe put these into another system with Changed<> query filter...
    for mut health_text in set.p0().iter_mut() {
        health_text.sections[1].value = format!("{} / {}", player.hp, player.hp_max);
    }

    let player_hp_perc = player.hp as f32 / player.hp_max as f32 * 100.;
    for mut health_bar in set.p1().iter_mut() {
        health_bar.size = Size::new(Val::Percent(player_hp_perc), Val::Percent(100.));
    }
}

fn win_setup(
    mut announcement: ResMut<Announcement>,
    mut player: ResMut<global::Player>,
    enemy: Res<global::Enemy>,
) {
    let mut player = &mut player.stats;
    let enemy_name = enemy.enemy_stats.name.clone();

    let _ = announcement
        .texts
        .add(format!("You defeated {}!", enemy_name));

    // Level up
    player.experience += enemy.stats.experience;
    let _ = announcement
        .texts
        .add(format!("You gained {} experience!", enemy.stats.experience));
    if player.level < 5 && player.experience >= global::XP_TABLE[player.level as usize - 1] {
        player.experience %= global::XP_TABLE[player.level as usize - 1];
        player.level += 1;

        player.hp_max += player.level * 50;
        player.hp = player.hp_max;
        player.strength += player.level * 5;
        player.wisdom += player.level * 5;
        player.mp_max += 40 + player.wisdom * 5;
        player.mp = player.mp_max;

        let _ = announcement.texts.add(format!("You leveled up!"));
    } else if player.level == 5 {
        player.experience = 1;
    }

    // TODO: gain loot
}

fn lose_setup(mut announcement: ResMut<Announcement>, enemy: Res<global::Enemy>) {
    let enemy_name = enemy.enemy_stats.name.clone();
    let _ = announcement
        .texts
        .add(format!("{} defeated you!", enemy_name));
}

fn battle_update(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<Timer>,
    mut battle_state: ResMut<State<BattleState>>,
    mut game_state: ResMut<State<global::GameState>>,
    mut announcement: ResMut<Announcement>,
    mut announcement_text: Query<&mut Text>,
    font_assets: Res<FontAssets>,
    enemy: Res<global::Enemy>,
    player: Res<global::Player>,
) {
    if timer.tick(time.delta()).finished() {
        if let Ok(text) = announcement.texts.remove() {
            *announcement_text
                .get_mut(announcement.entity.unwrap())
                .unwrap() =
                Text::with_section(text, common_text_style(&font_assets), Default::default());

            commands.insert_resource(Timer::from_seconds(TEXT_DURATION, false));
        } else {
            match battle_state.as_ref().current() {
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
                    game_state.set(global::GameState::Overworld).unwrap();
                    // TODO: transition to final boss victory screen
                }
                BattleState::Lose => {
                    battle_state.set(BattleState::Deinitialize).unwrap();
                    game_state.set(global::GameState::Lose).unwrap();
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
