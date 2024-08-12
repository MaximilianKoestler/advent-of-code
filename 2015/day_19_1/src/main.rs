//! Advent of code 2015 day 19 part 1

use std::collections::{HashMap, HashSet};
use std::io::BufRead;

use day_19_1::{parse_replacement, Replacement};

fn tokenize_molecule(molecule: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut token = String::new();
    for c in molecule.chars() {
        if c.is_uppercase() {
            if !token.is_empty() {
                tokens.push(token.clone());
            }
            token.clear();
        }
        token.push(c);
    }
    if !token.is_empty() {
        tokens.push(token.clone());
    }
    tokens
}

fn create_unique_modifications(
    molecule: &[String],
    replacements: &[Replacement],
) -> HashSet<String> {
    let mut replacement_map = HashMap::new();
    for replacement in replacements {
        replacement_map
            .entry(&replacement.original)
            .or_insert(Vec::new())
            .push(&replacement.replacement);
    }

    let mut unique_modifications = HashSet::new();
    for (i, token) in molecule.iter().enumerate() {
        let replacements_for_token = replacement_map.entry(token).or_default();
        for selected_replacement in replacements_for_token {
            let molecule_string: String = molecule
                .iter()
                .enumerate()
                .map(|(j, t)| {
                    if i == j {
                        selected_replacement.clone()
                    } else {
                        t.clone()
                    }
                })
                .collect();
            unique_modifications.insert(molecule_string);
        }
    }
    unique_modifications
}

fn main() {
    let file = std::fs::File::open("input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let lines: Vec<_> = reader.lines().map(Result::unwrap).collect();

    let (replacements, molecule) = lines.split_at(lines.len() - 2);
    let replacements: Vec<_> = replacements.iter().map(|s| parse_replacement(s)).collect();
    let molecule: Vec<String> = tokenize_molecule(&molecule[1]);
    let unique_modifications = create_unique_modifications(&molecule, &replacements);
    println!("Unique modifications: {}", unique_modifications.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_molecule() {
        assert_eq!(tokenize_molecule("HOH"), ["H", "O", "H"]);
        assert_eq!(tokenize_molecule("AbCdEf"), ["Ab", "Cd", "Ef"]);
    }

    #[test]
    fn test_create_unique_modifications() {
        let molecule = vec!["H".to_string(), "O".to_string(), "H".to_string()];
        let replacements = vec![
            parse_replacement("H => HO"),
            parse_replacement("H => OH"),
            parse_replacement("O => HH"),
        ];
        let expected = [
            "HOOH".to_string(),
            "HOHO".to_string(),
            "OHOH".to_string(),
            "HHHH".to_string(),
        ]
        .into_iter()
        .collect();

        let unique_modifications = create_unique_modifications(&molecule, &replacements);
        assert_eq!(unique_modifications, expected);
        assert_eq!(unique_modifications.len(), 4);
    }
}
