use crate::{despawn_screen, FontAssets, ImageAssets};

use super::{GameState, TEXT_COLOR};
use bevy::prelude::*;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Battle).with_system(battle_setup))
            // .add_system_set(
            //     SystemSet::on_update(GameState::Battle)
            // )
            // When exiting the state, despawn everything that was spawned for this screen
            .add_system_set(
                SystemSet::on_exit(GameState::Battle).with_system(despawn_screen::<BattleScreen>),
            );
    }
}

#[derive(Component)]
struct BattleScreen;

fn battle_setup(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    font_assets: Res<FontAssets>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: Rect::all(Val::Auto),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..default()
            },
            color: Color::CRIMSON.into(),
            ..default()
        })
        .insert(BattleScreen)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(50.0)),
                    ..default()
                },
                text: Text::with_section(
                    "FIRST TEXT",
                    TextStyle {
                        font: font_assets.font.clone(),
                        font_size: 80.0,
                        color: TEXT_COLOR,
                    },
                    Default::default(),
                ),
                ..default()
            });
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(50.0)),
                    ..default()
                },
                text: Text::with_section(
                    "SECOND TEXT",
                    TextStyle {
                        font: font_assets.font.clone(),
                        font_size: 80.0,
                        color: TEXT_COLOR,
                    },
                    Default::default(),
                ),
                ..default()
            });
        });
}
