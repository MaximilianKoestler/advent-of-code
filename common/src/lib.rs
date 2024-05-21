/// Converts an ASCII byte to a character.
///
/// # Arguments
///
/// * `b` - The ASCII byte to convert.
///
/// # Returns
///
/// Returns a `Result` containing the converted character if the byte is a valid ASCII character.
///
/// # Errors
///
/// Returns an `Err` containing a `std::io::Error` with `InvalidData` kind if the byte is not a valid ASCII character.
///
/// # Examples
///
/// ```
/// use common::ascii_byte_to_char;
///
/// assert_eq!(ascii_byte_to_char(b'A').unwrap(), 'A');
///
/// assert_eq!(
///     ascii_byte_to_char(128).unwrap_err().kind(),
///     std::io::ErrorKind::InvalidData
/// );
/// assert_eq!(
///     ascii_byte_to_char("👋🏽".bytes().next().unwrap())
///         .unwrap_err()
///         .kind(),
///     std::io::ErrorKind::InvalidData
/// );
/// ```
pub fn ascii_byte_to_char(b: u8) -> Result<char, std::io::Error> {
    if b.is_ascii() {
        Ok(b as char)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid ASCII",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_byte_to_char() {
        assert_eq!(ascii_byte_to_char(b'A').unwrap(), 'A');

        assert_eq!(
            ascii_byte_to_char(128).unwrap_err().kind(),
            std::io::ErrorKind::InvalidData
        );
        assert_eq!(
            ascii_byte_to_char("👋🏽".bytes().next().unwrap())
                .unwrap_err()
                .kind(),
            std::io::ErrorKind::InvalidData
        );
    }
}
