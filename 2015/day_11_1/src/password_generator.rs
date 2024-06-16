use std::iter;

use crate::{CheckResult, PasswordChecker, Token};

/// A simple automaton for performing rules checks on passwords.
/// Can calculate the validity in a single pass.
struct SimpleAutomaton {
    current_increasing_straight: usize,
    longest_increasing_straight: usize,
    last_letter: Option<u8>,
    non_overlapping_pairs: usize,
    last_pair_end: Option<usize>,
}

impl SimpleAutomaton {
    fn new() -> Self {
        Self {
            current_increasing_straight: 1,
            longest_increasing_straight: 1,
            last_letter: None,
            non_overlapping_pairs: 0,
            last_pair_end: None,
        }
    }
}

impl PasswordChecker for SimpleAutomaton {
    /// Checks if a password is valid according to the following rules:
    /// 1. It includes one increasing straight of at least three letters
    /// 2. It does not contain the letters i, o, or l
    /// 3. It contains at least two different, non-overlapping pairs of letters
    fn consume(&mut self, position: usize, token: Token) -> CheckResult {
        match token {
            Token::Char(c) => {
                if self.last_letter == Some(c - 1u8) {
                    self.current_increasing_straight += 1;
                } else {
                    self.current_increasing_straight = 1;
                }
                self.longest_increasing_straight = self
                    .longest_increasing_straight
                    .max(self.current_increasing_straight);

                if self.last_letter == Some(c) && self.last_pair_end != Some(position - 1) {
                    self.non_overlapping_pairs += 1;
                    self.last_pair_end = Some(position);
                }

                self.last_letter = Some(c);

                if matches!(c, b'i' | b'o' | b'l') {
                    CheckResult::Invalid(1 << 2)
                } else {
                    CheckResult::Undecided
                }
            }
            Token::End => {
                if self.longest_increasing_straight >= 3 && self.non_overlapping_pairs >= 2 {
                    CheckResult::Valid
                } else {
                    let mut error = 0;
                    if self.longest_increasing_straight < 3 {
                        error |= 1 << 1;
                    }
                    if self.non_overlapping_pairs < 2 {
                        error |= 1 << 3;
                    }
                    CheckResult::Invalid(error)
                }
            }
        }
    }
}

fn validity(chars: impl Iterator<Item = u8>) -> CheckResult {
    chars
        .map(Token::Char)
        .chain(iter::once(Token::End))
        .enumerate()
        .scan(SimpleAutomaton::new(), |automaton, (i, t)| {
            Some(automaton.consume(i, t))
        })
        .find(|r| !matches!(r, CheckResult::Undecided))
        .unwrap_or(CheckResult::Undecided)
}

#[allow(dead_code)]
fn validity_str(password: &str) -> CheckResult {
    validity(password.chars().map(|c| c as u8))
}

fn next_password(password: &mut [u8]) {
    let mut i = password.len();
    while i > 0 {
        i -= 1;
        if password[i] == b'z' {
            password[i] = b'a';
        } else {
            password[i] += 1;
            break;
        }
    }
}

#[must_use]
pub fn next_password_str(current: &str) -> String {
    let mut current: Vec<_> = current.chars().map(|c| c as u8).collect();
    next_password(&mut current);
    current.iter().map(|&c| c as char).collect()
}

#[must_use]
pub fn next_valid_password_str(current: &str) -> String {
    let mut current: Vec<_> = current.chars().map(|c| c as u8).collect();
    while !matches!(validity(current.iter().copied()), CheckResult::Valid) {
        next_password(&mut current);
    }
    current.iter().map(|&c| c as char).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conformity() {
        // passes first, fails second
        assert!(matches!(
            validity_str("hijklmmn"),
            CheckResult::Invalid(errors) if (errors & 1<<1) == 0 && (errors & 1<<2) != 0
        ));

        // passes third, fails first
        assert!(matches!(
            validity_str("abbceffg"),
            CheckResult::Invalid(errors) if (errors & 1<<1) != 0 && (errors & 1<<3) == 0
        ));

        // fails third
        assert!(matches!(
            validity_str("abbcegjk"),
            CheckResult::Invalid(errors) if (errors & 1<<3) != 0
        ));

        // passes all
        assert!(matches!(validity_str("abcdffaa"), CheckResult::Valid));

        // passes all
        assert!(matches!(validity_str("ghjaabcc"), CheckResult::Valid));
    }

    #[test]
    fn test_next_password() {
        assert_eq!(next_password_str("xx"), "xy");
        assert_eq!(next_password_str("xz"), "ya");
    }

    #[test]
    fn test_next_valid_password_1() {
        assert_eq!(next_valid_password_str("abcdefgh"), "abcdffaa");
    }

    #[test]
    #[ignore = "Takes too long"]
    fn test_next_valid_password_2() {
        assert_eq!(next_valid_password_str("ghijklmn"), "ghjaabcc");
    }
}
