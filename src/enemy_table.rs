use bevy::{prelude::*, utils::HashMap};

use crate::{
    global::{
        Element::*, EnemyAttack, EnemyAttackType::*, EnemyStats, EnemyTable, LootTable, Stats,
    },
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
                vec![EnemyAttack::new("Bounce", 2., None, 0)],
                vec![
                    LootTable {
                        no_drop_weight: 10,
                        items: vec![(0, 9), (5, 6)],
                    },
                    LootTable {
                        no_drop_weight: 13,
                        items: vec![
                            (10, 1),
                            (11, 1),
                            (12, 1),
                            (19, 1),
                            (20, 1),
                            (21, 1),
                            (28, 1),
                            (29, 1),
                            (30, 1),
                        ],
                    },
                ],
            ),
        );
        enemies.insert(
            1,
            (
                EnemyStats {
                    id: 1,
                    name: "Rocky".to_string(),
                    description: "He\'s a bit of a blockhead.".to_string(),
                    element: Some(Earth),
                    next_phase: None,
                },
                Stats {
                    hp_max: 50,
                    hp: 50,
                    mp_max: 8,
                    mp: 8,
                    strength: 7,
                    wisdom: 7,
                    defense: 8,
                    level: 1,
                    gold: 70,
                    experience: 200,
                    battle_sprite: image_assets.enemy2.clone(),
                },
                vec![EnemyAttack::new("Rock throw", 2., None, 0)],
                vec![
                    LootTable {
                        no_drop_weight: 10,
                        items: vec![(0, 9), (5, 6)],
                    },
                    LootTable {
                        no_drop_weight: 13,
                        items: vec![
                            (10, 1),
                            (11, 1),
                            (12, 1),
                            (19, 1),
                            (20, 1),
                            (21, 1),
                            (28, 1),
                            (29, 1),
                            (30, 1),
                        ],
                    },
                ],
            ),
        );
        enemies.insert(
            2,
            (
                EnemyStats {
                    id: 2,
                    name: "Mushroom".to_string(),
                    description: "Shrooms will mess you up, that\'s why they\'re illegal."
                        .to_string(),
                    element: Some(Earth),
                    next_phase: None,
                },
                Stats {
                    hp_max: 48,
                    hp: 48,
                    mp_max: 15,
                    mp: 15,
                    strength: 5,
                    wisdom: 10,
                    defense: 5,
                    level: 1,
                    gold: 55,
                    experience: 250,
                    battle_sprite: image_assets.enemy3.clone(),
                },
                vec![
                    EnemyAttack::new("Hallucinate", 2., Some(Magic), 5),
                    EnemyAttack::new("Spores", 2.5, None, 0),
                ],
                vec![
                    LootTable {
                        no_drop_weight: 10,
                        items: vec![(0, 9), (5, 6)],
                    },
                    LootTable {
                        no_drop_weight: 13,
                        items: vec![
                            (10, 1),
                            (11, 1),
                            (12, 1),
                            (19, 1),
                            (20, 1),
                            (21, 1),
                            (28, 1),
                            (29, 1),
                            (30, 1),
                        ],
                    },
                ],
            ),
        );
        enemies.insert(
            3,
            (
                EnemyStats {
                    id: 3,
                    name: "Duck".to_string(),
                    description: "He\'s a blast in the bath!".to_string(),
                    element: Some(Fire),
                    next_phase: None,
                },
                Stats {
                    hp_max: 72,
                    hp: 72,
                    mp_max: 10,
                    mp: 10,
                    strength: 13,
                    wisdom: 10,
                    defense: 8,
                    level: 2,
                    gold: 100,
                    experience: 1000,
                    battle_sprite: image_assets.enemy4.clone(),
                },
                vec![
                    EnemyAttack::new("Tackle", 2., None, 0),
                    EnemyAttack::new("Explosion", 3., Some(Magic), 4),
                ],
                vec![
                    LootTable {
                        no_drop_weight: 10,
                        items: vec![(1, 9), (6, 6)],
                    },
                    LootTable {
                        no_drop_weight: 13,
                        items: vec![
                            (10, 1),
                            (11, 1),
                            (12, 1),
                            (19, 1),
                            (20, 1),
                            (21, 1),
                            (28, 1),
                            (29, 1),
                            (30, 1),
                        ],
                    },
                ],
            ),
        );
        enemies.insert(
            4,
            (
                EnemyStats {
                    id: 4,
                    name: "Triple A battery".to_string(),
                    description: "Nobody likes triple A batteries.".to_string(),
                    element: Some(Electric),
                    next_phase: None,
                },
                Stats {
                    hp_max: 96,
                    hp: 96,
                    mp_max: 15,
                    mp: 15,
                    strength: 10,
                    wisdom: 15,
                    defense: 10,
                    level: 2,
                    gold: 80,
                    experience: 1200,
                    battle_sprite: image_assets.enemy5.clone(),
                },
                vec![
                    EnemyAttack::new("Hit", 2., None, 0),
                    EnemyAttack::new("Spark", 2., Some(Magic), 5),
                ],
                vec![
                    LootTable {
                        no_drop_weight: 10,
                        items: vec![(1, 9), (6, 6)],
                    },
                    LootTable {
                        no_drop_weight: 13,
                        items: vec![
                            (10, 1),
                            (11, 1),
                            (12, 1),
                            (19, 1),
                            (20, 1),
                            (21, 1),
                            (28, 1),
                            (29, 1),
                            (30, 1),
                        ],
                    },
                ],
            ),
        );
        enemies.insert(
            5,
            (
                EnemyStats {
                    id: 5,
                    name: "Skeleton".to_string(),
                    description: "Never skip leg day.".to_string(),
                    element: Some(Dark),
                    next_phase: None,
                },
                Stats {
                    hp_max: 156,
                    hp: 156,
                    mp_max: 15,
                    mp: 15,
                    strength: 28,
                    wisdom: 22,
                    defense: 15,
                    level: 3,
                    gold: 125,
                    experience: 2700,
                    battle_sprite: image_assets.enemy6.clone(),
                },
                vec![
                    EnemyAttack::new("Bone Crush", 2., None, 0),
                    EnemyAttack::new("Curse", 2., Some(Magic), 5),
                ],
                vec![
                    LootTable {
                        no_drop_weight: 10,
                        items: vec![(2, 9), (7, 6)],
                    },
                    LootTable {
                        no_drop_weight: 13,
                        items: vec![
                            (13, 1),
                            (14, 1),
                            (15, 1),
                            (22, 1),
                            (23, 1),
                            (24, 1),
                            (31, 1),
                            (32, 1),
                            (33, 1),
                        ],
                    },
                ],
            ),
        );
        enemies.insert(
            6,
            (
                EnemyStats {
                    id: 6,
                    name: "Mixtape".to_string(),
                    description: "An underappreciated mixtape, spits straight fire.".to_string(),
                    element: Some(Fire),
                    next_phase: None,
                },
                Stats {
                    hp_max: 135,
                    hp: 135,
                    mp_max: 20,
                    mp: 20,
                    strength: 20,
                    wisdom: 28,
                    defense: 14,
                    level: 3,
                    gold: 150,
                    experience: 3200,
                    battle_sprite: image_assets.enemy7.clone(),
                },
                vec![
                    EnemyAttack::new("Drop The Beat", 2., Some(Magic), 7),
                    EnemyAttack::new("Lay A Verse", 1.8, Some(Magic), 4),
                    EnemyAttack::new("Tangle", 2., None, 0),
                ],
                vec![
                    LootTable {
                        no_drop_weight: 10,
                        items: vec![(2, 9), (7, 6)],
                    },
                    LootTable {
                        no_drop_weight: 13,
                        items: vec![
                            (13, 1),
                            (14, 1),
                            (15, 1),
                            (22, 1),
                            (23, 1),
                            (24, 1),
                            (31, 1),
                            (32, 1),
                            (33, 1),
                        ],
                    },
                ],
            ),
        );
        enemies.insert(
            7,
            (
                EnemyStats {
                    id: 7,
                    name: "Squirrel".to_string(),
                    description: "It\'s nuts.".to_string(),
                    element: None,
                    next_phase: None,
                },
                Stats {
                    hp_max: 304,
                    hp: 304,
                    mp_max: 20,
                    mp: 20,
                    strength: 52,
                    wisdom: 40,
                    defense: 25,
                    level: 4,
                    gold: 200,
                    experience: 4500,
                    battle_sprite: image_assets.enemy8.clone(),
                },
                vec![
                    EnemyAttack::new("Bite", 2., None, 0),
                    EnemyAttack::new("Gnaw", 1.5, None, 0),
                    EnemyAttack::new("Acornucopia of pain", 2., Some(Magic), 5),
                ],
                vec![
                    LootTable {
                        no_drop_weight: 10,
                        items: vec![(3, 9), (8, 6)],
                    },
                    LootTable {
                        no_drop_weight: 13,
                        items: vec![
                            (13, 1),
                            (14, 1),
                            (15, 1),
                            (22, 1),
                            (23, 1),
                            (24, 1),
                            (31, 1),
                            (32, 1),
                            (33, 1),
                        ],
                    },
                ],
            ),
        );
        enemies.insert(
            8,
            (
                EnemyStats {
                    id: 8,
                    name: "Book".to_string(),
                    description: "Full of questionable knowledge.".to_string(),
                    element: Some(Light),
                    next_phase: None,
                },
                Stats {
                    hp_max: 273,
                    hp: 273,
                    mp_max: 30,
                    mp: 30,
                    strength: 37,
                    wisdom: 54,
                    defense: 24,
                    level: 4,
                    gold: 225,
                    experience: 5800,
                    battle_sprite: image_assets.enemy9.clone(),
                },
                vec![
                    EnemyAttack::new("Body Slam", 2., None, 0),
                    EnemyAttack::new("Confusion", 2., Some(Magic), 6),
                    EnemyAttack::new("Face the Book", 2.2, None, 0),
                ],
                vec![
                    LootTable {
                        no_drop_weight: 10,
                        items: vec![(3, 9), (8, 6)],
                    },
                    LootTable {
                        no_drop_weight: 13,
                        items: vec![
                            (13, 1),
                            (14, 1),
                            (15, 1),
                            (22, 1),
                            (23, 1),
                            (24, 1),
                            (31, 1),
                            (32, 1),
                            (33, 1),
                        ],
                    },
                ],
            ),
        );
        enemies.insert(
            9,
            (
                EnemyStats {
                    id: 9,
                    name: "Battery Rat".to_string(),
                    description: "This enemy seems familiar...".to_string(),
                    element: Some(Electric),
                    next_phase: None,
                },
                Stats {
                    hp_max: 513,
                    hp: 513,
                    mp_max: 30,
                    mp: 30,
                    strength: 70,
                    wisdom: 70,
                    defense: 36,
                    level: 5,
                    gold: 300,
                    experience: 12500,
                    battle_sprite: image_assets.enemy10.clone(),
                },
                vec![
                    EnemyAttack::new("Thunder Punch", 2., None, 0),
                    EnemyAttack::new("Thunder Shock", 2., Some(Magic), 10),
                    EnemyAttack::new("Thunder Wave", 1.8, Some(Magic), 5),
                ],
                vec![
                    LootTable {
                        no_drop_weight: 10,
                        items: vec![(4, 9), (9, 6)],
                    },
                    LootTable {
                        no_drop_weight: 13,
                        items: vec![
                            (16, 1),
                            (17, 1),
                            (18, 1),
                            (25, 1),
                            (26, 1),
                            (27, 1),
                            (34, 1),
                            (35, 1),
                            (36, 1),
                        ],
                    },
                ],
            ),
        );
        enemies.insert(
            10,
            (
                EnemyStats {
                    id: 10,
                    name: "Penguin".to_string(),
                    description: "Noot Noot!".to_string(),
                    element: Some(Water),
                    next_phase: None,
                },
                Stats {
                    hp_max: 472,
                    hp: 472,
                    mp_max: 40,
                    mp: 40,
                    strength: 93,
                    wisdom: 68,
                    defense: 34,
                    level: 5,
                    gold: 350,
                    experience: 12500,
                    battle_sprite: image_assets.enemy11.clone(),
                },
                vec![
                    EnemyAttack::new("Water Gun", 2., Some(Magic), 8),
                    EnemyAttack::new("Doot Doot", 2., None, 0),
                    EnemyAttack::new("Peck", 1.8, None, 0),
                ],
                vec![
                    LootTable {
                        no_drop_weight: 10,
                        items: vec![(4, 9), (9, 6)],
                    },
                    LootTable {
                        no_drop_weight: 13,
                        items: vec![
                            (16, 1),
                            (17, 1),
                            (18, 1),
                            (25, 1),
                            (26, 1),
                            (27, 1),
                            (34, 1),
                            (35, 1),
                            (36, 1),
                        ],
                    },
                ],
            ),
        );
        enemies.insert(
            11,
            (
                EnemyStats {
                    id: 11,
                    name: "Emperor Penguin".to_string(),
                    description: "".to_string(),
                    element: Some(Fire),
                    next_phase: Some(13),
                },
                Stats {
                    hp_max: 576,
                    hp: 576,
                    mp_max: 100,
                    mp: 100,
                    strength: 92,
                    wisdom: 117,
                    defense: 45,
                    level: 5,
                    gold: 0,
                    experience: 0,
                    battle_sprite: image_assets.enemy12.clone(),
                },
                vec![
                    EnemyAttack::new("Wing", 2., None, 0),
                    EnemyAttack::new("Ignition", 2., Some(Magic), 5),
                    EnemyAttack::new("Peck", 2.2, None, 0),
                    EnemyAttack::new("Flamethrower", 2.2, Some(Magic), 12),
                ],
                vec![],
            ),
        );
        enemies.insert(
            12,
            (
                EnemyStats {
                    id: 12,
                    name: "Emperor Penguin".to_string(),
                    description: "".to_string(),
                    element: Some(Electric),
                    next_phase: Some(14),
                },
                Stats {
                    hp_max: 645,
                    hp: 645,
                    mp_max: 50,
                    mp: 50,
                    strength: 118,
                    wisdom: 90,
                    defense: 50,
                    level: 5,
                    gold: 0,
                    experience: 0,
                    battle_sprite: image_assets.enemy13.clone(),
                },
                vec![
                    EnemyAttack::new("Static Peck", 2., None, 0),
                    EnemyAttack::new("Shock Volt", 2., Some(Magic), 5),
                    EnemyAttack::new("Tesla Contact", 2.2, None, 0),
                    EnemyAttack::new("Monarch\'s Thunder", 2.2, Some(Magic), 7),
                ],
                vec![],
            ),
        );
        enemies.insert(
            13,
            (
                EnemyStats {
                    id: 13,
                    name: "Emperor Penguin".to_string(),
                    description: "".to_string(),
                    element: Some(Water),
                    next_phase: None,
                },
                Stats {
                    hp_max: 304,
                    hp: 304,
                    mp_max: 100,
                    mp: 100,
                    strength: 78,
                    wisdom: 80,
                    defense: 55,
                    level: 5,
                    gold: 1000000,
                    experience: 1000000,
                    battle_sprite: image_assets.enemy14.clone(),
                },
                vec![
                    EnemyAttack::new("Frigid Onslaught", 2.5, None, 0),
                    EnemyAttack::new("Tsunami", 2.5, Some(Magic), 7),
                    EnemyAttack::new("Royal Decree", 0.5, Some(Percentile), 30),
                ],
                vec![],
            ),
        );

        EnemyTable { table: enemies }
    }
}
