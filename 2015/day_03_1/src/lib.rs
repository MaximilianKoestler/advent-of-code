pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl TryFrom<char> for Direction {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '^' => Ok(Direction::North),
            '>' => Ok(Direction::East),
            'v' => Ok(Direction::South),
            '<' => Ok(Direction::West),
            _ => Err(()),
        }
    }
}
