#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Name(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Signal {
    Immediate(u16),
    Connection(Name),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Gate {
    Not(Signal),
    And(Signal, Signal),
    Or(Signal, Signal),
    LShift(Signal, u16),
    RShift(Signal, u16),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Source {
    Value(u16),
    Gate(Gate),
    Direct(Name),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Wire {
    pub source: Source,
    pub name: Name,
}

mod parsers {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::alpha1,
        combinator::map,
        sequence::{preceded, separated_pair},
        IResult,
    };

    use super::{Gate, Name, Signal, Source, Wire};

    fn name(input: &str) -> IResult<&str, Name> {
        map(alpha1, |name: &str| Name(name.to_string()))(input)
    }

    fn value(input: &str) -> IResult<&str, u16> {
        nom::character::complete::u16(input)
    }

    pub fn signal(input: &str) -> IResult<&str, Signal> {
        alt((map(value, Signal::Immediate), map(name, Signal::Connection)))(input)
    }

    fn gate_not(input: &str) -> IResult<&str, Gate> {
        map(preceded(tag("NOT "), signal), Gate::Not)(input)
    }

    fn gate_and(input: &str) -> IResult<&str, Gate> {
        map(
            separated_pair(signal, tag(" AND "), signal),
            |(lhs, rhs)| Gate::And(lhs, rhs),
        )(input)
    }

    fn gate_or(input: &str) -> IResult<&str, Gate> {
        map(separated_pair(signal, tag(" OR "), signal), |(lhs, rhs)| {
            Gate::Or(lhs, rhs)
        })(input)
    }

    fn gate_lshift(input: &str) -> IResult<&str, Gate> {
        map(
            separated_pair(signal, tag(" LSHIFT "), value),
            |(lhs, rhs)| Gate::LShift(lhs, rhs),
        )(input)
    }

    fn gate_rshift(input: &str) -> IResult<&str, Gate> {
        map(
            separated_pair(signal, tag(" RSHIFT "), value),
            |(lhs, rhs)| Gate::RShift(lhs, rhs),
        )(input)
    }

    pub fn gate(input: &str) -> IResult<&str, Gate> {
        alt((gate_not, gate_and, gate_or, gate_lshift, gate_rshift))(input)
    }

    pub fn source(input: &str) -> IResult<&str, Source> {
        // order matters here, we need to try parsing a gate first!
        alt((
            map(gate, Source::Gate),
            map(value, Source::Value),
            map(name, Source::Direct),
        ))(input)
    }

    pub fn wire(input: &str) -> IResult<&str, Wire> {
        map(
            separated_pair(source, tag(" -> "), name),
            |(source, name)| Wire { source, name },
        )(input)
    }
}

impl<'a> TryFrom<&'a str> for Signal {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        parsers::signal(input).map(|(_, c)| c)
    }
}

impl<'a> TryFrom<&'a str> for Gate {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        parsers::gate(input).map(|(_, c)| c)
    }
}

impl<'a> TryFrom<&'a str> for Source {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        parsers::source(input).map(|(_, c)| c)
    }
}

