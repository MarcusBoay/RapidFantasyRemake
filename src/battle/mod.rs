use crate::{button_system, despawn_children, despawn_screen, global, FontAssets};

mod styles;
use queues::*;
pub use styles::*;

use bevy::prelude::*;
use rand::prelude::*;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(BattleState::Initialization)
            .add_state(ActionMenuState::Inactive)
            .add_state(MagicMenuState::Inactive)
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
                    .with_system(button_system)
                    .with_system(action_menu_button_action)
                    .with_system(magic_menu_button_action),
            )
            .add_system_set(
                SystemSet::on_exit(BattleState::Idle).with_system(deactivate_player_menus),
            )
            .add_system_set(
                SystemSet::on_enter(ActionMenuState::Active).with_system(spawn_action_menu),
            )
            .add_system_set(
                SystemSet::on_exit(ActionMenuState::Active)
                    .with_system(despawn_children::<ActionMenu>),
            )
            .add_system_set(
                SystemSet::on_enter(MagicMenuState::Active).with_system(spawn_magic_menu),
            )
            .add_system_set(
                SystemSet::on_exit(MagicMenuState::Active)
                    .with_system(despawn_children::<MagicMenu>),
            )
            .add_system_set(
                SystemSet::on_enter(BattleState::PlayerAction).with_system(player_attack_setup),
            )
            .add_system_set(
                SystemSet::on_update(BattleState::PlayerAction).with_system(battle_update),
            )
            .add_system_set(
                SystemSet::on_enter(BattleState::EnemyAction).with_system(enemy_attack_setup),
            )
            .add_system_set(
                SystemSet::on_update(BattleState::EnemyAction).with_system(battle_update),
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
    PlayerAction,
    EnemyAction,
    Win,
    Lose,
    Deinitialize,
    // maybe magic menu state?
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
struct ActionMenu;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum ActionMenuState {
    Active,
    Inactive,
}

#[derive(Component, Debug, Clone)]
pub enum PlayerButtonAction {
    Attack,
    LimitBreak,
    Magic,
    Block,
    Item,
}

#[derive(Component)]
struct MagicMenu;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum MagicMenuState {
    Active,
    Inactive,
}

#[derive(Component)]
struct EnemyHPBar;

#[derive(Component)]
struct HealthText;

#[derive(Component)]
struct HealthBar;

#[derive(Component)]
struct ManaText;

#[derive(Component)]
struct ManaBar;

#[derive(Component)]
struct LimitBar;

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
    let hp_perc = player.stats.hp as f32 / player.stats.hp_max as f32 * 100.;
    let mp_perc = player.stats.mp as f32 / player.stats.mp_max as f32 * 100.;

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
                                        player.stats.hp,
                                        player.stats.hp_max,
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
                                        player.stats.mp,
                                        player.stats.mp_max,
                                    ))
                                    .insert(ManaText);
                                    p.spawn_bundle(styled_player_mp_bar_container())
                                        .with_children(|p| {
                                            p.spawn_bundle(styled_player_mp_bar(mp_perc))
                                                .insert(ManaBar);
                                        });
                                });

                            p.spawn_bundle(styled_player_stats_child_container())
                                .with_children(|p| {
                                    p.spawn_bundle(styled_player_limit_break_text(&font_assets));
                                    p.spawn_bundle(styled_player_limit_break_bar_container())
                                        .with_children(|p| {
                                            p.spawn_bundle(styled_player_limit_break_bar(
                                                player.limit,
                                            ))
                                            .insert(LimitBar);
                                        });
                                });
                        });

                    p.spawn_bundle(styled_player_action_container())
                        .insert(PlayerActionContainer)
                        .with_children(|p| {
                            p.spawn_bundle(styled_player_action_button_container())
                                .insert(ActionMenu);
                            p.spawn_bundle(styled_player_magic_menu_container())
                                .insert(MagicMenu);
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
                    p.spawn_bundle(styled_battle_portrait(player.stats.battle_sprite.clone()));
                    p.spawn_bundle(styled_enemy_portrait_container())
                        .with_children(|p| {
                            p.spawn_bundle(styled_battle_portrait(enemy_sprite));
                            p.spawn_bundle(styled_enemy_hp_bar()).insert(EnemyHPBar);
                        });
                });
        });
}

