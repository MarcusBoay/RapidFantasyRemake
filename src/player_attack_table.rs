use bevy::{prelude::*, utils::HashMap};

use crate::global::{Element::*, PlayerAttack, PlayerAttackTable, PlayerAttackType::*};

impl FromWorld for PlayerAttackTable {
    fn from_world(_: &mut World) -> Self {
        let mut attacks = HashMap::new();

        attacks.insert(0, PlayerAttack::new(0, "Tackle", None, None, 0, 1));

        attacks.insert(
            1,
            PlayerAttack::new(1, "Sonic Spike", Some(Limit), None, 0, 1),
        );
        attacks.insert(
            2,
            PlayerAttack::new(2, "Sword Dance", Some(Limit), None, 0, 2),
        );
        attacks.insert(
            3,
            PlayerAttack::new(3, "Oblivion Strike", Some(Limit), None, 0, 3),
        );

        attacks.insert(
            4,
            PlayerAttack::new(4, "Fire Ball", Some(Magic), Some(Fire), 10, 1),
        );
        attacks.insert(
            5,
            PlayerAttack::new(5, "Bubble Beam", Some(Magic), Some(Water), 10, 1),
        );
        attacks.insert(
            6,
            PlayerAttack::new(6, "Lightning Bolt", Some(Magic), Some(Electric), 10, 1),
        );
        attacks.insert(
            7,
            PlayerAttack::new(7, "Stone Edge", Some(Magic), Some(Earth), 10, 1),
        );
        attacks.insert(
            8,
            PlayerAttack::new(8, "Holy Light", Some(Magic), Some(Light), 10, 1),
        );
        attacks.insert(
            9,
            PlayerAttack::new(9, "Dark Spear", Some(Magic), Some(Dark), 10, 1),
        );

        attacks.insert(
            10,
            PlayerAttack::new(10, "Red Blaze", Some(Magic), Some(Fire), 40, 2),
        );
        attacks.insert(
            11,
            PlayerAttack::new(11, "Waterfall", Some(Magic), Some(Water), 40, 2),
        );
        attacks.insert(
            12,
            PlayerAttack::new(12, "Electrocute", Some(Magic), Some(Electric), 40, 2),
        );
        attacks.insert(
            13,
            PlayerAttack::new(13, "Landslide", Some(Magic), Some(Earth), 40, 2),
        );
        attacks.insert(
            14,
            PlayerAttack::new(14, "Piercing Light", Some(Magic), Some(Light), 40, 2),
        );
        attacks.insert(
            15,
            PlayerAttack::new(15, "Pitch Black", Some(Magic), Some(Dark), 40, 2),
        );

        attacks.insert(
            16,
            PlayerAttack::new(16, "Inferno", Some(Magic), Some(Fire), 75, 3),
        );
        attacks.insert(
            17,
            PlayerAttack::new(17, "Tsunami", Some(Magic), Some(Water), 75, 3),
        );
        attacks.insert(
            18,
            PlayerAttack::new(18, "Plasma", Some(Magic), Some(Electric), 75, 3),
        );
        attacks.insert(
            19,
            PlayerAttack::new(19, "Earthquake", Some(Magic), Some(Earth), 75, 3),
        );
        attacks.insert(
            20,
            PlayerAttack::new(20, "Genesis", Some(Magic), Some(Light), 75, 3),
        );
        attacks.insert(
            21,
            PlayerAttack::new(21, "Blackhole", Some(Magic), Some(Dark), 75, 3),
        );

        PlayerAttackTable { table: attacks }
    }
}
