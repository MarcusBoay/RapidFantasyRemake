use crate::{button_system, despawn_screen, global, FontAssets, ImageAssets};

use bevy::{prelude::*, window::WindowMode};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(MenuState::Main)
            .add_system_set(
                SystemSet::on_enter(global::GameState::MainMenu).with_system(main_menu_setup),
            )
            .add_system_set(
                SystemSet::on_update(global::GameState::MainMenu)
                    .with_system(menu_action)
                    .with_system(button_system),
            )
            .add_system_set(
                SystemSet::on_exit(global::GameState::MainMenu)
                    .with_system(despawn_screen::<MainMenuScreen>),
            );
    }
}

// State used for the current menu screen
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum MenuState {
    // TODO: to use...
    Main,
    Settings, // TODO
    Disabled,
}

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    Play,
    Settings,
    SettingsDisplay,
    SettingsSound,
    BackToMainMenu,
    BackToSettings,
    Quit,
}

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct MainMenuScreen;

fn main_menu_setup(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    font_assets: Res<FontAssets>,
    mut menu_state: ResMut<State<MenuState>>,
) {
    // HACK: This is here because we need ImageAssets to be initialized...
    commands.init_resource::<global::EnemyTable>();
    commands.init_resource::<global::Areas>();

    // Reset state for recurring visit to this page.
    if *menu_state.current() == MenuState::Disabled {
        menu_state.set(MenuState::Main).unwrap();
    }

    let button_style = Style {
        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
        margin: Rect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font: font_assets.font.clone(),
        font_size: 40.0,
        color: global::TEXT_COLOR,
    };

    // Button panel
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: Rect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..default()
            },
            image: image_assets.main_menu.clone().into(),
            ..default()
        })
        .insert(MainMenuScreen)
        .with_children(|p| {
            // Display start game button
            p.spawn_bundle(ButtonBundle {
                style: button_style.clone(),
                color: global::NORMAL_BUTTON.into(),
                ..default()
            })
            .insert(MenuButtonAction::Play)
            .with_children(|p| {
                p.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "New Game",
                        button_text_style.clone(),
                        Default::default(),
                    ),
                    ..default()
                });
            });

            // TODO: settings button
        });
}

fn menu_action(
    mut commands: Commands,
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu_state: ResMut<State<MenuState>>,
    mut game_state: ResMut<State<global::GameState>>,
    image_assets: Res<ImageAssets>,
    attack_table: Res<global::PlayerAttackTable>,
) {
    for (interaction, menu_button_action) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                MenuButtonAction::Play => {
                    commands.insert_resource(global::Player::new(&image_assets));
                    commands.insert_resource(global::PlayerEquipmentEquipped::default());
                    commands.insert_resource(global::PlayerMagicEquipped::new(&attack_table));
                    commands.insert_resource(global::PlayerLimitEquipped::new(&attack_table));
                    commands.insert_resource(global::PlayerItemInventory::new());
                    commands.insert_resource(global::PlayerAttackInventory::new(&attack_table));
                    game_state.set(global::GameState::Overworld).unwrap();
                    menu_state.set(MenuState::Disabled).unwrap();
                    // player.stats = global::Stats::new(image_assets.player_battle.clone());
                    // TODO: set player inventories to new game
                }
                _ => todo!("Unhandled menu button action!!"), // TODO
            }
        }
    }
}
