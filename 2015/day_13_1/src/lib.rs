use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Name(pub String);

impl From<&str> for Name {
    fn from(s: &str) -> Self {
        Name(s.to_string())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    Gain,
    Lose,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Information {
    person: Name,
    action: Action,
    happiness: u32,
    neighbor: Name,
}

mod parsers {
    use nom::{
        branch::alt, bytes::complete::tag, character::complete::alpha1, combinator::map,
        sequence::terminated, sequence::tuple, IResult,
    };

    use super::{Action, Information, Name};

    fn name(input: &str) -> IResult<&str, Name> {
        map(alpha1, |name: &str| Name(name.to_string()))(input)
    }

    fn action_gain(input: &str) -> IResult<&str, Action> {
        map(tag("gain"), |_| Action::Gain)(input)
    }

    fn action_lose(input: &str) -> IResult<&str, Action> {
        map(tag("lose"), |_| Action::Lose)(input)
    }

    fn action(input: &str) -> IResult<&str, Action> {
        alt((action_gain, action_lose))(input)
    }

    fn happiness(input: &str) -> IResult<&str, u32> {
        terminated(nom::character::complete::u32, tag(" happiness units"))(input)
    }

    pub fn information(input: &str) -> IResult<&str, Information> {
        let (input, (person, _, action, _, happiness, _, neighbor)) = tuple((
            name,
            tag(" would "),
            action,
            tag(" "),
            happiness,
            tag(" by sitting next to "),
            name,
        ))(input)?;
        Ok((
            input,
            Information {
                person,
                action,
                happiness,
                neighbor,
            },
        ))
    }
}

impl<'a> TryFrom<&'a str> for Information {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        parsers::information(input).map(|(_, c)| c)
    }
}

#[must_use]
pub fn build_name_map(entries: &[Information]) -> HashMap<Name, usize> {
    entries
        .iter()
        .flat_map(|info| vec![info.person.clone(), info.neighbor.clone()])
        .unique()
        .sorted()
        .enumerate()
        .map(|(i, name)| (name, i))
        .collect()
}

#[must_use]
pub fn build_neighborhood_matrix<S: ::std::hash::BuildHasher>(
    entries: &[Information],
    name_map: &HashMap<Name, usize, S>,
) -> Vec<Vec<Option<i32>>> {
    let mut matrix = vec![vec![None; name_map.len()]; name_map.len()];
    for info in entries {
        let happiness = match info.action {
            Action::Gain => i32::try_from(info.happiness),
            Action::Lose => i32::try_from(info.happiness).map(|h| -h),
        };
        if let Ok(happiness) = happiness {
            let person = name_map[&info.person];
            let neighbor = name_map[&info.neighbor];
            matrix[person][neighbor] = Some(happiness);
        }
    }
    matrix
}

#[must_use]
pub fn compute_optimal_seating(matrix: &[Vec<Option<i32>>]) -> Option<(Vec<usize>, i32)> {
    (0..matrix.len())
        .permutations(matrix.len())
        .map(|seating| {
            (
                seating.clone(),
                seating
                    .iter()
                    .zip(seating.iter().skip(1).chain(seating.iter().take(1)))
                    .map(|(&person, &neighbor)| (person, neighbor))
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(seating, pairs)| {
            (
                seating,
                pairs
                    .into_iter()
                    .map(|(person, neighbor)| {
                        matrix[person][neighbor].unwrap_or(0)
                            + matrix[neighbor][person].unwrap_or(0)
                    })
                    .sum::<i32>(),
            )
        })
        .max_by_key(|(_, happiness)| *happiness)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_information() {
        assert_eq!(
            Information::try_from("Alice would gain 54 happiness units by sitting next to Bob"),
            Ok(Information {
                person: Name("Alice".to_string()),
                action: Action::Gain,
                happiness: 54,
                neighbor: Name("Bob".to_string())
            })
        );
        assert_eq!(
            Information::try_from("Bob would lose 79 happiness units by sitting next to Carol"),
            Ok(Information {
                person: Name("Bob".to_string()),
                action: Action::Lose,
                happiness: 79,
                neighbor: Name("Carol".to_string())
            })
        );
    }

    #[test]
    fn test_build_name_map() {
        let entries = vec![
            Information {
                person: Name("Alice".to_string()),
                action: Action::Gain,
                happiness: 54,
                neighbor: Name("Bob".to_string()),
            },
            Information {
                person: Name("Bob".to_string()),
                action: Action::Lose,
                happiness: 79,
                neighbor: Name("Carol".to_string()),
            },
        ];
        let name_map = build_name_map(&entries);
        assert_eq!(name_map.len(), 3);
        assert!(name_map.contains_key(&Name("Alice".to_string())));
        assert!(name_map.contains_key(&Name("Bob".to_string())));
        assert!(name_map.contains_key(&Name("Carol".to_string())));
        assert_eq!(
            name_map.values().cloned().collect::<HashSet<_>>(),
            vec![0, 1, 2].into_iter().collect()
        );
    }

    #[test]
    fn test_build_neighborhood_matrix() {
        let entries = vec![
            Information {
                person: Name("Alice".to_string()),
                action: Action::Gain,
                happiness: 54,
                neighbor: Name("Bob".to_string()),
            },
            Information {
                person: Name("Bob".to_string()),
                action: Action::Lose,
                happiness: 79,
                neighbor: Name("Carol".to_string()),
            },
        ];
        let name_map = build_name_map(&entries);
        let matrix = build_neighborhood_matrix(&entries, &name_map);
        assert_eq!(matrix.len(), 3);
        assert_eq!(
            matrix[name_map[&("Alice".into())]][name_map[&("Bob".into())]],
            Some(54)
        );
        assert_eq!(
            matrix[name_map[&("Bob".into())]][name_map[&("Carol".into())]],
            Some(-79)
        );
        assert_eq!(
            matrix[name_map[&("Carol".into())]][name_map[&("Alice".into())]],
            None
        );
    }

    #[test]
    fn test_full_algorithm() {
        let input = vec![
            "Alice would gain 54 happiness units by sitting next to Bob.",
            "Alice would lose 79 happiness units by sitting next to Carol.",
            "Alice would lose 2 happiness units by sitting next to David.",
            "Bob would gain 83 happiness units by sitting next to Alice.",
            "Bob would lose 7 happiness units by sitting next to Carol.",
            "Bob would lose 63 happiness units by sitting next to David.",
            "Carol would lose 62 happiness units by sitting next to Alice.",
            "Carol would gain 60 happiness units by sitting next to Bob.",
            "Carol would gain 55 happiness units by sitting next to David.",
            "David would gain 46 happiness units by sitting next to Alice.",
            "David would lose 7 happiness units by sitting next to Bob.",
            "David would gain 41 happiness units by sitting next to Carol",
        ];
        let entries = input
            .iter()
            .map(|&line| Information::try_from(line).unwrap())
            .collect::<Vec<_>>();

        let name_map = build_name_map(&entries);
        let matrix = build_neighborhood_matrix(&entries, &name_map);
        let (_, happiness) = compute_optimal_seating(&matrix).unwrap();
        assert_eq!(happiness, 330);
    }
}
