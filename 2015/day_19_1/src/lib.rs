pub struct Replacement {
    pub original: String,
    pub replacement: String,
}

pub fn parse_replacement(line: &str) -> Replacement {
    let parts: Vec<&str> = line.split(" => ").collect();
    Replacement {
        original: parts[0].to_string(),
        replacement: parts[1].to_string(),
    }
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
}
