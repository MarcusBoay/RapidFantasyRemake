use crate::{global, FontAssets};
use bevy::prelude::*;

// Common bundles

pub fn common_text_style(font_assets: &Res<FontAssets>) -> TextStyle {
    TextStyle {
        font: font_assets.font.clone(),
        font_size: 24.,
        color: global::TEXT_COLOR,
    }
}

pub fn styled_text_bundle<T: Into<String>>(text: T, font_assets: &Res<FontAssets>) -> TextBundle {
    TextBundle {
        text: Text::with_section(text, common_text_style(&font_assets), Default::default()),
        ..default()
    }
}

pub fn styled_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(100.0), Val::Px(50.0)),
            margin: Rect::all(Val::Px(1.0)),
            flex_shrink: 0.,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        color: global::NORMAL_BUTTON.into(),
        ..default()
    }
}

// Specific bundles

pub fn styled_menu_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            margin: Rect::all(Val::Auto),
            flex_direction: FlexDirection::Row,
            ..default()
        },
        ..default()
    }
}

pub fn styled_side_panel() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Undefined, Val::Percent(100.)),
            margin: Rect::all(Val::Px(4.)),
            flex_direction: FlexDirection::ColumnReverse,
            ..default()
        },
        ..default()
    }
}

pub fn styled_main_panel() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Undefined, Val::Percent(100.)),
            margin: Rect::all(Val::Px(4.)),
            flex_grow: 1.,
            flex_direction: FlexDirection::ColumnReverse,
            ..default()
        },
        ..default()
    }
}

pub fn styled_stats_panel() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Px(120.)),
            margin: Rect {
                bottom: Val::Px(4.),
                ..default()
            },
            flex_direction: FlexDirection::Row,
            align_content: AlignContent::FlexStart,
            justify_content: JustifyContent::FlexStart,
            ..default()
        },
        ..default()
    }
}

pub fn styled_stat_group_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Undefined, Val::Px(120.)),
            margin: Rect {
                top: Val::Px(1.),
                bottom: Val::Px(1.),
                right: Val::Px(4.),
                left: Val::Px(1.),
            },
            flex_shrink: 1.,
            flex_direction: FlexDirection::ColumnReverse,
            align_self: AlignSelf::FlexStart,
            justify_content: JustifyContent::FlexStart,
            ..default()
        },
        ..default()
    }
}

pub fn styled_stat_with_bar_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(250.), Val::Px(30.)),
            margin: Rect::all(Val::Px(1.)),
            flex_shrink: 1.,
            flex_direction: FlexDirection::ColumnReverse,
            ..default()
        },
        ..default()
    }
}

pub fn styled_stat_out_of_text(
    font_assets: &Res<FontAssets>,
    stat: String,
    current: i32,
    max: i32,
) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: stat,
                    style: common_text_style(font_assets),
                },
                TextSection {
                    value: format!("{} / {}", current, max),
                    style: common_text_style(font_assets),
                },
            ],
            ..default()
        },
        ..default()
    }
}

pub fn styled_stat_bar_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(250.), Val::Px(6.)),
            ..default()
        },
        color: Color::GRAY.into(),
        ..default()
    }
}

pub fn styled_stat_bar(perc: f32, color: Color) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(perc), Val::Percent(100.)),
            ..default()
        },
        color: color.into(),
        ..default()
    }
}
