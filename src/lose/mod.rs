use crate::{button_system, FontAssets, ImageAssets};

use super::{despawn_screen, GameState};
use bevy::prelude::*;

mod styles;
use styles::*;
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
        .spawn_bundle(styled_lose_screen(&image_assets))
        .insert(LoseScreen)
        .with_children(|p| {
            p.spawn_bundle(styled_lose_header(&font_assets));

            p.spawn_bundle(styled_button())
                .insert(MenuButtonAction::BackToMainMenu)
                .with_children(|p| {
                    p.spawn_bundle(styled_button_text(&font_assets));
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
