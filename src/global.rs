use bevy::{math::const_vec2, prelude::*, utils::HashMap};

pub(crate) const TEXT_COLOR: Color = Color::BLACK;
pub(crate) const BACKGROUND_SIZE: Vec2 = const_vec2!([1280., 720.]);
pub(crate) const BACKGROUND_COLOR: Color = Color::BLACK;

pub(crate) const NORMAL_BUTTON: Color = Color::rgb(0.6, 0.6, 0.6);
pub(crate) const HOVERED_BUTTON: Color = Color::rgb(0.8, 0.8, 0.8);
pub(crate) const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub(crate) const PRESSED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

pub(crate) const XP_TABLE: [i32; 5] = [1000, 8000, 27000, 64000, 1];

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub(crate) enum GameState {
    Initialization,
    MainMenu,
    Overworld,
    Menu,
    Battle,
    Lose,
    FinalVictory,
    Exit,
}

#[derive(Default)]
pub(crate) struct Player {
    pub(crate) entity: Option<Entity>,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) stats: Stats,
}

#[derive(Component, Clone, Default)]
pub(crate) struct Stats {
    pub(crate) hp_max: i32,
    pub(crate) mp_max: i32,
    pub(crate) hp: i32,
    pub(crate) mp: i32,
    pub(crate) strength: i32,
    pub(crate) wisdom: i32,
    pub(crate) defense: i32,
    pub(crate) level: i32,
    pub(crate) experience: i32,
    pub(crate) gold: i32,
    pub(crate) battle_sprite: Handle<Image>,
}

impl Stats {
    pub(crate) fn new(battle_sprite: Handle<Image>) -> Self {
        Stats {
            hp_max: 50,
            mp_max: 50,
            hp: 50,
            mp: 50,
            strength: 12,
            wisdom: 12,
            defense: 5,
            level: 1,
            experience: 0,
            gold: 0,
            battle_sprite,
        }
    }
}

#[derive(Component)]
pub(crate) struct LimitBreak(i32);

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub(crate) enum Element {
    Fire,
    Earth,
    Electric,
    Water,
    Light,
    Dark,
} // TODO: implement damage lookup table

#[derive(Default, Component)]
pub(crate) struct Enemy {
    pub(crate) entity: Option<Entity>,
    pub(crate) stats: Stats,
    pub(crate) enemy_stats: EnemyStats,
}

#[derive(Component, Clone, Default)]
pub(crate) struct EnemyStats {
    pub(crate) id: usize,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) element: Option<Element>,
    pub(crate) next_phase: Option<usize>, // id? maybe another enemystats?
} // TODO: implement enemy table

pub(crate) struct EnemyTable {
    pub(crate) table: HashMap<String, (EnemyStats, Stats)>, // TODO: maybe change this to an array?
}
