use bevy::{prelude::*, utils::HashMap};

use crate::global::{Item, ItemStats, ItemTable, ItemType::*};

impl FromWorld for ItemTable {
    fn from_world(_: &mut World) -> Self {
        let mut map = HashMap::new();
        map.insert(
            0,
            Item::new(0, "Red Potion I", Consumable, ItemStats::new_potion(20, 0)),
        );
        map.insert(
            1,
            Item::new(1, "Red Potion II", Consumable, ItemStats::new_potion(40, 0)),
        );
        map.insert(
            2,
            Item::new(
                2,
                "Red Potion III",
                Consumable,
                ItemStats::new_potion(80, 0),
            ),
        );
        map.insert(
            3,
            Item::new(
                3,
                "Red Potion IV",
                Consumable,
                ItemStats::new_potion(160, 0),
            ),
        );
        map.insert(
            4,
            Item::new(4, "Red Potion V", Consumable, ItemStats::new_potion(320, 0)),
        );
        map.insert(
            5,
            Item::new(5, "Blue Potion I", Consumable, ItemStats::new_potion(0, 15)),
        );
        map.insert(
            6,
            Item::new(
                6,
                "Blue Potion II",
                Consumable,
                ItemStats::new_potion(0, 30),
            ),
        );
        map.insert(
            7,
            Item::new(
                7,
                "Blue Potion III",
                Consumable,
                ItemStats::new_potion(0, 60),
            ),
        );
        map.insert(
            8,
            Item::new(
                8,
                "Blue Potion IV",
                Consumable,
                ItemStats::new_potion(0, 120),
            ),
        );
        map.insert(
            9,
            Item::new(
                9,
                "Blue Potion V",
                Consumable,
                ItemStats::new_potion(0, 240),
            ),
        );

        map.insert(
            10,
            Item::new(
                10,
                "Steel Sword",
                Weapon,
                ItemStats::new_eqiup(0, 0, 20, 0, 0),
            ),
        );
        map.insert(
            11,
            Item::new(
                11,
                "Steel Shield",
                Weapon,
                ItemStats::new_eqiup(25, 0, 5, 0, 6),
            ),
        );
        map.insert(
            12,
            Item::new(
                12,
                "Amethyst Wand",
                Weapon,
                ItemStats::new_eqiup(0, 0, 2, 10, 0),
            ),
        );
        map.insert(
            13,
            Item::new(
                13,
                "Doomblade",
                Weapon,
                ItemStats::new_eqiup(0, 0, 35, 0, 0),
            ),
        );
        map.insert(
            14,
            Item::new(
                14,
                "Obsidian Barrier",
                Weapon,
                ItemStats::new_eqiup(50, 0, 10, 0, 9),
            ),
        );
        map.insert(
            15,
            Item::new(
                15,
                "Ancient Staff",
                Weapon,
                ItemStats::new_eqiup(0, 0, 4, 20, 0),
            ),
        );
        map.insert(
            16,
            Item::new(
                16,
                "Cataclysm",
                Weapon,
                ItemStats::new_eqiup(0, 0, 50, 0, 0),
            ),
        );
        map.insert(
            17,
            Item::new(
                17,
                "PeaceKeeper",
                Weapon,
                ItemStats::new_eqiup(75, 0, 15, 0, 12),
            ),
        );
        map.insert(
            18,
            Item::new(
                18,
                "Blind Justice",
                Weapon,
                ItemStats::new_eqiup(0, 0, 6, 30, 0),
            ),
        );

        map.insert(
            19,
            Item::new(
                19,
                "Cactus Armor",
                Armor,
                ItemStats::new_eqiup(10, 0, 10, 0, 3),
            ),
        );
        map.insert(
            20,
            Item::new(
                20,
                "Steel Armor",
                Armor,
                ItemStats::new_eqiup(50, 0, 0, 0, 9),
            ),
        );
        map.insert(
            21,
            Item::new(
                21,
                "Enchanted Robes",
                Armor,
                ItemStats::new_eqiup(5, 0, 0, 10, 2),
            ),
        );
        map.insert(
            22,
            Item::new(
                22,
                "Spiked Armor",
                Armor,
                ItemStats::new_eqiup(20, 0, 20, 0, 6),
            ),
        );
        map.insert(
            23,
            Item::new(
                23,
                "Mythril Armor",
                Armor,
                ItemStats::new_eqiup(100, 0, 0, 0, 18),
            ),
        );
        map.insert(
            24,
            Item::new(
                24,
                "Spectre Robes",
                Armor,
                ItemStats::new_eqiup(10, 0, 0, 20, 4),
            ),
        );
        map.insert(
            25,
            Item::new(
                25,
                "Gladiator Armor",
                Armor,
                ItemStats::new_eqiup(40, 0, 30, 0, 9),
            ),
        );
        map.insert(
            26,
            Item::new(
                26,
                "Titan Armor",
                Armor,
                ItemStats::new_eqiup(200, 0, 0, 0, 36),
            ),
        );
        map.insert(
            27,
            Item::new(
                27,
                "Ethereal Robes",
                Armor,
                ItemStats::new_eqiup(15, 0, 0, 30, 6),
            ),
        );

        map.insert(
            28,
            Item::new(
                28,
                "Power Ring",
                Accessory,
                ItemStats::new_eqiup(0, 0, 10, 2, 0),
            ),
        );
        map.insert(
            29,
            Item::new(
                29,
                "Hard Bracelet",
                Accessory,
                ItemStats::new_eqiup(25, 0, 0, 5, 6),
            ),
        );
        map.insert(
            30,
            Item::new(
                30,
                "Wise Necklace",
                Accessory,
                ItemStats::new_eqiup(0, 0, 0, 20, 0),
            ),
        );
        map.insert(
            31,
            Item::new(
                31,
                "Mighty Ring",
                Accessory,
                ItemStats::new_eqiup(0, 0, 20, 4, 0),
            ),
        );
        map.insert(
            32,
            Item::new(
                32,
                "Resistant Bracelet",
                Accessory,
                ItemStats::new_eqiup(50, 0, 0, 10, 9),
            ),
        );
        map.insert(
            33,
            Item::new(
                33,
                "Sagacious Necklace",
                Accessory,
                ItemStats::new_eqiup(0, 0, 0, 35, 0),
            ),
        );
        map.insert(
            34,
            Item::new(
                34,
                "Ultimate Ring",
                Accessory,
                ItemStats::new_eqiup(0, 0, 30, 6, 0),
            ),
        );
        map.insert(
            35,
            Item::new(
                35,
                "Ultimate Bracelet",
                Accessory,
                ItemStats::new_eqiup(75, 0, 0, 15, 12),
            ),
        );
        map.insert(
            36,
            Item::new(
                36,
                "Ultimate Necklace",
                Accessory,
                ItemStats::new_eqiup(0, 0, 0, 50, 0),
            ),
        );

        ItemTable(map)
    }
}
