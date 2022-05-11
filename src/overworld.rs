use crate::{despawn_screen, global, ImageAssets};

use bevy::{math::const_vec2, prelude::*};
use rand::{prelude::SliceRandom, thread_rng, Rng};

const TIME_STEP: f32 = 1.0 / 60.0;

const PLAYER_SPEED: f32 = 640.0;
const PLAYER_SPRINT: f32 = 1.5;
const PLAYER_SIZE: Vec2 = const_vec2!([64.0, 64.0]);

const MIN_ENEMY_SPAWN_STEPS: f32 = 64.;
const ENEMY_TRY_SPAWN_STEPS: f32 = 64.;
const ENEMY_SPAWN_CHANCE: usize = 10; // higher is lesser chance

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerSteps>()
            .add_system_set(
                SystemSet::on_enter(global::GameState::Overworld).with_system(overworld_setup),
            )
            .add_system_set(
                SystemSet::on_update(global::GameState::Overworld)
                    .with_system(move_player)
                    .with_system(change_player_image)
                    .with_system(spawn_monster)
                    .with_system(open_menu)
                    .with_system(go_to_area),
            )
            // When exiting the state, despawn everything that was spawned for this screen
            .add_system_set(
                SystemSet::on_exit(global::GameState::Overworld)
                    .with_system(despawn_screen::<OverworldScreen>),
            );
    }
}

#[derive(Component)]
struct OverworldScreen;

#[derive(Default, Deref)]
struct PlayerSteps(f32);

fn overworld_setup(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    areas: Res<global::Areas>,
    mut player: ResMut<global::Player>,
) {
    // Ensure open_menu() doesn't conflict with close_menu() from menu/mod.rs.
    commands.insert_resource(Timer::from_seconds(global::MENU_TOGGLE_DURATION, false));

    // Overworld
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                ..default()
            },
            texture: areas.get(&player.area).unwrap().background.clone(),
            sprite: Sprite {
                custom_size: Some(global::BACKGROUND_SIZE),
                ..default()
            },
            ..default()
        })
        .insert(OverworldScreen)
        .with_children(|p| {
            player.entity = Some(
                p.spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(player.x, player.y, 100.), // TODO: use player's last known coords
                        ..default()
                    },
                    texture: image_assets.player_down.clone(),
                    sprite: Sprite {
                        custom_size: Some(PLAYER_SIZE),
                        ..default()
                    },
                    ..default()
                })
                .id(),
            );
        });
}

