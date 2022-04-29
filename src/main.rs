mod battle;
mod mainmenu; // why does this work?????
mod overworld;
use bevy::{math::const_vec2, prelude::*, window::PresentMode};
use bevy_asset_loader::{AssetCollection, AssetLoader};

fn main() {
    let mut app = App::new();
    AssetLoader::new(GameState::Initialization)
        .continue_to_state(GameState::Battle) // TODO: change back to MainMenu after done testing
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
    .add_plugin(overworld::OverworldPlugin)
    .add_plugin(battle::BattlePlugin)
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

const TEXT_COLOR: Color = Color::BLACK;
const BACKGROUND_SIZE: Vec2 = const_vec2!([1280., 720.]);
const BACKGROUND_COLOR: Color = Color::BLACK;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

#[derive(AssetCollection)]
pub struct ImageAssets {
    #[asset(path = "images/main_menu.png")]
    main_menu: Handle<Image>,
    #[asset(path = "images/overworld1.png")]
    overworld1: Handle<Image>,

    #[asset(path = "images/player_up.png")]
    player_up: Handle<Image>,
    #[asset(path = "images/player_down.png")]
    player_down: Handle<Image>,
    #[asset(path = "images/player_left.png")]
    player_left: Handle<Image>,
    #[asset(path = "images/player_right.png")]
    player_right: Handle<Image>,

    #[asset(path = "images/player_battle.png")]
    player_battle: Handle<Image>,
    #[asset(path = "images/enemy1.png")]
    enemy1: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraMono-Medium.ttf")]
    font: Handle<Font>,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Stats {
    hp: i32,
    mp: i32,
    attack: i32, // TODO: change to str, wisdom, ...
}

fn setup_main(mut commands: Commands) {
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // TODO: Player stats
    commands.spawn().insert(Player).insert(Stats {
        hp: 2000,
        mp: 1234,
        attack: 333,
    });
}

// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in interaction_query.iter_mut() {
        *color = match (*interaction, selected) {
            (Interaction::Clicked, _) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// Recursively set the visibility of entities
// https://github.com/bevyengine/bevy/issues/838#issuecomment-772082427
fn set_visible_recursive(
    is_visible: bool,
    entity: Entity,
    visible_query: &mut Query<&mut Visibility>,
    children_query: &Query<&Children>,
) {
    if let Ok(mut visible) = visible_query.get_mut(entity) {
        visible.is_visible = is_visible;
    }

    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            set_visible_recursive(is_visible, *child, visible_query, children_query);
        }
    }
}
