use crate::{button_system, FontAssets, ImageAssets, NORMAL_BUTTON};

use super::{despawn_screen, GameState, TEXT_COLOR};
use bevy::prelude::*;

pub struct LosePlugin;

impl Plugin for LosePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Lose).with_system(lose_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Lose)
                    .with_system(menu_action)
                    .with_system(button_system),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Lose).with_system(despawn_screen::<LoseScreen>),
            );
    }
}

#[derive(Component)]
enum MenuButtonAction {
    BackToMainMenu,
}

#[derive(Component)]
struct LoseScreen;

fn lose_setup(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    font_assets: Res<FontAssets>,
) {
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
            image: image_assets.game_over.clone().into(),
            ..default()
        })
        .insert(LoseScreen)
        .with_children(|p| {
            p.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "You Lost",
                    TextStyle {
                        font: font_assets.font_bold.clone(),
                        font_size: 120.,
                        color: TEXT_COLOR,
                    },
                    Default::default(),
                ),
                style: Style {
                    margin: Rect {
                        top: Val::Px(150.),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            });

            p.spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(400.), Val::Px(65.)),
                    margin: Rect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: NORMAL_BUTTON.into(),
                ..default()
            })
            .insert(MenuButtonAction::BackToMainMenu)
            .with_children(|p| {
                p.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Back to Main Menu",
                        TextStyle {
                            font: font_assets.font.clone(),
                            font_size: 40.0,
                            color: TEXT_COLOR,
                        },
                        Default::default(),
                    ),
                    ..default()
                });
            });
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<State<GameState>>,
) {
    for (interaction, menu_button_action) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                MenuButtonAction::BackToMainMenu => {
                    game_state.set(GameState::MainMenu).unwrap();
                }
            }
        }
    }
}
