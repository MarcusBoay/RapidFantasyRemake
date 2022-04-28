mod mainmenu; // why does this work?????
mod overworld;
use bevy::{math::const_vec2, prelude::*, window::PresentMode};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    MainMenu,
    Overworld,
    Menu,
    Battle,
    Lose,
    FinalVictory,
    Exit,
}

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const BACKGROUND_SIZE: Vec2 = const_vec2!([1280., 720.]);
const BACKGROUND_COLOR: Color = Color::BLACK;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Rapid Fantasy - Remake".to_string(),
            width: 1280.0, // FIXME: this is causing the window to be fullscreen...
            height: 720.0,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup_main)
        .add_plugins(DefaultPlugins)
        .add_state(GameState::MainMenu)
        .add_plugin(mainmenu::MainMenuPlugin)
        .add_plugin(overworld::RapidFantasyPlugin)
        .run();
}

fn setup_main(mut commands: Commands) {
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
