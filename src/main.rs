use bevy::{
    prelude::*,
    math::const_vec3,
    render::render_resource::Texture
};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Overworld,
    Exit,
}

const TIME_STEP: f32 = 1.0 / 60.0;

// TODO: replace with map image
const TEMP_BACKGROUND_COLOR: Color = Color::WHITE;
// TODO: replace with player image
const TEMP_PLAYER_COLOR: Color = Color::PINK;
const PLAYER_SIZE: Vec3 = const_vec3!([1.0, 1.0, 0.0]);
const PLAYER_SPEED: f32 = 640.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapidFantasyPlugin)
        .run();
}

pub struct RapidFantasyPlugin;

impl Plugin for RapidFantasyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system_set(
                SystemSet::new()
                    .with_system(move_player)
                    .with_system(change_player_image)
            )
            // .add_system(move_player);
            ;
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Player
    let player_image = asset_server.load("player_down.png");
    commands.spawn()
        .insert(Player)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 50.0, 0.0),
                scale: PLAYER_SIZE,
                ..default()
            },
            texture: player_image,
            sprite: Sprite {
                color: TEMP_PLAYER_COLOR,
                ..default()
            },
            ..default()
        });
}

#[derive(Component)]
struct Player;

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>
) {
    let mut player_transform = query.single_mut();
    let mut direction_horizontal = 0.0;
    let mut direction_vertical = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction_horizontal -= 1.0;
    } else if keyboard_input.pressed(KeyCode::Right) {
        direction_horizontal += 1.0;
    } else if keyboard_input.pressed(KeyCode::Up) {
        direction_vertical += 1.0;
    } else if keyboard_input.pressed(KeyCode::Down) {
        direction_vertical -= 1.0;
    }

    let new_player_position_x = player_transform.translation.x + direction_horizontal * PLAYER_SPEED * TIME_STEP;
    let new_player_position_y = player_transform.translation.y + direction_vertical * PLAYER_SPEED * TIME_STEP;

    // TODO: clamp within map area
    player_transform.translation.x = new_player_position_x;
    player_transform.translation.y = new_player_position_y;
}

fn change_player_image(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Handle<Image>, With<Player>>,
    asset_server: Res<AssetServer>
) {
    let mut player_image = query.single_mut();
    let new_player_image = 
        if keyboard_input.pressed(KeyCode::Left) {
            Some(asset_server.load("player_left.png"))
        } else if keyboard_input.pressed(KeyCode::Right) {
            Some(asset_server.load("player_right.png"))
        } else if keyboard_input.pressed(KeyCode::Up) {
            Some(asset_server.load("player_up.png"))
        } else if keyboard_input.pressed(KeyCode::Down) {
            Some(asset_server.load("player_down.png"))
        } else { // Don't change sprite if no input.
            None
        };

    if let Some(new_player_image) = new_player_image {
        *player_image = new_player_image;
    }
}