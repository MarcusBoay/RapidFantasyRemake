mod battle;
mod enemy_table;
mod global;
mod lose;
mod mainmenu; // why does this work?????
mod overworld;
mod player_attack_table;
use bevy::{prelude::*, window::PresentMode};
use bevy_asset_loader::{AssetCollection, AssetLoader};

fn main() {
    let mut app = App::new();
    AssetLoader::new(global::GameState::Initialization)
        .continue_to_state(global::GameState::MainMenu) // TODO: change back to MainMenu after done testing
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
    .insert_resource(ClearColor(global::BACKGROUND_COLOR))
    .init_resource::<global::PlayerAttackTable>()
    .init_resource::<global::Player>()
    .init_resource::<global::PlayerMagicEquipped>()
    .init_resource::<global::PlayerLimitEquipped>()
    .init_resource::<global::Enemy>()
    .add_state(global::GameState::Initialization)
    .add_startup_system(setup_main)
    .add_plugins(DefaultPlugins)
    .add_plugin(mainmenu::MainMenuPlugin)
    .add_plugin(overworld::OverworldPlugin)
    .add_plugin(battle::BattlePlugin)
    .add_plugin(lose::LosePlugin)
    .run();
}

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

#[derive(AssetCollection)]
pub struct ImageAssets {
    #[asset(path = "images/main_menu.png")]
    main_menu: Handle<Image>,
    #[asset(path = "images/overworld1.png")]
    overworld1: Handle<Image>,

    #[asset(path = "images/game_over.png")]
    game_over: Handle<Image>,

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
    #[asset(path = "images/enemy2.png")]
    enemy2: Handle<Image>,
    #[asset(path = "images/enemy3.png")]
    enemy3: Handle<Image>,
    #[asset(path = "images/enemy4.png")]
    enemy4: Handle<Image>,
    #[asset(path = "images/enemy5.png")]
    enemy5: Handle<Image>,
    #[asset(path = "images/enemy6.png")]
    enemy6: Handle<Image>,
    #[asset(path = "images/enemy7.png")]
    enemy7: Handle<Image>,
    #[asset(path = "images/enemy8.png")]
    enemy8: Handle<Image>,
    #[asset(path = "images/enemy9.png")]
    enemy9: Handle<Image>,
    #[asset(path = "images/enemy10.png")]
    enemy10: Handle<Image>,
    #[asset(path = "images/enemy11.png")]
    enemy11: Handle<Image>,
    #[asset(path = "images/enemy12.png")]
    enemy12: Handle<Image>,
    #[asset(path = "images/enemy13.png")]
    enemy13: Handle<Image>,
    #[asset(path = "images/enemy14.png")]
    enemy14: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraMono-Medium.ttf")]
    font: Handle<Font>,
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    font_bold: Handle<Font>,
}

fn setup_main(mut commands: Commands) {
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
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
            (Interaction::Clicked, _) => global::PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => global::HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => global::HOVERED_BUTTON.into(),
            (Interaction::None, Some(_)) => global::PRESSED_BUTTON.into(),
            (Interaction::None, None) => global::NORMAL_BUTTON.into(),
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