impl<'a> TryFrom<&'a str> for Wire {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        parsers::wire(input).map(|(_, c)| c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal() {
        assert_eq!(Signal::try_from("123"), Ok(Signal::Immediate(123)));
        assert_eq!(
            Signal::try_from("x"),
            Ok(Signal::Connection(Name("x".to_string())))
        );
        assert_eq!(
            Signal::try_from("xy"),
            Ok(Signal::Connection(Name("xy".to_string())))
        );
    }

    #[test]
    fn test_gate() {
        assert_eq!(
            Gate::try_from("NOT x"),
            Ok(Gate::Not(Signal::Connection(Name("x".to_string()))))
        );

        assert_eq!(
            Gate::try_from("NOT 123"),
            Ok(Gate::Not(Signal::Immediate(123)))
        );

        assert_eq!(
            Gate::try_from("x AND y"),
            Ok(Gate::And(
                Signal::Connection(Name("x".to_string())),
                Signal::Connection(Name("y".to_string()))
            ))
        );

        assert_eq!(
            Gate::try_from("xy AND 123"),
            Ok(Gate::And(
                Signal::Connection(Name("xy".to_string())),
                Signal::Immediate(123)
            ))
        );

        assert_eq!(
            Gate::try_from("123 OR y"),
            Ok(Gate::Or(
                Signal::Immediate(123),
                Signal::Connection(Name("y".to_string()))
            ))
        );
    }

    #[test]
    fn test_wire_source() {
        assert_eq!(
            Source::try_from("123 OR y"),
            Ok(Source::Gate(Gate::Or(
                Signal::Immediate(123),
                Signal::Connection(Name("y".to_string()))
            )))
        );
    }

    #[test]
    fn test_value() {
        assert_eq!(
            Wire::try_from("123 -> x"),
            Ok(Wire {
                source: Source::Value(123),
                name: Name("x".to_string()),
            })
        );
    }

    #[test]
    fn test_direct() {
        assert_eq!(
            Wire::try_from("y -> x"),
            Ok(Wire {
                source: Source::Direct(Name("y".to_string())),
                name: Name("x".to_string()),
            })
        );
    }

    #[test]
    fn test_gate_not() {
        assert_eq!(
            Wire::try_from("NOT x -> z"),
            Ok(Wire {
                source: Source::Gate(Gate::Not(Signal::Connection(Name("x".to_string())))),
                name: Name("z".to_string()),
            })
        );

        assert_eq!(
            Wire::try_from("NOT 123 -> z"),
            Ok(Wire {
                source: Source::Gate(Gate::Not(Signal::Immediate(123))),
                name: Name("z".to_string()),
            })
        );
    }

    #[test]
    fn test_gate_and() {
        assert_eq!(
            Wire::try_from("x AND y -> z"),
            Ok(Wire {
                source: Source::Gate(Gate::And(
                    Signal::Connection(Name("x".to_string())),
                    Signal::Connection(Name("y".to_string()))
                )),
                name: Name("z".to_string()),
            })
        );

        assert_eq!(
            Wire::try_from("xy AND 123 -> z"),
            Ok(Wire {
                source: Source::Gate(Gate::And(
                    Signal::Connection(Name("xy".to_string())),
                    Signal::Immediate(123)
                )),
                name: Name("z".to_string()),
            })
        );
    }
    #[test]
    fn test_gate_or() {
        assert_eq!(
            Wire::try_from("x OR y -> z"),
            Ok(Wire {
                source: Source::Gate(Gate::Or(
                    Signal::Connection(Name("x".to_string())),
                    Signal::Connection(Name("y".to_string()))
                )),
                name: Name("z".to_string()),
            })
        );

        assert_eq!(
            Wire::try_from("123 OR x -> z"),
            Ok(Wire {
                source: Source::Gate(Gate::Or(
                    Signal::Immediate(123),
                    Signal::Connection(Name("x".to_string())),
                )),
                name: Name("z".to_string()),
            })
        );
    }

    #[test]
    fn test_gate_lshift() {
        assert_eq!(
            Wire::try_from("x LSHIFT 2 -> z"),
            Ok(Wire {
                source: Source::Gate(Gate::LShift(Signal::Connection(Name("x".to_string())), 2)),
                name: Name("z".to_string()),
            })
        );

        assert_eq!(
            Wire::try_from("123 LSHIFT 2 -> z"),
            Ok(Wire {
                source: Source::Gate(Gate::LShift(Signal::Immediate(123), 2)),
                name: Name("z".to_string()),
            })
        );
    }

    #[test]
    fn test_gate_rshift() {
        assert_eq!(
            Wire::try_from("x RSHIFT 2 -> z"),
            Ok(Wire {
                source: Source::Gate(Gate::RShift(Signal::Connection(Name("x".to_string())), 2)),
                name: Name("z".to_string()),
            })
        );

        assert_eq!(
            Wire::try_from("123 RSHIFT 2 -> z"),
            Ok(Wire {
                source: Source::Gate(Gate::RShift(Signal::Immediate(123), 2)),
                name: Name("z".to_string()),
            })
        );
    }
}
