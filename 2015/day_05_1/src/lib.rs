/// Represents the result of a nicety check.
#[derive(Debug)]
pub enum CheckResult {
    /// The nicety check is still undecided.
    Undecided,
    /// The string is definitely nice.
    Nice,
    /// The string is definitely naughty.
    Naughty,
}

/// Represents a token in the input string.
#[derive(Debug)]
pub enum Token {
    /// A character token.
    Char(char),
    /// The end of the string token.
    End,
}

pub trait NicetyChecker {
    /// Consumes a token and returns the result of the nicety check.
    fn consume(&mut self, position: usize, token: Token) -> CheckResult;
}
