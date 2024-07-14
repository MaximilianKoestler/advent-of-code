use std::ops::AddAssign;

/// A direction on a 2D grid
#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

/// A position on a 2D grid
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl TryFrom<char> for Direction {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '^' => Ok(Self::North),
            '>' => Ok(Self::East),
            'v' => Ok(Self::South),
            '<' => Ok(Self::West),
            _ => Err(()),
        }
    }
}

impl AddAssign<Direction> for Position {
    fn add_assign(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.y += 1,
            Direction::East => self.x += 1,
            Direction::South => self.y -= 1,
            Direction::West => self.x -= 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_from_char() {
        assert_eq!(Direction::try_from('^'), Ok(Direction::North));
        assert_eq!(Direction::try_from('>'), Ok(Direction::East));
        assert_eq!(Direction::try_from('v'), Ok(Direction::South));
        assert_eq!(Direction::try_from('<'), Ok(Direction::West));
        assert_eq!(Direction::try_from('x'), Err(()));
    }

    #[test]
    fn test_position_add_assign() {
        let mut position = Position { x: 0, y: 0 };

        position += Direction::North;
        assert_eq!(position, Position { x: 0, y: 1 });

        position += Direction::East;
        assert_eq!(position, Position { x: 1, y: 1 });

        position += Direction::South;
        assert_eq!(position, Position { x: 1, y: 0 });

        position += Direction::West;
        assert_eq!(position, Position { x: 0, y: 0 });
    }
}