fn spawn_action_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    player: Res<global::Player>,
    action_menu: Query<Entity, With<ActionMenu>>,
) {
    commands.entity(action_menu.single()).with_children(|p| {
        for player_button_action in [
            if player.limit < 100 {
                PlayerButtonAction::Attack
            } else {
                PlayerButtonAction::LimitBreak
            },
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
}

fn spawn_magic_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    magic_menu: Query<Entity, With<MagicMenu>>,
    magic_equipped: Res<global::PlayerMagicEquipped>,
) {
    commands.entity(magic_menu.single()).with_children(|p| {
        for magic in magic_equipped.equipped.iter() {
            if let Some(magic) = &magic {
                p.spawn_bundle(styled_player_action_button())
                    .insert(magic.clone())
                    .with_children(|p| {
                        p.spawn_bundle(styled_player_magic_button_text(
                            &magic.name[..],
                            &font_assets,
                        ));
                    });
            } else {
                p.spawn_bundle(styled_player_action_button())
                    .with_children(|p| {
                        p.spawn_bundle(styled_player_magic_button_text("-", &font_assets));
                    });
            }
        }
    });
}

fn deactivate_player_menus(
    mut action_menu_state: ResMut<State<ActionMenuState>>,
    mut magic_menu_state: ResMut<State<MagicMenuState>>,
) {
    if *magic_menu_state.current() == MagicMenuState::Active {
        magic_menu_state.set(MagicMenuState::Inactive).unwrap();
    }
    if *action_menu_state.current() == ActionMenuState::Active {
        action_menu_state.set(ActionMenuState::Inactive).unwrap();
    }
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
    mut action_menu_state: ResMut<State<ActionMenuState>>,
) {
    *announcement_text
        .get_mut(announcement.entity.unwrap())
        .unwrap() = Text::with_section("", common_text_style(&font_assets), Default::default());

    // Show action menu.
    action_menu_state.set(ActionMenuState::Active).unwrap();

    // Reset player battle actions.
    player_battle_action.attack = None;
    player_battle_action.block = false;
    // player_battle_action.item = None;
}

fn action_menu_button_action(
    interaction_query: Query<
        (&Interaction, &PlayerButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut battle_state: ResMut<State<BattleState>>,
    mut magic_menu_state: ResMut<State<MagicMenuState>>,
    mut player_battle_action: ResMut<PlayerBattleAction>,
    player_limit: Res<global::PlayerLimitEquipped>,
    player_attack_table: Res<global::PlayerAttackTable>,
) {
    for (interaction, menu_button_action) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                PlayerButtonAction::Attack => {
                    battle_state.set(BattleState::PlayerAction).unwrap();
                    player_battle_action.attack =
                        Some(player_attack_table.table.get(&0).unwrap().clone())
                }
                PlayerButtonAction::LimitBreak => {
                    battle_state.set(BattleState::PlayerAction).unwrap();
                    player_battle_action.attack = Some(player_limit.equipped.clone());
                }
                PlayerButtonAction::Block => {
                    // TODO: block & use item
                    battle_state.set(BattleState::PlayerAction).unwrap();
                    player_battle_action.block = true;
                }
                PlayerButtonAction::Magic => {
                    // Switch magic menu state.
                    match magic_menu_state.current() {
                        MagicMenuState::Active => {
                            magic_menu_state.set(MagicMenuState::Inactive).unwrap()
                        }
                        MagicMenuState::Inactive => {
                            magic_menu_state.set(MagicMenuState::Active).unwrap()
                        }
                    }
                }
                _ => todo!("Unhandled player action button!!"),
            }
        }
    }
}

fn magic_menu_button_action(
    interaction_query: Query<
        (&Interaction, &global::PlayerAttack),
        (Changed<Interaction>, With<Button>),
    >,
    mut battle_state: ResMut<State<BattleState>>,
    mut player_battle_action: ResMut<PlayerBattleAction>,
) {
    for (interaction, menu_button_action) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            battle_state.set(BattleState::PlayerAction).unwrap();
            player_battle_action.attack = Some(menu_button_action.clone());
        }
    }
}

fn player_attack_setup(
    mut set: ParamSet<(
        Query<&mut Text, With<ManaText>>,
        Query<&mut Style, With<ManaBar>>,
        Query<(&mut Style, &mut UiColor), With<LimitBar>>,
        Query<&mut Style, With<EnemyHPBar>>,
    )>,
    mut announcement: ResMut<Announcement>,
    mut player: ResMut<global::Player>,
    mut enemy: ResMut<global::Enemy>,
    player_action: Res<PlayerBattleAction>,
) {
    // TODO: ensure there is mp to use magic...
    if let Some(attack) = &player_action.attack {
        let damage = calculate_player_attack_damage(&attack, &player, &enemy);

        enemy.stats.hp = std::cmp::min(
            std::cmp::max(0, enemy.stats.hp - damage),
            enemy.stats.hp_max,
        );

        if let Some(global::PlayerAttackType::Limit) = &attack.attack_type {
            player.limit = 0;
            for (mut limit_bar, mut color) in set.p2().iter_mut() {
                limit_bar.size.width = Val::Percent(0.);
                *color = Color::ORANGE.into();
            }
        }

        player.stats.mp -= attack.mp_use;

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

        let player = &player.stats;
        for mut mp_text in set.p0().iter_mut() {
            mp_text.sections[1].value = format!("{} / {}", player.mp, player.mp_max);
        }

        let player_mp_perc = player.mp as f32 / player.mp_max as f32 * 100.;
        for mut mp_bar in set.p1().iter_mut() {
            mp_bar.size.width = Val::Percent(player_mp_perc);
        }

        let enemy_hp_perc = enemy.stats.hp as f32 / enemy.stats.hp_max as f32 * 256.;
        for mut enemy_hp_bar in set.p3().iter_mut() {
            enemy_hp_bar.size.width = Val::Px(enemy_hp_perc);
        }
    }

    if player_action.block {
        let _ = announcement
            .texts
            .add(format!("You blocked their next attack."));
    }
    // TODO: handle use item
}

