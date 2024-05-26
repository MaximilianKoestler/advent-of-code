//! Advent of code 2015 day 8 part 1

use std::io::BufRead;

enum State {
    Start,
    Valid,
    EscapeSymbol,
    EscapeHexX,
    EscapeHex1,
    End,
    Error,
}

struct CharacterCounter {
    /// the number of characters in the raw input string
    code: usize,

    /// the number of characters in memory after parsing
    data: usize,

    state: State,
}

impl CharacterCounter {
    fn new() -> Self {
        Self {
            code: 0,
            data: 0,
            state: State::Start,
        }
    }

    fn process(&mut self, input: char) {
        self.code += 1;
        self.state = match (&self.state, input) {
            (State::Start, '"') => State::Valid,
            (State::Valid, '"') => State::End,
            (State::Valid, '\\') => State::EscapeSymbol,
            (State::Valid, _) | (State::EscapeSymbol, '\\' | '"') => {
                self.data += 1;
                State::Valid
            }
            (State::EscapeSymbol, 'x') => State::EscapeHexX,
            (State::EscapeHexX, c) if c.is_ascii_hexdigit() => State::EscapeHex1,
            (State::EscapeHex1, c) if c.is_ascii_hexdigit() => {
                self.data += 1;
                State::Valid
            }
            (_, _) => State::Error,
        }
    }

    fn count(&self) -> Option<(usize, usize)> {
        if !matches!(self.state, State::End) {
            return None;
        }
        Some((self.code, self.data))
    }
}

fn count_characters(input: &str) -> Option<(usize, usize)> {
    let mut counter = CharacterCounter::new();
    input.chars().for_each(|c| counter.process(c));
    counter.count()
}

fn main() {
    let file = std::fs::File::open("input/input.txt").unwrap();
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
    fn test_not_terminated_string() {
        assert_eq!(count_characters(r#""abc"#), None);
    }

    #[test]
    fn test_not_started_string() {
        assert_eq!(count_characters(r#"abc""#), None);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(count_characters(r#""""#), Some((2, 0)));
    }

    #[test]
    fn test_abc_string() {
        assert_eq!(count_characters(r#""abc""#), Some((5, 3)));
    }

    #[test]
    fn test_aaaaaa_string() {
        assert_eq!(count_characters(r#""aaa\"aaa""#), Some((10, 7)));
    }

    #[test]
    fn test_hex_string() {
        assert_eq!(count_characters(r#""\x27""#), Some((6, 1)));
    }
}
