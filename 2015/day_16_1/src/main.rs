//! Advent of code 2015 day 15 part 1

use std::collections::HashMap;
use std::io::BufRead;

#[derive(Debug)]
struct MfcsamRecord {
    name: String,
    properties: HashMap<String, i32>,
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

    fn matches(&self, other: &Self) -> bool {
        self.properties
            .iter()
            .all(|(key, value)| other.properties.get(key).map_or(true, |v| v == value))
    }
}

fn main() {
    let sample = MfcsamRecord {
        name: String::new(),
        properties: [
            ("children".to_string(), 3),
            ("cats".to_string(), 7),
            ("samoyeds".to_string(), 2),
            ("pomeranians".to_string(), 3),
            ("akitas".to_string(), 0),
            ("vizlsas".to_string(), 0),
            ("goldfish".to_string(), 5),
            ("trees".to_string(), 3),
            ("cars".to_string(), 2),
            ("perfumes".to_string(), 1),
        ]
        .into_iter()
        .collect(),
    };

    let file = std::fs::File::open("input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let matching_sue = reader
        .lines()
        .map(Result::unwrap)
        .map(|s| MfcsamRecord::from_str(&s))
        .find(|sue| sample.matches(sue))
        .unwrap();

    println!("{}", matching_sue.name);
}
