//! Advent of code 2015 day 9 part 1

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

#[derive(Clone, Debug, PartialEq)]
struct LocationPair {
    from: String,
    to: String,
    distance: u64,
}

mod parsers {
    use nom::{
        bytes::complete::tag, character::complete::alpha1, combinator::map,
        sequence::separated_pair, IResult,
    };

    fn city(input: &str) -> IResult<&str, &str> {
        alpha1(input)
    }

    fn distance(input: &str) -> IResult<&str, u64> {
        nom::character::complete::u64(input)
    }

    pub fn location_pair(input: &str) -> IResult<&str, super::LocationPair> {
        map(
            separated_pair(
                separated_pair(city, tag(" to "), city),
                tag(" = "),
                distance,
            ),
            |((from, to), distance)| super::LocationPair {
                from: from.to_string(),
                to: to.to_string(),
                distance,
            },
        )(input)
    }
}

impl<'a> TryFrom<&'a str> for LocationPair {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        parsers::location_pair(input).map(|(_, c)| c)
    }
}

fn build_index(locations: &[LocationPair]) -> (Vec<String>, HashMap<String, usize>) {
    let index_to_city: HashSet<_> = locations
        .iter()
        .flat_map(|l| vec![l.from.clone(), l.to.clone()])
        .collect();
    let mut index_to_city: Vec<_> = index_to_city.into_iter().collect();
    index_to_city.sort_unstable();

    let city_to_index: HashMap<_, _> = index_to_city
        .iter()
        .enumerate()
        .map(|(i, city)| (city.clone(), i))
        .collect();

    (index_to_city, city_to_index)
}

fn build_distance_matrix(
    locations: &[LocationPair],
    city_to_index: &HashMap<String, usize>,
) -> Vec<Vec<u64>> {
    let mut matrix = vec![vec![0; city_to_index.len()]; city_to_index.len()];

    for location in locations {
        let from = city_to_index[&location.from];
        let to = city_to_index[&location.to];
        matrix[from][to] = location.distance;
        matrix[to][from] = location.distance;
    }

    matrix
}

fn solve_traveling_salesman(matrix: &[Vec<u64>]) -> (u64, Vec<usize>) {
    (0..matrix.len())
        .permutations(matrix.len())
        .map(|path| {
            (
                path.windows(2)
                    .map(|pair| matrix[pair[0]][pair[1]])
                    .sum::<u64>(),
                path.clone(),
            )
        })
        .min_by_key(|(distance, _)| *distance)
        .unwrap()
}

fn main() {
    let file = std::fs::File::open("input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let locations: Vec<LocationPair> = reader
        .lines()
        .map(Result::unwrap)
        .map(|s| LocationPair::try_from(&*s).unwrap())
        .collect();

    let (index_to_city, city_to_index) = build_index(&locations);
    let matrix = build_distance_matrix(&locations, &city_to_index);

    let (distance, path) = solve_traveling_salesman(&matrix);
    println!("Shortest distance: {distance}");

    let path: Vec<_> = path.iter().map(|i| index_to_city[*i].clone()).collect();
    println!("Path: {path:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_locations() -> Vec<LocationPair> {
        vec![
            LocationPair {
                from: "London".to_string(),
                to: "Dublin".to_string(),
                distance: 464,
            },
            LocationPair {
                from: "London".to_string(),
                to: "Belfast".to_string(),
                distance: 518,
            },
            LocationPair {
                from: "Dublin".to_string(),
                to: "Belfast".to_string(),
                distance: 141,
            },
        ]
    }

    #[test]
    fn test_location_pair() {
        let locations = get_test_locations();
        assert_eq!(
            LocationPair::try_from("London to Dublin = 464"),
            Ok(locations[0].clone())
        );
        assert_eq!(
            LocationPair::try_from("London to Belfast = 518"),
            Ok(locations[1].clone())
        );
        assert_eq!(
            LocationPair::try_from("Dublin to Belfast = 141"),
            Ok(locations[2].clone())
        );
    }

    #[test]
    fn test_build_index() {
        let (index_to_city, city_to_index) = build_index(&get_test_locations());
        assert_eq!(index_to_city, vec!["Belfast", "Dublin", "London"]);
        assert_eq!(city_to_index.len(), index_to_city.len());
        assert_eq!(city_to_index["Belfast"], 0);
        assert_eq!(city_to_index["Dublin"], 1);
        assert_eq!(city_to_index["London"], 2);
    }

    #[test]
    fn test_build_distance_matrix() {
        let locations = get_test_locations();
        let (_, city_to_index) = build_index(&locations);
        let matrix = build_distance_matrix(&locations, &city_to_index);
        assert_eq!(
            matrix,
            vec![vec![0, 141, 518], vec![141, 0, 464], vec![518, 464, 0]]
        );
    }

    #[test]
    fn test_solve_traveling_salesman() {
        let locations = get_test_locations();
        let (_, city_to_index) = build_index(&locations);
        let matrix = build_distance_matrix(&locations, &city_to_index);
        let (distance, path) = solve_traveling_salesman(&matrix);
        assert_eq!(distance, 605);

        let expected_path = vec![2, 1, 0];
        let expected_path_reversed: Vec<_> = expected_path.iter().rev().copied().collect();
        assert!(path == expected_path || path == expected_path_reversed);
    }
}
