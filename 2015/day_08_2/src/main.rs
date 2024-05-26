//! Advent of code 2015 day 8 part 2

use std::io::BufRead;

struct CharacterCounter {
    /// the number of characters in the raw input string
    code: usize,

    /// the number of characters in memory after parsing
    data: usize,
}

impl CharacterCounter {
    fn new() -> Self {
        Self { code: 2, data: 0 }
    }

    fn process(&mut self, input: char) {
        self.data += 1;
        self.code += 1;
        match input {
            '\\' | '"' => self.code += 1,
            _ => {}
        }
    }

    fn count(&self) -> Option<(usize, usize)> {
        Some((self.code, self.data))
    }
}

fn count_characters(input: &str) -> Option<(usize, usize)> {
    let mut counter = CharacterCounter::new();
    input.chars().for_each(|c| counter.process(c));
    counter.count()
}

fn main() {
    let file = std::fs::File::open("../day_08_1/input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let (code, data) = reader
        .lines()
        .map(Result::unwrap)
        .map(|s| count_characters(&s).unwrap())
        .fold((0, 0), |(code_acc, data_acc), (code, data)| {
            (code_acc + code, data_acc + data)
        });

    println!("Size difference: {}", code - data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(count_characters(r#""""#), Some((6, 2)));
    }

    #[test]
    fn test_abc_string() {
        assert_eq!(count_characters(r#""abc""#), Some((9, 5)));
    }

    #[test]
    fn test_aaaaaa_string() {
        assert_eq!(count_characters(r#""aaa\"aaa""#), Some((16, 10)));
    }

    #[test]
    fn test_hex_string() {
        assert_eq!(count_characters(r#""\x27""#), Some((11, 6)));
    }
}
