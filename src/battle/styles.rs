use bevy::prelude::*;

use crate::{FontAssets, NORMAL_BUTTON, TEXT_COLOR};

use super::PlayerButtonAction;

pub fn common_text_style(font_assets: &Res<FontAssets>) -> TextStyle {
    TextStyle {
        font: font_assets.font.clone(),
        font_size: 24.,
        color: TEXT_COLOR,
    }
}

pub fn styled_battle_screen() -> NodeBundle {
    NodeBundle {
        style: Style {
            margin: Rect::all(Val::Auto),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            ..default()
        },
        color: Color::CRIMSON.into(),
        ..default()
    }
}

pub fn styled_bottom_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Percent(40.)),
            flex_direction: FlexDirection::Row,
            ..default()
        },
        ..default()
    }
}

pub fn styled_player_stats_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::ColumnReverse,
            size: Size::new(Val::Percent(30.), Val::Percent(92.)),
            margin: Rect::all(Val::Percent(1.)),
            ..default()
        },
        ..default()
    }
}

pub fn styled_player_stats_child_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::ColumnReverse,
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            ..default()
        },
        ..default()
    }
}

pub fn styled_player_hp_text(font_assets: &Res<FontAssets>, hp: i32, hp_max: i32) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "HP: ".to_string(),
                    style: common_text_style(font_assets),
                },
                TextSection {
                    value: format!("{} / {}", hp, hp_max),
                    style: common_text_style(font_assets),
                },
            ],
            ..default()
        },
        ..default()
    }
}

pub fn styled_player_hp_bar_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Px(12.)),
            ..default()
        },
        color: Color::GRAY.into(),
        ..default()
    }
}

pub fn styled_player_hp_bar(hp_perc: f32) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(hp_perc), Val::Percent(100.)), // TODO: update width
            ..default()
        },
        color: Color::LIME_GREEN.into(), // TODO: update to RED / YELLOW based on HP percentage
        ..default()
    }
}

pub fn styled_player_mp_text(font_assets: &Res<FontAssets>, mp: i32, mp_max: i32) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "MP: ".to_string(),
                    style: common_text_style(font_assets),
                },
                TextSection {
                    value: format!("{} / {}", mp, mp_max),
                    style: common_text_style(font_assets),
                },
            ],
            ..default()
        },
        ..default()
    }
}

pub fn styled_player_mp_bar_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Px(12.)),
            ..default()
        },
        color: Color::GRAY.into(),
        ..default()
    }
}

pub fn styled_player_mp_bar(mp_perc: f32) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(mp_perc), Val::Percent(100.)), // TODO: update width
            ..default()
        },
        color: Color::BLUE.into(),
        ..default()
    }
}

pub fn styled_player_limit_break_text(font_assets: &Res<FontAssets>) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            "Limit Break".to_string(),
            common_text_style(font_assets),
            Default::default(),
        ),
        ..default()
    }
}

pub fn styled_player_limit_break_bar_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Px(15.)),
            ..default()
        },
        color: Color::GRAY.into(),
        ..default()
    }
}

pub fn styled_player_limit_break_bar() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(62.), Val::Percent(100.)), // TODO: update width based on limit break value
            ..default()
        },
        color: Color::ORANGE.into(), // TODO: update color based on limit break value
        ..default()
    }
}

pub fn styled_player_action_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(70.), Val::Percent(92.)),
            margin: Rect::all(Val::Percent(1.)),
            flex_direction: FlexDirection::ColumnReverse,
            ..default()
        },
        visibility: Visibility { is_visible: false },
        ..default()
    }
}

pub fn styled_player_action_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(250.0), Val::Percent(23.0)),
            margin: Rect::all(Val::Px(1.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        visibility: Visibility { is_visible: false },
        color: NORMAL_BUTTON.into(),
        ..default()
    }
}

pub fn styled_player_action_button_text(
    player_button_action: &PlayerButtonAction,
    font_assets: &Res<FontAssets>,
) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            format!("{:?}", player_button_action),
            TextStyle {
                font: font_assets.font.clone(),
                font_size: 40.0,
                color: TEXT_COLOR,
            },
            Default::default(),
        ),
        visibility: Visibility { is_visible: false },
        ..default()
    }
}

pub fn styled_announcement_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Percent(10.)),
            align_items: AlignItems::Center,
            padding: Rect {
                left: Val::Percent(1.),
                right: Val::Percent(1.),
                ..default()
            },
            ..default()
        },
        ..default()
    }
}

pub fn styled_announcement_text(font_assets: &Res<FontAssets>, name: String) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            format!("A wild {} appeared!", name),
            common_text_style(&font_assets),
            Default::default(),
        ),
        ..default()
    }
}

pub fn styled_battle_images_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Percent(50.)),
            ..default()
        },
        color: Color::WHITE.into(),
        ..default()
    }
}

pub fn styled_battle_portrait(image: Handle<Image>) -> ImageBundle {
    ImageBundle {
        style: Style {
            size: Size::new(Val::Px(256.), Val::Px(256.)),
            margin: Rect::all(Val::Auto),
            ..default()
        },
        image: UiImage(image),
        ..default()
    }
}
