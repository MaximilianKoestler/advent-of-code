//! Advent of code 2015 day 21 part 2

use std::{collections::HashMap, io::BufRead};

use day_21_1::{
    fight_result, get_stats, parsers, valid_item_combinations, EntityStats, ITEM_STRING,
};

fn main() {
    let file = std::fs::File::open("../day_21_1/input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let boss_config: HashMap<_, _> = reader
        .lines()
        .map(|line| {
            parsers::key_value_line(&line.unwrap())
                .map(|(_, (k, v))| (k.to_string(), v))
                .unwrap()
        })
        .collect();

    let boss = EntityStats {
        hit_points: boss_config["Hit Points"],
        damage: boss_config["Damage"],
        armor: boss_config["Armor"],
    };

    println!("Boss: {boss:?}");

    let (_, items) = parsers::items(ITEM_STRING).unwrap();

    let (mask, (cost, _, _)) = valid_item_combinations(&items)
        .map(|mask| (mask, get_stats(mask, &items)))
        .filter(|(_, (_, damage, armor))| {
            !fight_result(
                &EntityStats {
                    hit_points: 100,
                    damage: *damage,
                    armor: *armor,
                },
                &boss,
            )
        })
        .max_by_key(|(_, (cost, _, _))| *cost)
        .unwrap();

    println!("Maximum cost: {cost}");
    println!("Items:");
    for (i, item) in items.iter().enumerate() {
        if mask & (1 << i) != 0 {
            println!("  - {}", item.spec.name);
        }
    }
}