fn go_to_area(
    mut background: Query<&mut Handle<Image>, With<OverworldScreen>>,
    mut transforms: Query<&mut Transform, Changed<Transform>>,
    mut player: ResMut<global::Player>,
    mut player_steps: ResMut<PlayerSteps>,
    areas: Res<global::Areas>,
) {
    // TODO: rename these. these are horribly named...
    fn is_below(player: &global::Player) -> bool {
        player.y < -(global::BACKGROUND_SIZE[1] / 2. + PLAYER_SIZE[1] / 4.)
    }
    fn is_above(player: &global::Player) -> bool {
        player.y > global::BACKGROUND_SIZE[1] / 2. + PLAYER_SIZE[1] / 4.
    }
    fn is_left(player: &global::Player) -> bool {
        player.x < -(global::BACKGROUND_SIZE[0] / 2. + PLAYER_SIZE[0] / 4.)
    }
    fn is_right(player: &global::Player) -> bool {
        player.x > global::BACKGROUND_SIZE[0] / 2. + PLAYER_SIZE[0] / 4.
    }
    fn get_new_below_y(player: &mut global::Player) {
        player.y = -player.y + PLAYER_SIZE[1] / 2.;
    }
    fn get_new_above_y(player: &mut global::Player) {
        player.y = -player.y - PLAYER_SIZE[1] / 2.;
    }
    fn get_new_left_x(player: &mut global::Player) {
        player.x = -player.x + PLAYER_SIZE[0] / 2.;
    }
    fn get_new_right_x(player: &mut global::Player) {
        player.x = -player.x - PLAYER_SIZE[0] / 2.;
    }

    let mut changed_area = true;
    match &player.area {
        0 if is_above(&player) => {
            get_new_below_y(&mut player);
            player.area = 1;
        }
        1 if is_below(&player) => {
            get_new_above_y(&mut player);
            player.area = 0;
        }
        1 if is_above(&player) => {
            get_new_below_y(&mut player);
            player.area = 2;
        }
        2 if is_below(&player) => {
            get_new_above_y(&mut player);
            player.area = 1;
        }
        2 if is_right(&player) => {
            get_new_left_x(&mut player);
            player.area = 3;
        }
        3 if is_left(&player) => {
            get_new_right_x(&mut player);
            player.area = 2;
        }
        3 if is_right(&player) => {
            get_new_left_x(&mut player);
            player.area = 4;
        }
        4 if is_left(&player) => {
            get_new_right_x(&mut player);
            player.area = 3;
        }
        4 if is_right(&player) => {
            get_new_left_x(&mut player);
            player.area = 5;
        }
        5 if is_left(&player) => {
            get_new_right_x(&mut player);
            player.area = 4;
        }
        _ => changed_area = false,
    }

    if changed_area {
        *background.single_mut() = areas.get(&player.area).unwrap().background.clone();
        let player_transform = &mut transforms.get_mut(player.entity.unwrap()).unwrap();
        player_transform.translation.x = player.x;
        player_transform.translation.y = player.y;

        player_steps.0 = 0.;
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: ResMut<global::Player>,
    mut transforms: Query<&mut Transform>,
    mut player_steps: ResMut<PlayerSteps>,
) {
    let player_transform = &mut transforms.get_mut(player.entity.unwrap()).unwrap();
    let mut direction_horizontal = 0.0;
    let mut direction_vertical = 0.0;

    // Only mono-directional movement allowed.
    if keyboard_input.pressed(KeyCode::Left) {
        direction_horizontal -= 1.0;
    } else if keyboard_input.pressed(KeyCode::Right) {
        direction_horizontal += 1.0;
    } else if keyboard_input.pressed(KeyCode::Up) {
        direction_vertical += 1.0;
    } else if keyboard_input.pressed(KeyCode::Down) {
        direction_vertical -= 1.0;
    }

    // Sprinting.
    if keyboard_input.pressed(KeyCode::LShift) {
        direction_horizontal *= PLAYER_SPRINT;
        direction_vertical *= PLAYER_SPRINT;
    }

    let steps_horizontal = direction_horizontal * PLAYER_SPEED * TIME_STEP;
    let steps_vertical = direction_vertical * PLAYER_SPEED * TIME_STEP;

    let new_player_position_x = player_transform.translation.x + steps_horizontal;
    let new_player_position_y = player_transform.translation.y + steps_vertical;

    player.x = new_player_position_x;
    player.y = new_player_position_y;

    // TODO: clamp within map area
    player_transform.translation.x = new_player_position_x;
    player_transform.translation.y = new_player_position_y;

    player_steps.0 += steps_horizontal.abs() + steps_vertical.abs();
}

fn change_player_image(
    keyboard_input: Res<Input<KeyCode>>,
    image_assets: Res<ImageAssets>,
    player: ResMut<global::Player>,
    mut image: Query<&mut Handle<Image>>,
) {
    let new_player_image = if keyboard_input.pressed(KeyCode::Left) {
        Some(image_assets.player_left.clone())
    } else if keyboard_input.pressed(KeyCode::Right) {
        Some(image_assets.player_right.clone())
    } else if keyboard_input.pressed(KeyCode::Up) {
        Some(image_assets.player_up.clone())
    } else if keyboard_input.pressed(KeyCode::Down) {
        Some(image_assets.player_down.clone())
    } else {
        // Don't change sprite if no input.
        None
    };

    if let Some(new_player_image) = new_player_image {
        *image.get_mut(player.entity.unwrap()).unwrap() = new_player_image;
    }
}

// TODO: spawn final boss monster during interaction
fn spawn_monster(
    mut player_steps: ResMut<PlayerSteps>,
    mut game_state: ResMut<State<global::GameState>>,
    mut commands: Commands,
    enemy_table: Res<global::EnemyTable>,
    mut enemy: ResMut<global::Enemy>,
    areas: Res<global::Areas>,
    player: Res<global::Player>,
) {
    let area_enemies = &areas.get(&player.area).unwrap().enemies;
    if area_enemies.len() == 0 {
        // Length 0 means the area is a safe area.
        player_steps.0 = 0.;
        return;
    }

    if player_steps.0 > MIN_ENEMY_SPAWN_STEPS + ENEMY_TRY_SPAWN_STEPS {
        let should_spawn_enemy_roll = thread_rng().gen_range(0..ENEMY_SPAWN_CHANCE) == 0;
        if should_spawn_enemy_roll {
            let enemy_id_roll = area_enemies.choose(&mut rand::thread_rng()).unwrap();
            let spawned_enemy = enemy_table.table.get(enemy_id_roll).unwrap();
            let (enemy_stats, stats, attacks, loot_table) = (
                spawned_enemy.0.clone(),
                spawned_enemy.1.clone(),
                spawned_enemy.2.clone(),
                spawned_enemy.3.clone(),
            );

            enemy.entity = Some(commands.spawn().id());
            enemy.stats = stats;
            enemy.enemy_stats = enemy_stats;
            enemy.attacks = attacks;
            enemy.loot_table = loot_table;
            game_state.set(global::GameState::Battle).unwrap();

            player_steps.0 = 0.;
        } else {
            player_steps.0 = MIN_ENEMY_SPAWN_STEPS;
        }
    }
}

fn open_menu(
    time: Res<Time>,
    mut timer: ResMut<Timer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<global::GameState>>,
) {
    if timer.tick(time.delta()).finished() && keyboard_input.just_pressed(KeyCode::P) {
        game_state.set(global::GameState::Menu).unwrap();
    }
}
