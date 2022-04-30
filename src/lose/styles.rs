use crate::{global, FontAssets, ImageAssets};
use bevy::prelude::*;

pub fn styled_lose_screen(image_assets: &Res<ImageAssets>) -> NodeBundle {
    NodeBundle {
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
    }
}

pub fn styled_lose_header(font_assets: &Res<FontAssets>) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            "You Lost",
            TextStyle {
                font: font_assets.font_bold.clone(),
                font_size: 120.,
                color: global::TEXT_COLOR,
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
    }
}

pub fn styled_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(400.), Val::Px(65.)),
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        color: global::NORMAL_BUTTON.into(),
        ..default()
    }
}

pub fn styled_button_text(font_assets: &Res<FontAssets>) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            "Back to Main Menu",
            TextStyle {
                font: font_assets.font.clone(),
                font_size: 40.0,
                color: global::TEXT_COLOR,
            },
            Default::default(),
        ),
        ..default()
    }
}
