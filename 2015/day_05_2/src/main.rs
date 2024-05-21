//! Advent of code 2015 day 5 part 2
//! Restrictions for today:
//!   - As many iterator adaptors as possible
//!   - No manual loops
//!   - No external non-std dependencies

use std::collections::BTreeMap;
use std::io::BufRead;
use std::iter;

use day_05_1::{CheckResult, NicetyChecker, Token};

/// A simple automaton for performing nicety checks on strings.
/// Can calculate the nicety in a single pass.
struct ComplicatedAutomaton {
    last_letter: Option<char>,
    before_last_letter: Option<char>,
    pairs: BTreeMap<(char, char), usize>,
    non_overlapping_repeated_pair: bool,
    repeat_with_one_between: bool,
}

impl ComplicatedAutomaton {
    fn new() -> Self {
        Self {
            last_letter: None,
            before_last_letter: None,
            pairs: BTreeMap::new(),
            non_overlapping_repeated_pair: false,
            repeat_with_one_between: false,
        }
    }
}

impl NicetyChecker for ComplicatedAutomaton {
    /// Checks if a string is nice according to the following rules:
    /// - It contains a pair of any two letters that appears at least twice without overlapping
    /// - It contains at least one letter which repeats with exactly one letter between them
    fn consume(&mut self, position: usize, token: Token) -> CheckResult {
        match token {
            Token::Char(c) => {
                if !self.non_overlapping_repeated_pair {
                    if let Some(last) = self.last_letter {
                        self.pairs
                            .entry((last, c))
                            .and_modify(|other_position| {
                                self.non_overlapping_repeated_pair = *other_position < position - 1;
                            })
                            .or_insert(position);
                    }
                }

                self.repeat_with_one_between =
                    self.repeat_with_one_between || self.before_last_letter == Some(c);

                self.before_last_letter = self.last_letter;
                self.last_letter = Some(c);

                if self.non_overlapping_repeated_pair && self.repeat_with_one_between {
                    CheckResult::Nice
                } else {
                    CheckResult::Undecided
                }
            }
            Token::End => {
                if self.non_overlapping_repeated_pair && self.repeat_with_one_between {
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
        .enumerate()
        .scan(ComplicatedAutomaton::new(), |automaton, (i, t)| {
            Some(automaton.consume(i, t))
        })
        .find(|r| !matches!(r, CheckResult::Undecided))
        .map_or_else(|| false, |r| matches!(r, CheckResult::Nice))
}

fn main() {
    let file = std::fs::File::open("../day_05_1/input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let nice_strings = reader
        .lines()
        .map(Result::unwrap)
        .filter(|s| is_nice(s.chars()))
        .count();

    println!("Nice Strings: {nice_strings}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nice() {
        assert!(is_nice("qjhvhtzxzqqjkmpb".chars()));
        assert!(is_nice("xxyxx".chars()));
        assert!(is_nice("xxxx".chars()));
        assert!(!is_nice("uurcxstgmygtbstg".chars()));
        assert!(!is_nice("ieodomkazucvgmuy".chars()));
    }
}
