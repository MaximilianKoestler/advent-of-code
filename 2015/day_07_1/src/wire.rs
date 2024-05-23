#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WireName(pub String);

#[derive(Clone, Debug, PartialEq)]
pub enum Signal {
    Immediate(u16),
    Connection(WireName),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Gate {
    Not(Signal),
    And(Signal, Signal),
    Or(Signal, Signal),
    LShift(Signal, u16),
    RShift(Signal, u16),
}

#[derive(Clone, Debug, PartialEq)]
pub enum WireSource {
    Value(u16),
    Gate(Gate),
    Direct(WireName),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Wire {
    pub source: WireSource,
    pub name: WireName,
}

mod parsers {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::alpha1,
        combinator::{map, map_res},
        sequence::{pair, separated_pair},
        IResult,
    };

    use super::{Gate, Signal, Wire, WireName, WireSource};

    fn parse_name(input: &str) -> IResult<&str, WireName> {
        map(map_res(alpha1, str::parse), WireName)(input)
    }

    fn parse_value(input: &str) -> IResult<&str, u16> {
        nom::character::complete::u16(input)
    }

    pub fn parse_signal(input: &str) -> IResult<&str, Signal> {
        alt((
            map(parse_value, Signal::Immediate),
            map(parse_name, Signal::Connection),
        ))(input)
    }

    fn parse_gate_not(input: &str) -> IResult<&str, Gate> {
        pair(tag("NOT "), parse_signal)(input)
            .map(|(input, (_, signal))| (input, Gate::Not(signal)))
    }

    fn parse_gate_and(input: &str) -> IResult<&str, Gate> {
        separated_pair(parse_signal, tag(" AND "), parse_signal)(input)
            .map(|(input, (lhs, rhs))| (input, Gate::And(lhs, rhs)))
    }

    fn parse_gate_or(input: &str) -> IResult<&str, Gate> {
        separated_pair(parse_signal, tag(" OR "), parse_signal)(input)
            .map(|(input, (lhs, rhs))| (input, Gate::Or(lhs, rhs)))
    }

    fn parse_gate_lshift(input: &str) -> IResult<&str, Gate> {
        separated_pair(parse_signal, tag(" LSHIFT "), parse_value)(input)
            .map(|(input, (lhs, rhs))| (input, Gate::LShift(lhs, rhs)))
    }

    fn parse_gate_rshift(input: &str) -> IResult<&str, Gate> {
        separated_pair(parse_signal, tag(" RSHIFT "), parse_value)(input)
            .map(|(input, (lhs, rhs))| (input, Gate::RShift(lhs, rhs)))
    }

    pub fn parse_gate(input: &str) -> IResult<&str, Gate> {
        alt((
            parse_gate_not,
            parse_gate_and,
            parse_gate_or,
            parse_gate_lshift,
            parse_gate_rshift,
        ))(input)
    }

    fn parse_direct(input: &str) -> IResult<&str, WireName> {
        parse_name(input)
    }

    pub fn parse_wire_source(input: &str) -> IResult<&str, WireSource> {
        // order matters here, we need to try parsing a gate first!
        alt((
            map(parse_gate, WireSource::Gate),
            map(parse_value, WireSource::Value),
            map(parse_direct, WireSource::Direct),
        ))(input)
    }

    pub fn parse_wire(input: &str) -> IResult<&str, Wire> {
        separated_pair(parse_wire_source, tag(" -> "), parse_name)(input)
            .map(|(input, (source, name))| (input, Wire { source, name }))
    }
}

impl<'a> TryFrom<&'a str> for Signal {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        parsers::parse_signal(input).map(|(_, c)| c)
    }
}

impl<'a> TryFrom<&'a str> for Gate {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        parsers::parse_gate(input).map(|(_, c)| c)
    }
}

impl<'a> TryFrom<&'a str> for WireSource {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        parsers::parse_wire_source(input).map(|(_, c)| c)
    }
}

