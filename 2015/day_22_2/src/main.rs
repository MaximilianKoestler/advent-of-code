//! Advent of code 2015 day 22 part 2

use std::collections::HashMap;

use day_22_1::{find_cheapest_spell_dfs, BossStats, PlayerStats, Rules, Spell};

fn rules() -> Rules {
    let mut spell_costs = HashMap::new();
    spell_costs.insert(Spell::MagicMissile, 53);
    spell_costs.insert(Spell::Drain, 73);
    spell_costs.insert(Spell::Shield, 113);
    spell_costs.insert(Spell::Poison, 173);
    spell_costs.insert(Spell::Recharge, 229);

    Rules {
        spell_costs,
        start_of_turn_damage: 1,
    }
}

fn main() {
    let player = PlayerStats {
        hit_points: 50,
        mana: 500,
    };
    let boss = BossStats {
        hit_points: 51,
        damage: 9,
    };

    let min_mana = find_cheapest_spell_dfs(&rules(), &player, &boss, 8).unwrap();
    println!("Minimum mana: {min_mana}");
}
