mod mainmenu; // why does this work?????
mod overworld;
use bevy::{math::const_vec2, prelude::*, window::PresentMode};
use bevy_asset_loader::{AssetCollection, AssetLoader};

fn main() {
    let mut app = App::new();
    AssetLoader::new(GameState::Initialization)
        .continue_to_state(GameState::MainMenu)
        .with_collection::<ImageAssets>()
        .with_collection::<FontAssets>()
        .build(&mut app);
    app.insert_resource(WindowDescriptor {
        title: "Rapid Fantasy - Remake".to_string(),
        width: 1280.0, // FIXME: this is causing the window to be fullscreen...
        height: 720.0,
        present_mode: PresentMode::Fifo,
        ..default()
    })
    .insert_resource(ClearColor(BACKGROUND_COLOR))
    .add_state(GameState::Initialization)
    .add_startup_system(setup_main)
    .add_plugins(DefaultPlugins)
    .add_plugin(mainmenu::MainMenuPlugin)
    .add_plugin(overworld::RapidFantasyPlugin)
    .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Initialization,
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

#[derive(AssetCollection)]
struct ImageAssets {
    #[asset(path = "main_menu.png")]
    main_menu: Handle<Image>,
    #[asset(path = "overworld1.png")]
    overworld1: Handle<Image>,

    #[asset(path = "player_up.png")]
    player_up: Handle<Image>,
    #[asset(path = "player_down.png")]
    player_down: Handle<Image>,
    #[asset(path = "player_left.png")]
    player_left: Handle<Image>,
    #[asset(path = "player_right.png")]
    player_right: Handle<Image>,
}

#[derive(AssetCollection)]
struct FontAssets {
    #[asset(path = "fonts/FiraMono-Medium.ttf")]
    font: Handle<Font>,
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