impl<'a> TryFrom<&'a str> for Wire {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        parsers::parse_wire(input).map(|(_, c)| c)
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
            Ok(Signal::Connection(WireName("x".to_string())))
        );
        assert_eq!(
            Signal::try_from("xy"),
            Ok(Signal::Connection(WireName("xy".to_string())))
        );
    }

    #[test]
    fn test_gate() {
        assert_eq!(
            Gate::try_from("NOT x"),
            Ok(Gate::Not(Signal::Connection(WireName("x".to_string()))))
        );

        assert_eq!(
            Gate::try_from("NOT 123"),
            Ok(Gate::Not(Signal::Immediate(123)))
        );

        assert_eq!(
            Gate::try_from("x AND y"),
            Ok(Gate::And(
                Signal::Connection(WireName("x".to_string())),
                Signal::Connection(WireName("y".to_string()))
            ))
        );

        assert_eq!(
            Gate::try_from("xy AND 123"),
            Ok(Gate::And(
                Signal::Connection(WireName("xy".to_string())),
                Signal::Immediate(123)
            ))
        );

        assert_eq!(
            Gate::try_from("123 OR y"),
            Ok(Gate::Or(
                Signal::Immediate(123),
                Signal::Connection(WireName("y".to_string()))
            ))
        );
    }

    #[test]
    fn test_wire_source() {
        assert_eq!(
            WireSource::try_from("123 OR y"),
            Ok(WireSource::Gate(Gate::Or(
                Signal::Immediate(123),
                Signal::Connection(WireName("y".to_string()))
            )))
        );
    }

    #[test]
    fn test_value() {
        let wire = Wire::try_from("123 -> x").unwrap();
        assert_eq!(
            wire,
            Wire {
                source: WireSource::Value(123),
                name: WireName("x".to_string()),
            }
        );
    }

    #[test]
    fn test_direct() {
        let wire = Wire::try_from("y -> x").unwrap();
        assert_eq!(
            wire,
            Wire {
                source: WireSource::Direct(WireName("y".to_string())),
                name: WireName("x".to_string()),
            }
        );
    }

    #[test]
    fn test_gate_not() {
        let wire = Wire::try_from("NOT x -> z").unwrap();
        assert_eq!(
            wire,
            Wire {
                source: WireSource::Gate(Gate::Not(Signal::Connection(WireName("x".to_string())))),
                name: WireName("z".to_string()),
            }
        );

        let wire = Wire::try_from("NOT 123 -> z").unwrap();
        assert_eq!(
            wire,
            Wire {
                source: WireSource::Gate(Gate::Not(Signal::Immediate(123))),
                name: WireName("z".to_string()),
            }
        );
    }

    #[test]
    fn test_gate_and() {
        let wire = Wire::try_from("x AND y -> z").unwrap();
        assert_eq!(
            wire,
            Wire {
                source: WireSource::Gate(Gate::And(
                    Signal::Connection(WireName("x".to_string())),
                    Signal::Connection(WireName("y".to_string()))
                )),
                name: WireName("z".to_string()),
            }
        );

        let wire = Wire::try_from("xy AND 123 -> z").unwrap();
        assert_eq!(
            wire,
            Wire {
                source: WireSource::Gate(Gate::And(
                    Signal::Connection(WireName("xy".to_string())),
                    Signal::Immediate(123)
                )),
                name: WireName("z".to_string()),
            }
        );
    }
    #[test]
    fn test_gate_or() {
        let wire = Wire::try_from("x OR y -> z").unwrap();
        assert_eq!(
            wire,
            Wire {
                source: WireSource::Gate(Gate::Or(
                    Signal::Connection(WireName("x".to_string())),
                    Signal::Connection(WireName("y".to_string()))
                )),
                name: WireName("z".to_string()),
            }
        );

        let wire = Wire::try_from("123 OR x -> z").unwrap();
        assert_eq!(
            wire,
            Wire {
                source: WireSource::Gate(Gate::Or(
                    Signal::Immediate(123),
                    Signal::Connection(WireName("x".to_string())),
                )),
                name: WireName("z".to_string()),
            }
        );
    }

    #[test]
    fn test_gate_lshift() {
        let wire = Wire::try_from("x LSHIFT 2 -> z").unwrap();
        assert_eq!(
            wire,
            Wire {
                source: WireSource::Gate(Gate::LShift(
                    Signal::Connection(WireName("x".to_string())),
                    2
                )),
                name: WireName("z".to_string()),
            }
        );

        let wire = Wire::try_from("123 LSHIFT 2 -> z").unwrap();
        assert_eq!(
            wire,
            Wire {
                source: WireSource::Gate(Gate::LShift(Signal::Immediate(123), 2)),
                name: WireName("z".to_string()),
            }
        );
    }

    #[test]
    fn test_gate_rshift() {
        let wire = Wire::try_from("x RSHIFT 2 -> z").unwrap();
        assert_eq!(
            wire,
            Wire {
                source: WireSource::Gate(Gate::RShift(
                    Signal::Connection(WireName("x".to_string())),
                    2
                )),
                name: WireName("z".to_string()),
            }
        );

        let wire = Wire::try_from("123 RSHIFT 2 -> z").unwrap();
        assert_eq!(
            wire,
            Wire {
                source: WireSource::Gate(Gate::RShift(Signal::Immediate(123), 2)),
                name: WireName("z".to_string()),
            }
        );
    }
}