fn calculate_player_attack_damage(
    attack: &global::PlayerAttack,
    player: &global::Player,
    enemy: &global::Enemy,
) -> i32 {
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

    // Apply elemental modifier.
    power = power * element_modifier(&attack.element, &enemy.enemy_stats.element);

    // Get damage reduction.
    let mut damage_reduction =
        enemy.stats.defense as f32 + enemy.stats.defense as f32 / 300. * power;
    if let Some(global::PlayerAttackType::Magic) = attack.attack_type {
        damage_reduction *= 0.2;
    }

    (power - damage_reduction).round() as i32
}

fn element_modifier(attack: &Option<global::Element>, receiver: &Option<global::Element>) -> f32 {
    if attack.is_none() || receiver.is_none() {
        return 1.0;
    }

    let (attack, receiver) = (attack.clone().unwrap(), receiver.clone().unwrap());
    use global::Element::*;
    match (&attack, &receiver) {
        (Water, Fire)
        | (Fire, Earth)
        | (Earth, Electric)
        | (Electric, Water)
        | (Dark, Light)
        | (Light, Dark) => 2.0,
        (Fire, Water) | (Earth, Fire) | (Electric, Earth) | (Water, Electric) => 0.5,
        _ if &attack == &receiver => -1.0,
        _ => 1.0,
    }
}

fn enemy_attack_setup(
    mut set: ParamSet<(
        Query<&mut Text, With<HealthText>>,
        Query<&mut Style, With<HealthBar>>,
        Query<(&mut Style, &mut UiColor), With<LimitBar>>,
    )>,
    mut announcement: ResMut<Announcement>,
    mut player: ResMut<global::Player>,
    mut enemy: ResMut<global::Enemy>,
    player_action: Res<PlayerBattleAction>,
) {
    // Pick attack as long as there is mp available.
    let mut rng = thread_rng();
    let mut attack_index = rng.gen_range(0..enemy.attacks.len());
    while enemy.attacks[attack_index].mp_use > enemy.stats.mp {
        attack_index = rng.gen_range(0..enemy.attacks.len());
    }
    let attack = enemy.attacks[attack_index].clone();

    let damage = calculate_enemy_attack_damage(&attack, &enemy, &player, player_action.block);

    let limit_addition = (200. * (damage as f32) / (player.stats.hp_max as f32)) as f32;
    let new_limit = player.limit as f32 + limit_addition;
    player.limit = if new_limit < 100. {
        new_limit as u8
    } else {
        100
    };

    player.stats.hp = std::cmp::min(
        std::cmp::max(0, player.stats.hp - damage),
        player.stats.hp_max,
    );

    enemy.stats.mp -= attack.mp_use;

    let _ = announcement.texts.add(format!(
        "{} used {}, dealing {} damage to you!",
        enemy.enemy_stats.name, attack.name, damage
    ));

    // TODO: maybe put these into another system with Changed<> query filter...
    for mut health_text in set.p0().iter_mut() {
        health_text.sections[1].value = format!("{} / {}", player.stats.hp, player.stats.hp_max);
    }

    let player_hp_perc = player.stats.hp as f32 / player.stats.hp_max as f32 * 100.;
    for mut health_bar in set.p1().iter_mut() {
        health_bar.size.width = Val::Percent(player_hp_perc);
    }

    for (mut limit, mut color) in set.p2().iter_mut() {
        limit.size.width = Val::Percent(player.limit as f32);
        if player.limit == 100 {
            *color = Color::RED.into();
        }
    }
}

fn calculate_enemy_attack_damage(
    attack: &global::EnemyAttack,
    enemy: &global::Enemy,
    player: &global::Player,
    player_block: bool,
) -> i32 {
    // Get attack power.
    let power = attack.damage_modifier
        * if let Some(global::EnemyAttackType::Magic) = attack.attack_type {
            enemy.stats.wisdom as f32
        } else if let Some(global::EnemyAttackType::Percentile) = attack.attack_type {
            player.stats.hp_max as f32
        } else {
            // Physical attack.
            enemy.stats.strength as f32
        };

    // Get damage reduction.
    let mut damage_reduction =
        player.stats.defense as f32 + player.stats.defense as f32 / 300. * power;
    if let Some(global::EnemyAttackType::Magic) = attack.attack_type {
        damage_reduction *= 0.2;
    }
    if player_block {
        damage_reduction *= 2.;
    }

    // enemy damage cannot go below 0
    std::cmp::max(0, (power - damage_reduction).round() as i32)
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
                BattleState::PlayerAction => {
                    if enemy.stats.hp <= 0 {
                        battle_state.set(BattleState::Win).unwrap();
                    } else {
                        battle_state.set(BattleState::EnemyAction).unwrap();
                    }
                }
                BattleState::EnemyAction => {
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
