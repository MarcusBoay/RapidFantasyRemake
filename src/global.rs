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
    pub(crate) limit: u8,
    pub(crate) area: u32,
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

#[derive(Clone)]
pub(crate) enum PlayerAttackType {
    Limit,
    Magic,
}

#[derive(Default, Clone, Component)]
pub(crate) struct PlayerAttack {
    pub(crate) id: usize,
    pub(crate) name: String,
    pub(crate) attack_type: Option<PlayerAttackType>, // None = standard attack
    pub(crate) element: Option<Element>,              // None = no type
    pub(crate) mp_use: i32,
    pub(crate) tier: u8,
}

impl PlayerAttack {
    pub(crate) fn new(
        id: usize,
        name: &str,
        attack_type: Option<PlayerAttackType>,
        element: Option<Element>,
        mp_use: i32,
        tier: u8,
    ) -> Self {
        PlayerAttack {
            id,
            name: name.to_string(),
            attack_type,
            element,
            mp_use,
            tier,
        }
    }
}

pub(crate) struct PlayerAttackTable {
    pub(crate) table: HashMap<u32, PlayerAttack>,
}

#[derive(Default)]
pub(crate) struct PlayerMagicEquipped {
    pub(crate) equipped: [Option<PlayerAttack>; 4],
}

// TODO: remove!!!
// impl FromWorld for PlayerMagicEquipped {
//     fn from_world(world: &mut World) -> Self {
//         let attack_table = &world.get_resource_mut::<PlayerAttackTable>().unwrap().table;
//         PlayerMagicEquipped {
//             equipped: [
//                 Some(attack_table.get(&4).unwrap().clone()),
//                 None,
//                 None,
//                 None,
//             ],
//         }
//     }
// }

pub(crate) struct PlayerLimitEquipped {
    pub(crate) equipped: PlayerAttack,
}

impl FromWorld for PlayerLimitEquipped {
    fn from_world(world: &mut World) -> Self {
        let attack_table = &world.get_resource_mut::<PlayerAttackTable>().unwrap().table;
        PlayerLimitEquipped {
            equipped: attack_table.get(&1).unwrap().clone(),
        }
    }
}

impl Stats {
    pub(crate) fn new(battle_sprite: Handle<Image>) -> Self {
        Stats {
            hp_max: 100,
            mp_max: 100,
            hp: 100,
            mp: 100,
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
    pub(crate) attacks: Vec<EnemyAttack>,
}

#[derive(Component, Clone, Default)]
pub(crate) struct EnemyStats {
    pub(crate) id: usize,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) element: Option<Element>,
    pub(crate) next_phase: Option<usize>, // id? maybe another enemystats?
} // TODO: implement enemy table

#[derive(Clone)]
pub(crate) enum EnemyAttackType {
    Magic,
    Percentile,
}

#[derive(Default, Clone)]
pub(crate) struct EnemyAttack {
    pub(crate) name: String,
    pub(crate) damage_modifier: f32,
    pub(crate) mp_use: i32,
    pub(crate) attack_type: Option<EnemyAttackType>, // None = Physical
}

impl EnemyAttack {
    pub(crate) fn new(
        name: &str,
        damage_modifier: f32,
        attack_type: Option<EnemyAttackType>,
        mp_use: i32,
    ) -> Self {
        EnemyAttack {
            name: name.to_string(),
            damage_modifier,
            attack_type,
            mp_use,
        }
    }
}

pub(crate) struct EnemyTable {
    pub(crate) table: HashMap<u32, (EnemyStats, Stats, Vec<EnemyAttack>)>,
}
