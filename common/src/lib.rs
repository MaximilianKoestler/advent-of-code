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
