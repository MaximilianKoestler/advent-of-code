#[derive(Debug, Clone, Copy)]
pub enum ItemCategory {
    Weapon,
    Armor,
    Ring,
}

#[derive(Debug, Clone)]
pub struct ItemSpec {
    pub name: String,
    pub cost: u32,
    pub damage: u32,
    pub armor: u32,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub spec: ItemSpec,
    pub category: ItemCategory,
}

#[derive(Debug)]
pub struct EntityStats {
    pub hit_points: u32,
    pub damage: u32,
    pub armor: u32,
}

pub const ITEM_STRING: &str = r"
Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0

Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5

Rings:      Cost  Damage  Armor
Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3
";

pub mod parsers {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_until},
        character::complete::newline,
        combinator::{all_consuming, map},
        sequence::{terminated, tuple},
        IResult,
    };

    use super::{Item, ItemCategory, ItemSpec};

    fn item_category(input: &str) -> IResult<&str, ItemCategory> {
        alt((
            map(tag("Weapons"), |_| ItemCategory::Weapon),
            map(tag("Armor"), |_| ItemCategory::Armor),
            map(tag("Rings"), |_| ItemCategory::Ring),
        ))(input)
    }

    fn spaces(input: &str) -> IResult<&str, &str> {
        nom::character::complete::multispace0(input)
    }

    fn name(input: &str) -> IResult<&str, &str> {
        take_until("  ")(input)
    }

    fn number(input: &str) -> IResult<&str, u32> {
        nom::character::complete::u32(input)
    }

    fn header(input: &str) -> IResult<&str, ItemCategory> {
        let (input, (item_category, _, _, _, _, _, _, _, _)) = tuple((
            item_category,
            tag(":"),
            spaces,
            tag("Cost"),
            spaces,
            tag("Damage"),
            spaces,
            tag("Armor"),
            newline,
        ))(input)?;
        Ok((input, item_category))
    }

    fn item_spec(input: &str) -> IResult<&str, ItemSpec> {
        let (input, (name, _, cost, _, damage, _, armor, _)) = tuple((
            name, spaces, number, spaces, number, spaces, number, newline,
        ))(input)?;
        Ok((
            input,
            ItemSpec {
                name: name.to_string(),
                cost,
                damage,
                armor,
            },
        ))
    }

    /// Parse a block of items
    ///
    /// # Errors
    /// Returns a `nom::error:Error` if the input is not a valid block of items
    pub fn item_block(input: &str) -> IResult<&str, Vec<Item>> {
        let (input, _) = spaces(input)?;
        let (input, category) = header(input)?;
        let (input, item_specs) = terminated(nom::multi::many1(item_spec), spaces)(input)?;

        let items = item_specs
            .into_iter()
            .map(|spec| Item { spec, category })
            .collect();

        Ok((input, items))
    }

    /// Parse a multiple blocks of items into a single list
    ///
    /// # Errors
    /// Returns a `nom::error:Error` if the input is not a valid list of items
    pub fn items(input: &str) -> IResult<&str, Vec<Item>> {
        map(all_consuming(nom::multi::many1(item_block)), |i| i.concat())(input)
    }

    fn key(input: &str) -> IResult<&str, &str> {
        take_until(":")(input)
    }

    /// Parse a key-value pair
    ///
    /// # Errors
    /// Returns a `nom::error:Error` if the input is not a valid key-value pair
    pub fn key_value_line(input: &str) -> IResult<&str, (&str, u32)> {
        let (input, (key, _, value)) = tuple((key, tag(": "), number))(input)?;
        Ok((input, (key, value)))
    }
}

fn item_combinations() -> impl Iterator<Item = u32> {
    0..1 << 16
}

fn combination_is_valid(mask: u32, items: &[Item]) -> bool {
    let mut num_weapons = 0;
    let mut num_armor = 0;
    let mut num_rings = 0;

    for (i, item) in items.iter().enumerate() {
        if mask & (1 << i) != 0 {
            match item.category {
                ItemCategory::Weapon => num_weapons += 1,
                ItemCategory::Armor => num_armor += 1,
                ItemCategory::Ring => num_rings += 1,
            }
        }
    }

    num_weapons == 1 && num_armor <= 1 && num_rings <= 2
}

pub fn valid_item_combinations(items: &[Item]) -> impl Iterator<Item = u32> + '_ {
    item_combinations().filter(move |&mask| combination_is_valid(mask, items))
}

#[must_use]
pub fn get_stats(mask: u32, items: &[Item]) -> (u32, u32, u32) {
    let mut cost = 0;
    let mut damage = 0;
    let mut armor = 0;

    for (i, item) in items.iter().enumerate() {
        if mask & (1 << i) != 0 {
            cost += item.spec.cost;
            damage += item.spec.damage;
            armor += item.spec.armor;
        }
    }

    (cost, damage, armor)
}

#[must_use]
pub fn fight_result(player: &EntityStats, boss: &EntityStats) -> bool {
    let player_damage = player.damage.checked_sub(boss.armor).unwrap_or(1).max(1);
    let boss_damage = boss.damage.checked_sub(player.armor).unwrap_or(1).max(1);

    let player_turns = (boss.hit_points + player_damage - 1) / player_damage;
    let boss_turns = (player.hit_points + boss_damage - 1) / boss_damage;

    player_turns <= boss_turns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_parsing() {
        let (_, item_list) = parsers::items(ITEM_STRING).unwrap();
        assert_eq!(item_list.len(), 16);
    }

    #[test]
    fn test_fight_result() {
        let player = EntityStats {
            hit_points: 8,
            damage: 5,
            armor: 5,
        };
        let boss = EntityStats {
            hit_points: 12,
            damage: 7,
            armor: 2,
        };
        assert_eq!(fight_result(&player, &boss), true);
    }
}
