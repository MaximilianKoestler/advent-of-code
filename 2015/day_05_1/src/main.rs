//! Advent of code 2015 day 5 part 1
//! Restrictions for today:
//!   - As many iterator adaptors as possible
//!   - No manual loops
//!   - No external non-std dependencies

use std::io::BufRead;
use std::iter;

/// Represents the result of a nicety check.
enum CheckResult {
    /// The nicety check is still undecided.
    Undecided,
    /// The string is definitely nice.
    Nice,
    /// The string is definitely naughty.
    Naughty,
}

/// Represents a token in the input string.
enum Token {
    /// A character token.
    Char(char),
    /// The end of the string token.
    End,
}

/// Checks if a string is nice according to the following rules:
/// - It contains at least three vowels (aeiou only)
/// - It contains at least one letter that appears twice in a row
/// - It does not contain the strings ab, cd, pq, or xy
trait NicetyChecker {
    /// Consumes a token and returns the result of the nicety check.
    fn consume(&mut self, t: Token) -> CheckResult;
}

/// A simple automaton for performing nicety checks on strings.
/// Can calculate the nicety in a single pass.
struct SimpleAutomaton {
    vowels: u32,
    last_letter: Option<char>,
    double_letter: bool,
}

impl SimpleAutomaton {
    fn new() -> Self {
        Self {
            vowels: 0,
            last_letter: None,
            double_letter: false,
        }
    }
}

impl NicetyChecker for SimpleAutomaton {
    fn consume(&mut self, t: Token) -> CheckResult {
        match t {
            Token::Char(c) => {
                if matches!(c, 'a' | 'e' | 'i' | 'o' | 'u') {
                    self.vowels += 1;
                }
                if self.last_letter == Some(c) {
                    self.double_letter = true;
                }

                if matches!(c, 'b' | 'd' | 'q' | 'y')
                    && self.last_letter == Some(((c as u8) - 1) as char)
                {
                    CheckResult::Naughty
                } else {
                    self.last_letter = Some(c);
                    CheckResult::Undecided
                }
            }
            Token::End => {
                if self.vowels >= 3 && self.double_letter {
                    CheckResult::Nice
                } else {
                    CheckResult::Naughty
                }
            }
        }
    }
}

fn is_nice(chars: impl Iterator<Item = char>) -> bool {
    chars
        .map(Token::Char)
        .chain(iter::once(Token::End))
        .scan(SimpleAutomaton::new(), |automaton, t| {
            Some(automaton.consume(t))
        })
        .all(|r| match r {
            CheckResult::Undecided => true,
            CheckResult::Nice => true,
            CheckResult::Naughty => false,
        })
}

fn main() {
    let file = std::fs::File::open("input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let nice_strings = reader
        .lines()
        .map(Result::unwrap)
        .filter(|s| is_nice(s.chars()))
        .count();

    println!("Nice Strings: {}", nice_strings);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nice() {
        assert!(is_nice("ugknbfddgicrmopn".chars()));
        assert!(is_nice("aaa".chars()));
        assert!(!is_nice("jchzalrnumimnmhp".chars()));
        assert!(!is_nice("haegwjzuvuyypxyu".chars()));
        assert!(!is_nice("dvszwmarrgswjxmb".chars()));
    }
}
