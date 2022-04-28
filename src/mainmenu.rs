use crate::{FontAssets, ImageAssets};

use super::{despawn_screen, GameState, TEXT_COLOR};
use bevy::{prelude::*, window::WindowMode};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(MenuState::Main)
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(main_menu_setup))
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu)
                    .with_system(menu_action)
                    .with_system(button_system),
            )
            // .add_system_set(SystemSet::on_exit())
            .add_system(change_window_settings) // TODO: settings screen..
            // When exiting the state, despawn everything that was spawned for this screen
            .add_system_set(
                SystemSet::on_exit(GameState::MainMenu)
                    .with_system(despawn_screen::<OnMainMenuScreen>),
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

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// Tag component used to mark wich setting is currently selected
#[derive(Component)]
struct SelectedOption;

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
struct OnMainMenuScreen;

fn main_menu_setup(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    font_assets: Res<FontAssets>,
) {
    // Common style for all buttons on the screen
    let button_style = Style {
        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
        margin: Rect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    // let button_icon_style = Style {
    //     size: Size::new(Val::Px(30.0), Val::Auto),
    //     // This takes the icons out of the flexbox flow, to be positionned exactly
    //     position_type: PositionType::Absolute,
    //     // The icon will be close to the left border of the button
    //     position: Rect {
    //         left: Val::Px(10.0),
    //         right: Val::Auto,
    //         top: Val::Auto,
    //         bottom: Val::Auto,
    //     },
    //     ..default()
    // };
    let button_text_style = TextStyle {
        // font: Default::default(),
        font: font_assets.font.clone(),
        font_size: 40.0,
        color: TEXT_COLOR,
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
        .insert(OnMainMenuScreen)
        .with_children(|parent| {
            // Display start game button
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(MenuButtonAction::Play)
                .with_children(|parent| {
                    // parent.spawn_bundle(TextBundle {
                    //     text: Text::with_section(
                    //         "Start Game",
                    //         button_text_style.clone(),
                    //         Default::default(),
                    //     ),
                    //     ..default()
                    // });
                    parent.spawn_bundle(TextBundle {
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

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu_state: ResMut<State<MenuState>>,
    mut game_state: ResMut<State<GameState>>,
) {
    for (interaction, menu_button_action) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                MenuButtonAction::Play => {
                    game_state.set(GameState::Overworld).unwrap();
                    menu_state.set(MenuState::Disabled).unwrap();
                }
                _ => unimplemented!("Unhandled menu button action!!"), // TODO
            }
        }
    }
}

// TODO: make this a button in the settings menu
fn change_window_settings(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    if input.just_pressed(KeyCode::O) {
        if window.mode() == WindowMode::Windowed {
            window.set_mode(WindowMode::Fullscreen);
        } else {
            window.set_mode(WindowMode::Windowed);
        }
    }
}
