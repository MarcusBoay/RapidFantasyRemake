use bevy::{
    math::const_vec2,
    prelude::*,
    utils::{HashMap, HashSet},
};
use rand::{thread_rng, Rng};

use crate::ImageAssets;

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
pub(crate) struct Area {
    pub(crate) id: usize,
    pub(crate) enemies: Vec<usize>, // enemy ids
    pub(crate) background: Handle<Image>,
}

impl Area {
    fn new(id: usize, enemies: Vec<usize>, background: Handle<Image>) -> Area {
        Area {
            id,
            enemies,
            background,
        }
    }
}

#[derive(Deref)]
pub(crate) struct Areas(HashMap<usize, Area>);

impl FromWorld for Areas {
    fn from_world(world: &mut World) -> Self {
        let image_assets = world.get_resource_mut::<ImageAssets>().unwrap();
        let mut areas = HashMap::new();
        areas.insert(0, Area::new(0, vec![], image_assets.area0.clone()));
        areas.insert(1, Area::new(1, vec![0, 1, 2], image_assets.area1.clone()));
        areas.insert(2, Area::new(2, vec![3, 4], image_assets.area2.clone()));
        areas.insert(3, Area::new(3, vec![5, 6], image_assets.area3.clone()));
        areas.insert(4, Area::new(4, vec![7, 8], image_assets.area4.clone()));
        areas.insert(5, Area::new(5, vec![9, 10], image_assets.area5.clone()));

        Areas(areas)
    }
}

#[derive(Default)]
pub(crate) struct Player {
    pub(crate) entity: Option<Entity>,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) stats: Stats,
    pub(crate) limit: u8,
    pub(crate) area: usize,
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

#[derive(Clone, Hash, PartialEq, Eq)]
pub(crate) enum PlayerAttackType {
    Limit,
    Magic,
}

#[derive(Default, Clone, Component, Hash, PartialEq, Eq)]
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

#[derive(Deref)]
pub(crate) struct PlayerMagicEquipped(pub(crate) [Option<PlayerAttack>; 4]);

impl FromWorld for PlayerMagicEquipped {
    fn from_world(world: &mut World) -> Self {
        let attack_table = &world.get_resource_mut::<PlayerAttackTable>().unwrap().table;
        // Player starts out with tier 1 magic equipped.
        PlayerMagicEquipped([
            Some(attack_table.get(&4).unwrap().clone()),
            Some(attack_table.get(&5).unwrap().clone()),
            Some(attack_table.get(&6).unwrap().clone()),
            Some(attack_table.get(&7).unwrap().clone()),
        ])
    }
}

#[derive(Deref)]
pub(crate) struct PlayerLimitEquipped(pub(crate) PlayerAttack);

impl FromWorld for PlayerLimitEquipped {
    fn from_world(world: &mut World) -> Self {
        let attack_table = &world.get_resource_mut::<PlayerAttackTable>().unwrap().table;
        // Player starts out with 1st limit break.
        PlayerLimitEquipped(attack_table.get(&1).unwrap().clone())
    }
}

#[derive(Deref, DerefMut)]
pub(crate) struct PlayerAttackInventory(pub(crate) HashSet<PlayerAttack>);

impl FromWorld for PlayerAttackInventory {
    fn from_world(world: &mut World) -> Self {
        let attack_table = world
            .get_resource_mut::<PlayerAttackTable>()
            .unwrap()
            .table
            .clone();
        let mut attacks = HashSet::new();

        // Player starts out with tier 1 limit and magic.
        attacks.insert(attack_table.get(&0).unwrap().clone());

        attacks.insert(attack_table.get(&1).unwrap().clone());

        for i in 4..10 {
            attacks.insert(attack_table.get(&i).unwrap().clone());
        }

        Self(attacks)
    }
}

#[derive(Deref, DerefMut)]
pub(crate) struct PlayerItemInventory(pub(crate) HashMap<usize, usize>); // id, quantity, TODO: maybe replace id with Item...

impl FromWorld for PlayerItemInventory {
    fn from_world(_: &mut World) -> Self {
        let mut items = HashMap::new();
        // Player starts out with 5 red and blue potions.
        items.insert(0, 5);
        items.insert(5, 5);
        PlayerItemInventory(items)
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
    pub(crate) table: HashMap<usize, (EnemyStats, Stats, Vec<EnemyAttack>, Vec<LootTable>)>,
}

#[derive(Clone)]
pub(crate) enum ItemType {
    Consumable,
    Weapon,
    Armor,
    Accessory,
}

#[derive(Clone)]
pub(crate) struct ItemStats {
    pub(crate) hp_max: i32,
    pub(crate) mp_max: i32,
    pub(crate) hp: i32,
    pub(crate) mp: i32,
    pub(crate) strength: i32,
    pub(crate) wisdom: i32,
    pub(crate) defense: i32,
}

impl ItemStats {
    pub(crate) fn new_potion(hp: i32, mp: i32) -> Self {
        Self {
            hp_max: 0,
            mp_max: 0,
            hp,
            mp,
            strength: 0,
            wisdom: 0,
            defense: 0,
        }
    }

    pub(crate) fn new_eqiup(
        hp_max: i32,
        mp_max: i32,
        strength: i32,
        wisdom: i32,
        defense: i32,
    ) -> Self {
        Self {
            hp_max,
            mp_max,
            hp: 0,
            mp: 0,
            strength,
            wisdom,
            defense,
        }
    }
}

#[derive(Clone)]
pub(crate) struct Item {
    pub(crate) id: usize,
    pub(crate) name: String,
    pub(crate) item_type: ItemType,
    pub(crate) stats: ItemStats,
}

impl Item {
    pub(crate) fn new(id: usize, name: &str, item_type: ItemType, stats: ItemStats) -> Self {
        Self {
            id,
            name: name.to_string(),
            item_type,
            stats,
        }
    }
}

#[derive(Deref)]
pub(crate) struct ItemTable(pub(crate) HashMap<usize, Item>);

pub(crate) struct LootTable {
    pub(crate) no_drop_weight: usize,
    pub(crate) items: Vec<(usize, usize)>, // item id, weight
}

impl LootTable {
    pub(crate) fn get_item_id(&self) -> Option<usize> {
        let roll = thread_rng().gen_range(0..self.get_total_weight());

        let mut cur_weight = self.no_drop_weight;
        for item in &self.items {
            if roll >= cur_weight && roll < cur_weight + item.1 {
                return Some(item.0);
            }
            cur_weight += item.1;
        }

        None
    }

    fn get_total_weight(&self) -> usize {
        self.no_drop_weight + self.items.iter().map(|x| x.1).sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loot_table_get_total_weight() {
        let loot_table = LootTable {
            no_drop_weight: 6,
            items: vec![(1, 7), (2, 8)],
        };
        assert_eq!(6 + 7 + 8, loot_table.get_total_weight());
    }

    #[test]
    fn loot_table_get_no_item() {
        let loot_table = LootTable {
            no_drop_weight: 6,
            items: vec![],
        };
        assert_eq!(None, loot_table.get_item_id());
    }

    #[test]
    fn loot_table_get_item() {
        let loot_table = LootTable {
            no_drop_weight: 0,
            items: vec![(32, 1)],
        };
        assert_eq!(Some(32), loot_table.get_item_id());
    }
}
