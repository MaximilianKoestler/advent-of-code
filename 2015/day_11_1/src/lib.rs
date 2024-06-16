pub mod password_generator;

/// Represents the result of a password check.
#[derive(Debug)]
pub enum CheckResult {
    /// The password check is still undecided.
    Undecided,
    /// The string is definitely a valid password.
    Valid,
    /// The string is definitely invalid (includes the requirement numbers).
    Invalid(u32),
}

/// Represents a token in the input string.
#[derive(Debug)]
pub enum Token {
    /// A character token.
    Char(u8),
    /// The end of the string token.
    End,
}

pub trait PasswordChecker {
    /// Consumes a token and returns the result of the password check.
    fn consume(&mut self, position: usize, token: Token) -> CheckResult;
}
