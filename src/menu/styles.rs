use crate::{
    global::{self, BACKGROUND_SIZE},
    FontAssets,
};
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
        text: styled_text(text, &font_assets),
        ..default()
    }
}

pub fn styled_text<T: Into<String>>(text: T, font_assets: &Res<FontAssets>) -> Text {
    Text::with_section(text, common_text_style(&font_assets), Default::default())
}

pub fn styled_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(100.), Val::Px(50.)),
            margin: Rect::all(Val::Px(1.)),
            flex_shrink: 0.,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        color: global::NORMAL_BUTTON.into(),
        ..default()
    }
}

pub fn styled_subpanel_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(500.), Val::Px(50.)),
            margin: Rect::all(Val::Px(1.)),
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
            size: Size::new(Val::Px(BACKGROUND_SIZE[0]), Val::Px(BACKGROUND_SIZE[1])),
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
            size: Size::new(Val::Px(100.), Val::Percent(100.)),
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
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
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

pub fn styled_sub_panel() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            ..default()
        },
        ..default()
    }
}

pub fn styled_sub_sub_panel() -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::ColumnReverse,
            align_self: AlignSelf::Center,
            size: Size::new(Val::Percent(50.0), Val::Percent(100.)),
            overflow: Overflow::Hidden,
            ..default()
        },
        color: Color::FUCHSIA.into(),
        ..default()
    }
}

pub fn styled_item_list() -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::ColumnReverse,
            flex_grow: 1.0,
            max_size: Size::new(Val::Px(500.), Val::Percent(100.)),
            ..default()
        },
        color: Color::BLUE.into(),
        ..default()
    }
}
