use crate::{despawn_screen, Enemy, EnemyTable, ImageAssets, Player};

use super::{GameState, BACKGROUND_SIZE};
use bevy::{math::const_vec2, prelude::*};

const TIME_STEP: f32 = 1.0 / 60.0;

const PLAYER_SPEED: f32 = 640.0;
const PLAYER_SPRINT: f32 = 1.5;
const PLAYER_SIZE: Vec2 = const_vec2!([64.0, 64.0]);

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Overworld).with_system(overworld_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Overworld)
                    .with_system(move_player)
                    .with_system(change_player_image)
                    .with_system(spawn_monster),
            )
            // When exiting the state, despawn everything that was spawned for this screen
            .add_system_set(
                SystemSet::on_exit(GameState::Overworld)
                    .with_system(despawn_screen::<OverworldScreen>),
            );
    }
}

#[derive(Component)]
struct OverworldScreen;

fn overworld_setup(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut player: ResMut<Player>,
) {
    commands.init_resource::<EnemyTable>();

    // Overworld
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                ..default()
            },
            texture: image_assets.overworld1.clone(),
            sprite: Sprite {
                custom_size: Some(BACKGROUND_SIZE),
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

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: ResMut<Player>,
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
    player: ResMut<Player>,
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
    mut game_state: ResMut<State<GameState>>,
    mut commands: Commands,
    enemy_table: Res<EnemyTable>,
    mut enemy: ResMut<Enemy>,
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        // TODO: random chance, area enemies
        let enemy_stats = enemy_table.table.get("Slime").unwrap().0.clone();
        let stats = enemy_table.table.get("Slime").unwrap().1.clone();

        enemy.entity = Some(commands.spawn().id());
        enemy.stats = stats;
        enemy.enemy_stats = enemy_stats;
        game_state.set(GameState::Battle).unwrap();
    }
}
