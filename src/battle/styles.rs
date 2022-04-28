use bevy::prelude::*;

use crate::{FontAssets, TEXT_COLOR};

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

pub fn styled_player_hp_text(font_assets: &Res<FontAssets>) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "HP: ".to_string(),
                    style: common_text_style(font_assets),
                },
                TextSection {
                    value: "1234 / 4321".to_string(), // TODO: use actual value,
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

pub fn styled_player_hp_bar() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(80.), Val::Percent(100.)), // TODO: update width
            ..default()
        },
        color: Color::LIME_GREEN.into(), // TODO: update to RED / YELLOW based on HP percentage
        ..default()
    }
}

pub fn styled_player_mp_text(font_assets: &Res<FontAssets>) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "MP: ".to_string(),
                    style: common_text_style(font_assets),
                },
                TextSection {
                    value: "777 / 999".to_string(), // TODO: use actual value,
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

pub fn styled_player_mp_bar() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(80.), Val::Percent(100.)), // TODO: update width
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
            ..default()
        },
        color: Color::GRAY.into(),
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

pub fn styled_announcement_text(font_assets: &Res<FontAssets>) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            "A wild encounter appeared!".to_string(),
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
