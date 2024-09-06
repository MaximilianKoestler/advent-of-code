use itertools::{repeat_n, Itertools};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerStats {
    pub hit_points: i32,
    pub mana: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BossStats {
    pub hit_points: i32,
    pub damage: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    stats: PlayerStats,
    armor: i32,
    shield_timer: i32,
    poison_timer: i32,
    recharge_timer: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Boss {
    stats: BossStats,
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    PlayerWins(i32),
    BossWins,
    Invalid,
    Undecided(Player, Boss, i32),
}

pub struct Rules {
    pub spell_costs: HashMap<Spell, i32>,
    pub start_of_turn_damage: i32,
}

const SPELLS: [Spell; 5] = [
    Spell::MagicMissile,
    Spell::Drain,
    Spell::Shield,
    Spell::Poison,
    Spell::Recharge,
];

fn spell_combinations(depth: usize) -> impl Iterator<Item = Vec<&'static Spell>> {
    repeat_n(SPELLS.iter(), depth)
        .multi_cartesian_product()
        .map(Vec::from_iter)
}

fn simulate_single(rules: &Rules, player: &Player, boss: &Boss, spell: &Spell) -> Outcome {
    let mut player = player.clone();
    let mut boss = boss.clone();

    player.stats.hit_points -= rules.start_of_turn_damage;
    if player.stats.hit_points <= 0 {
        return Outcome::BossWins;
    }

    // effects at start of player turn
    if player.recharge_timer > 0 {
        player.stats.mana += 101;
        player.recharge_timer -= 1;
    }
    if player.shield_timer > 0 {
        player.armor = 7;
        player.shield_timer -= 1;
    } else {
        player.armor = 0;
    }
    if player.poison_timer > 0 {
        boss.stats.hit_points -= 3;
        player.poison_timer -= 1;
    }

    let cost = rules.spell_costs.get(spell).unwrap();
    if cost > &player.stats.mana {
        return Outcome::BossWins;
    }
    player.stats.mana -= cost;

    match spell {
        Spell::MagicMissile => {
            boss.stats.hit_points -= 4;
        }
        Spell::Drain => {
            boss.stats.hit_points -= 2;
            player.stats.hit_points += 2;
        }
        Spell::Shield => {
            if player.shield_timer > 0 {
                return Outcome::Invalid;
            }
            player.shield_timer = 6;
        }
        Spell::Poison => {
            if player.poison_timer > 0 {
                return Outcome::Invalid;
            }
            player.poison_timer = 6;
        }
        Spell::Recharge => {
            if player.recharge_timer > 0 {
                return Outcome::Invalid;
            }
            player.recharge_timer = 5;
        }
    }

    // effects at start of boss turn
    if player.recharge_timer > 0 {
        player.stats.mana += 101;
        player.recharge_timer -= 1;
    }
    if player.shield_timer > 0 {
        player.armor = 7;
        player.shield_timer -= 1;
    } else {
        player.armor = 0;
    }
    if player.poison_timer > 0 {
        boss.stats.hit_points -= 3;
        player.poison_timer -= 1;
    }

    if boss.stats.hit_points <= 0 {
        return Outcome::PlayerWins(*cost);
    }

    let damage = (boss.stats.damage - player.armor).max(1);
    player.stats.hit_points -= damage;

    if player.stats.hit_points <= 0 {
        return Outcome::BossWins;
    }

    Outcome::Undecided(player, boss, *cost)
}

fn simulate(rules: &Rules, player: &PlayerStats, boss: &BossStats, spells: &[&Spell]) -> Outcome {
    let mut player = Player {
        stats: player.clone(),
        armor: 0,
        shield_timer: 0,
        poison_timer: 0,
        recharge_timer: 0,
    };

    let mut boss = Boss {
        stats: boss.clone(),
    };

    let mut total_cost = 0;
    for spell in spells {
        match simulate_single(rules, &player, &boss, spell) {
            Outcome::PlayerWins(cost) => {
                total_cost += cost;
                return Outcome::PlayerWins(total_cost);
            }
            Outcome::BossWins => {
                return Outcome::BossWins;
            }
            Outcome::Invalid => {
                return Outcome::Invalid;
            }
            Outcome::Undecided(new_player, new_boss, cost) => {
                player = new_player;
                boss = new_boss;
                total_cost += cost;
            }
        }
    }

    Outcome::Undecided(player, boss, total_cost)
}

#[must_use]
pub fn find_cheapest_win_exhaustive(
    rules: &Rules,
    player: &PlayerStats,
    boss: &BossStats,
    max_depth: usize,
) -> Option<i32> {
    // this takes a while to compute
    spell_combinations(max_depth)
        .map(|spells| simulate(rules, player, boss, &spells))
        .filter_map(|outcome| {
            if let Outcome::PlayerWins(cost) = outcome {
                Some(cost)
            } else {
                None
            }
        })
        .min()
}

#[must_use]
pub fn find_cheapest_spell_dfs(
    rules: &Rules,
    player: &PlayerStats,
    boss: &BossStats,
    max_depth: usize,
) -> Option<i32> {
    let player = Player {
        stats: player.clone(),
        armor: 0,
        shield_timer: 0,
        poison_timer: 0,
        recharge_timer: 0,
    };

    let boss = Boss {
        stats: boss.clone(),
    };

    let mut stack = Vec::new();
    for spell in SPELLS {
        stack.push((player.clone(), boss.clone(), spell, 0, 1));
    }

    let mut best_result: Option<i32> = None;
    while let Some((player, boss, spell, cost, depth)) = stack.pop() {
        match simulate_single(rules, &player, &boss, &spell) {
            Outcome::PlayerWins(new_cost) => {
                let cost = cost + new_cost;
                best_result = best_result.map(|old| old.min(cost)).or(Some(cost));
            }
            Outcome::Undecided(player, boss, new_cost)
                if depth < max_depth && (best_result.is_none() || Some(new_cost) < best_result) =>
            {
                for spell in SPELLS {
                    stack.push((
                        player.clone(),
                        boss.clone(),
                        spell,
                        cost + new_cost,
                        depth + 1,
                    ));
                }
            }
            _ => (),
        }
    }
    best_result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rules() -> Rules {
        let mut spell_costs = HashMap::new();
        spell_costs.insert(Spell::MagicMissile, 53);
        spell_costs.insert(Spell::Drain, 73);
        spell_costs.insert(Spell::Shield, 113);
        spell_costs.insert(Spell::Poison, 173);
        spell_costs.insert(Spell::Recharge, 229);

        Rules {
            spell_costs,
            start_of_turn_damage: 0,
        }
    }

    #[test]
    fn test_spell_combinations_1() {
        let spells: Vec<_> = spell_combinations(1).collect();
        assert_eq!(spells.len(), 5);

        let expected = vec![
            vec![&Spell::MagicMissile],
            vec![&Spell::Drain],
            vec![&Spell::Shield],
            vec![&Spell::Poison],
            vec![&Spell::Recharge],
        ];
        assert_eq!(spells, expected);
    }

    #[test]
    fn test_spell_combinations_2() {
        let spells: Vec<_> = spell_combinations(2).collect();
        assert_eq!(spells.len(), 25);

        let expected = vec![
            vec![&Spell::MagicMissile, &Spell::MagicMissile],
            vec![&Spell::MagicMissile, &Spell::Drain],
            vec![&Spell::MagicMissile, &Spell::Shield],
            vec![&Spell::MagicMissile, &Spell::Poison],
            vec![&Spell::MagicMissile, &Spell::Recharge],
            vec![&Spell::Drain, &Spell::MagicMissile],
            vec![&Spell::Drain, &Spell::Drain],
            vec![&Spell::Drain, &Spell::Shield],
            vec![&Spell::Drain, &Spell::Poison],
            vec![&Spell::Drain, &Spell::Recharge],
            vec![&Spell::Shield, &Spell::MagicMissile],
            vec![&Spell::Shield, &Spell::Drain],
            vec![&Spell::Shield, &Spell::Shield],
            vec![&Spell::Shield, &Spell::Poison],
            vec![&Spell::Shield, &Spell::Recharge],
            vec![&Spell::Poison, &Spell::MagicMissile],
            vec![&Spell::Poison, &Spell::Drain],
            vec![&Spell::Poison, &Spell::Shield],
            vec![&Spell::Poison, &Spell::Poison],
            vec![&Spell::Poison, &Spell::Recharge],
            vec![&Spell::Recharge, &Spell::MagicMissile],
            vec![&Spell::Recharge, &Spell::Drain],
            vec![&Spell::Recharge, &Spell::Shield],
            vec![&Spell::Recharge, &Spell::Poison],
            vec![&Spell::Recharge, &Spell::Recharge],
        ];
        assert_eq!(spells, expected);
    }

    #[test]
    fn test_simulate_player_wins() {
        let player = PlayerStats {
            hit_points: 1000,
            mana: 1000,
        };
        let boss = BossStats {
            hit_points: 4,
            damage: 1,
        };
        let spells = vec![&Spell::MagicMissile];
        assert_eq!(
            simulate(&rules(), &player, &boss, &spells),
            Outcome::PlayerWins(53)
        );
    }
    #[test]
    fn test_simulate_undecided() {
        let player = PlayerStats {
            hit_points: 1000,
            mana: 1000,
        };
        let boss = BossStats {
            hit_points: 5,
            damage: 1,
        };
        let spells = vec![&Spell::MagicMissile];
        assert_eq!(
            simulate(&rules(), &player, &boss, &spells),
            Outcome::Undecided(
                Player {
                    stats: PlayerStats {
                        hit_points: 999,
                        mana: 947
                    },
                    armor: 0,
                    shield_timer: 0,
                    poison_timer: 0,
                    recharge_timer: 0
                },
                Boss {
                    stats: BossStats {
                        hit_points: 1,
                        damage: 1
                    }
                },
                53
            )
        );
    }

    #[test]
    fn test_simulate_boss_wins() {
        let player = PlayerStats {
            hit_points: 10,
            mana: 1000,
        };
        let boss = BossStats {
            hit_points: 1000,
            damage: 10,
        };
        let spells = vec![&Spell::MagicMissile];
        assert_eq!(
            simulate(&rules(), &player, &boss, &spells),
            Outcome::BossWins
        );
    }

    #[test]
    fn test_simulate_invalid() {
        let player = PlayerStats {
            hit_points: 1000,
            mana: 1000,
        };
        let boss = BossStats {
            hit_points: 1000,
            damage: 1,
        };
        let spells = vec![&Spell::Shield, &Spell::Shield];
        assert_eq!(
            simulate(&rules(), &player, &boss, &spells),
            Outcome::Invalid
        );
    }

    #[test]
    fn test_simulate_example() {
        let player = PlayerStats {
            hit_points: 10,
            mana: 250,
        };
        let boss = BossStats {
            hit_points: 13,
            damage: 8,
        };

        let mut player = Player {
            stats: player.clone(),
            armor: 0,
            shield_timer: 0,
            poison_timer: 0,
            recharge_timer: 0,
        };

        let mut boss = Boss {
            stats: boss.clone(),
        };

        if let Outcome::Undecided(p, b, cost) =
            simulate_single(&rules(), &player, &boss, &Spell::Poison)
        {
            player = p;
            boss = b;

            assert_eq!(player.stats.hit_points, 2);
            assert_eq!(player.stats.mana, 77);
            assert_eq!(boss.stats.hit_points, 10);
            assert_eq!(cost, 173)
        } else {
            panic!();
        }

        assert_eq!(
            simulate_single(&rules(), &player, &boss, &Spell::MagicMissile),
            Outcome::PlayerWins(53)
        )
    }
}
