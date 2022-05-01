use bevy::{prelude::*, utils::HashMap};

use crate::{
    global::{EnemyAttack, EnemyAttackType, EnemyStats, EnemyTable, Stats},
    ImageAssets,
};

impl FromWorld for EnemyTable {
    fn from_world(world: &mut World) -> Self {
        let image_assets = world.get_resource_mut::<ImageAssets>().unwrap();
        let mut enemies = HashMap::new();

        enemies.insert(
            0,
            (
                EnemyStats {
                    id: 0,
                    name: "Slime".to_string(),
                    description: "I wonder if it\'s edible?".to_string(),
                    element: None,
                    next_phase: None,
                },
                Stats {
                    hp_max: 39,
                    hp: 39,
                    mp_max: 10,
                    mp: 10,
                    strength: 8,
                    wisdom: 8,
                    defense: 5,
                    level: 1,
                    experience: 180,
                    gold: 50,
                    battle_sprite: image_assets.enemy1.clone(),
                },
                vec![EnemyAttack {
                    name: "Bounce".to_string(),
                    damage_modifier: 2.,
                    mp_use: 0,
                    attack_type: None,
                }],
            ),
        );
        // TODO: add more enemies
        // -> write python parser to easily convert data

        EnemyTable { table: enemies }
    }
}
