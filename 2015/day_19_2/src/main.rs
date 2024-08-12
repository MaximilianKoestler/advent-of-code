//! Advent of code 2015 day 19 part 2

use std::collections::HashSet;
use std::io::BufRead;
use std::{collections::HashMap, vec};

use day_19_1::{parse_replacement, tokenize_molecule, Replacement};

fn bfs(molecule: &[String], replacements: &[Replacement]) -> usize {
    let mut replacement_map = HashMap::new();
    for replacement in replacements {
        replacement_map
            .entry(&replacement.original)
            .or_insert(Vec::new())
            .push(tokenize_molecule(&replacement.replacement));
    }

    let mut stack = HashMap::new();
    let options: HashSet<_> = vec![vec!["e".to_string()]].into_iter().collect();
    stack.insert(0, options);
    let mut current_depth = 0;
    loop {
        let candidates = stack.get(&current_depth).unwrap().clone();
        for candidate in candidates {
            for (i, token) in candidate.iter().cloned().enumerate() {
                if replacement_map.contains_key(&token) {
                    let replacements_for_token = replacement_map.get(&token).unwrap();
                    for replacement in replacements_for_token {
                        let mut new_molecule = candidate.clone();
                        new_molecule.splice(i..i + 1, replacement.iter().cloned());
                        if new_molecule == molecule {
                            return current_depth + 1;
                        }
                        stack
                            .entry(current_depth + 1)
                            .or_insert_with(HashSet::new)
                            .insert(new_molecule);
                    }
                }
            }
        }
        current_depth += 1;
    }
}

fn main() {
    let file = std::fs::File::open("../day_19_1/input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let lines: Vec<_> = reader.lines().map(Result::unwrap).collect();

    let (replacements, molecule) = lines.split_at(lines.len() - 2);
    let replacements: Vec<_> = replacements.iter().map(|s| parse_replacement(s)).collect();
    let molecule: Vec<String> = tokenize_molecule(&molecule[1]);
    let minimum_steps = bfs(&molecule, &replacements);
    println!("Minimum steps: {minimum_steps}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
        let replacements = vec![
            parse_replacement("e => H"),
            parse_replacement("e => O"),
            parse_replacement("H => HO"),
            parse_replacement("H => OH"),
            parse_replacement("O => HH"),
        ];

        let molecule = vec!["H".to_string(), "O".to_string(), "H".to_string()];
        assert_eq!(bfs(&molecule, &replacements), 3);

        let molecule = vec![
            "H".to_string(),
            "O".to_string(),
            "H".to_string(),
            "O".to_string(),
            "H".to_string(),
            "O".to_string(),
        ];
        assert_eq!(bfs(&molecule, &replacements), 6);
    }
}
