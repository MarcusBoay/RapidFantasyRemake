use crate::{despawn_screen, global, ImageAssets};

use bevy::{math::const_vec2, prelude::*};

const TIME_STEP: f32 = 1.0 / 60.0;

const PLAYER_SPEED: f32 = 640.0;
const PLAYER_SPRINT: f32 = 1.5;
const PLAYER_SIZE: Vec2 = const_vec2!([64.0, 64.0]);

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(global::GameState::Overworld).with_system(overworld_setup),
        )
        .add_system_set(
            SystemSet::on_update(global::GameState::Overworld)
                .with_system(move_player)
                .with_system(change_player_image)
                .with_system(spawn_monster)
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

fn overworld_setup(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut player: ResMut<global::Player>,
) {
    // Overworld
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                ..default()
            },
            texture: get_overworld_background(&player, &image_assets),
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

fn get_overworld_background(
    player: &global::Player,
    image_assets: &Res<ImageAssets>,
) -> Handle<Image> {
    match player.area {
        0 => image_assets.area0.clone(),
        1 => image_assets.area1.clone(),
        2 => image_assets.area2.clone(),
        3 => image_assets.area3.clone(),
        4 => image_assets.area4.clone(),
        5 => image_assets.area5.clone(),
        _ => unreachable!(),
    }
}

fn go_to_area(
    mut background: Query<&mut Handle<Image>, With<OverworldScreen>>,
    mut transforms: Query<&mut Transform, Changed<Transform>>,
    mut player: ResMut<global::Player>,
    image_assets: Res<ImageAssets>,
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
        *background.single_mut() = get_overworld_background(&player, &image_assets);
        let player_transform = &mut transforms.get_mut(player.entity.unwrap()).unwrap();
        player_transform.translation.x = player.x;
        player_transform.translation.y = player.y;
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: ResMut<global::Player>,
    mut transforms: Query<&mut Transform>,
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

    let new_player_position_x =
        player_transform.translation.x + direction_horizontal * PLAYER_SPEED * TIME_STEP;
    let new_player_position_y =
        player_transform.translation.y + direction_vertical * PLAYER_SPEED * TIME_STEP;

    player.x = new_player_position_x;
    player.y = new_player_position_y;

    // TODO: clamp within map area
    player_transform.translation.x = new_player_position_x;
    player_transform.translation.y = new_player_position_y;
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

// TODO: change to random chance to spawn
// TODO: spawn final boss monster during interaction
fn spawn_monster(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<global::GameState>>,
    mut commands: Commands,
    enemy_table: Res<global::EnemyTable>,
    mut enemy: ResMut<global::Enemy>,
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        // TODO: random chance, area enemies
        let spawned_enemy = enemy_table.table.get(&0).unwrap();
        let (enemy_stats, stats, attacks) = (
            spawned_enemy.0.clone(),
            spawned_enemy.1.clone(),
            spawned_enemy.2.clone(),
        );

        enemy.entity = Some(commands.spawn().id());
        enemy.stats = stats;
        enemy.enemy_stats = enemy_stats;
        enemy.attacks = attacks;
        game_state.set(global::GameState::Battle).unwrap();
    }
}
