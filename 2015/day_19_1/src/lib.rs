pub struct Replacement {
    pub original: String,
    pub replacement: String,
}

#[must_use]
pub fn parse_replacement(line: &str) -> Replacement {
    let parts: Vec<&str> = line.split(" => ").collect();
    Replacement {
        original: parts[0].to_string(),
        replacement: parts[1].to_string(),
    }
}

#[must_use]
pub fn tokenize_molecule(molecule: &str) -> Vec<String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_replacement() {
        let replacement = parse_replacement("H => HO");
        assert_eq!(replacement.original, "H");
        assert_eq!(replacement.replacement, "HO");
    }

    #[test]
    fn test_tokenize_molecule() {
        assert_eq!(tokenize_molecule("HOH"), ["H", "O", "H"]);
        assert_eq!(tokenize_molecule("AbCdEf"), ["Ab", "Cd", "Ef"]);
    }
}
