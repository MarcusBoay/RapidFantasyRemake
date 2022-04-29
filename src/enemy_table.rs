use bevy::{prelude::*, utils::HashMap};

use crate::{EnemyTable, ImageAssets, EnemyStats, Element, Stats};

impl FromWorld for EnemyTable {
    fn from_world(world: &mut World) -> Self {
        let image_assets = world.get_resource_mut::<ImageAssets>().unwrap();
        let mut enemies = HashMap::new();

        let name = "Slime".to_string();
        enemies.insert(
            name.clone(),
            (
                EnemyStats {
                    id: enemies.len(),
                    name: name.clone(),
                    description: "I wonder if it\'s edible?".to_string(),
                    element: Element::None,
                    next_phase: None,
                },
                Stats {
                    hp_max: 39,
                    hp: 39,
                    mp_max: 10,
                    mp: 10,
                    strength: 8,
                    wisdom: 8,
                    defense: 8,
                    level: 1,
                    experience: 180,
                    gold: 50,
                    battle_sprite: image_assets.enemy1.clone(),
                },
            ),
        );
        // TODO: add more enemies
        // TODO: maybe put this into another file...

        EnemyTable { table: enemies }
    }
}