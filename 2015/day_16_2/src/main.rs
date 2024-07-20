//! Advent of code 2015 day 15 part 1

use std::collections::HashMap;
use std::io::BufRead;

enum Comparison {
    Equal,
    Less,
    Greater,
}

#[derive(Debug)]
struct MfcsamRecord {
    name: String,
    properties: HashMap<String, i32>,
}

struct McfsamSample {
    properties: HashMap<String, (i32, Comparison)>,
}

impl MfcsamRecord {
    fn from_str(s: &str) -> Self {
        let mut parts = s.splitn(2, ": ");
        let name = parts.next().unwrap().to_string();
        let properties = parts
            .next()
            .unwrap()
            .split(", ")
            .map(|s| {
                let mut parts = s.splitn(2, ": ");
                let key = parts.next().unwrap();
                let value = parts.next().unwrap().parse().unwrap();
                (key.to_string(), value)
            })
            .collect();
        Self { name, properties }
    }

    fn matches(&self, other: &McfsamSample) -> bool {
        self.properties.iter().all(|(key, value)| {
            other.properties.get(key).map_or(true, |(v, c)| match c {
                Comparison::Equal => value == v,
                Comparison::Less => value < v,
                Comparison::Greater => value > v,
            })
        })
    }
}

fn main() {
    let sample = McfsamSample {
        properties: [
            ("children".to_string(), (3, Comparison::Equal)),
            ("cats".to_string(), (7, Comparison::Greater)),
            ("samoyeds".to_string(), (2, Comparison::Equal)),
            ("pomeranians".to_string(), (3, Comparison::Less)),
            ("akitas".to_string(), (0, Comparison::Equal)),
            ("vizlsas".to_string(), (0, Comparison::Equal)),
            ("goldfish".to_string(), (5, Comparison::Less)),
            ("trees".to_string(), (3, Comparison::Greater)),
            ("cars".to_string(), (2, Comparison::Equal)),
            ("perfumes".to_string(), (1, Comparison::Equal)),
        ]
        .into_iter()
        .collect(),
    };

    let file = std::fs::File::open("../day_16_1/input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let matching_sue = reader
        .lines()
        .map(Result::unwrap)
        .map(|s| MfcsamRecord::from_str(&s))
        .find(|sue| sue.matches(&sample))
        .unwrap();

    println!("{}", matching_sue.name);
}
