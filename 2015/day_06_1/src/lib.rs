use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, recognize},
    error::ErrorKind,
    sequence::separated_pair,
    IResult,
};

/// Represents the possible actions that can be performed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

fn parse_action(input: &str) -> IResult<&str, Action> {
    let (input, action) = alt((tag("turn on"), tag("turn off"), tag("toggle")))(input)?;
    match action {
        "turn on" => Ok((input, Action::TurnOn)),
        "turn off" => Ok((input, Action::TurnOff)),
        "toggle" => Ok((input, Action::Toggle)),
        _ => Err(nom::Err::Error(nom::error::Error::new(
            "Invalid action",
            ErrorKind::Fail,
        ))),
    }
}

impl<'a> TryFrom<&'a str> for Action {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        parse_action(input).map(|(_, c)| c)
    }
}

/// Represents a coordinate with x and y values.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(input)
}

fn parse_coordinate(input: &str) -> IResult<&str, Coordinate> {
    let (input, (x, y)) = separated_pair(parse_usize, tag(","), parse_usize)(input)?;
    Ok((input, Coordinate { x, y }))
}

impl<'a> TryFrom<&'a str> for Coordinate {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        parse_coordinate(input).map(|(_, c)| c)
    }
}

/// Represents an instruction with an action, start coordinate, and end coordinate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Instruction {
    pub action: Action,
    pub start: Coordinate,
    pub end: Coordinate,
}

impl<'a> TryFrom<&'a str> for Instruction {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let (input, action) = parse_action(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, start) = parse_coordinate(input)?;
        let (input, _) = tag(" through ")(input)?;
        let (_, end) = parse_coordinate(input)?;

        Ok(Self { action, start, end })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_action() {
        assert_eq!(Action::try_from("turn on"), Ok(Action::TurnOn));
        assert_eq!(Action::try_from("turn off"), Ok(Action::TurnOff));
        assert_eq!(Action::try_from("toggle"), Ok(Action::Toggle));
    }

    #[test]
    fn test_parse_coordinate() {
        assert_eq!(
            Coordinate::try_from("123,456"),
            Ok(Coordinate { x: 123, y: 456 })
        );
    }

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            Instruction::try_from("turn on 0,0 through 999,999"),
            Ok(Instruction {
                action: Action::TurnOn,
                start: Coordinate { x: 0, y: 0 },
                end: Coordinate { x: 999, y: 999 },
            })
        );
    }
}
