//! Advent of code 2015 day 7 part 1

use std::collections::HashMap;
use std::io::BufRead;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map, map_res, recognize},
    sequence::{pair, separated_pair},
    IResult,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct WireName(String);

#[derive(Clone, Debug, PartialEq)]
enum Signal {
    Immediate(u16),
    Connection(WireName),
}

#[derive(Clone, Debug, PartialEq)]
enum Gate {
    Not(Signal),
    And(Signal, Signal),
    Or(Signal, Signal),
    LShift(Signal, u16),
    RShift(Signal, u16),
}

#[derive(Clone, Debug, PartialEq)]
enum WireSource {
    Value(u16),
    Gate(Gate),
    Direct(WireName),
}

#[derive(Clone, Debug, PartialEq)]
struct Wire {
    source: WireSource,
    name: WireName,
}

fn parse_name(input: &str) -> IResult<&str, WireName> {
    map_res(recognize(alpha1), str::parse)(input).map(|(input, name)| (input, WireName(name)))
}

fn parse_value(input: &str) -> IResult<&str, u16> {
    map_res(recognize(digit1), str::parse)(input)
}

fn parse_signal(input: &str) -> IResult<&str, Signal> {
    alt((
        map(parse_value, Signal::Immediate),
        map(parse_name, Signal::Connection),
    ))(input)
}

impl<'a> TryFrom<&'a str> for Signal {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        parse_signal(input).map(|(_, c)| c)
    }
}

fn parse_gate_not(input: &str) -> IResult<&str, Gate> {
    pair(tag("NOT "), parse_signal)(input).map(|(input, (_, signal))| (input, Gate::Not(signal)))
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

fn parse_gate(input: &str) -> IResult<&str, Gate> {
    alt((
        parse_gate_not,
        parse_gate_and,
        parse_gate_or,
        parse_gate_lshift,
        parse_gate_rshift,
    ))(input)
}

impl<'a> TryFrom<&'a str> for Gate {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        parse_gate(input).map(|(_, c)| c)
    }
}

fn parse_direct(input: &str) -> IResult<&str, WireName> {
    parse_name(input)
}

fn parse_wire_source(input: &str) -> IResult<&str, WireSource> {
    // order matters here, we need to try parsing a gate first!
    alt((
        map(parse_gate, WireSource::Gate),
        map(parse_value, WireSource::Value),
        map(parse_direct, WireSource::Direct),
    ))(input)
}

impl<'a> TryFrom<&'a str> for WireSource {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        parse_wire_source(input).map(|(_, c)| c)
    }
}

fn parse_wire(input: &str) -> IResult<&str, Wire> {
    separated_pair(parse_wire_source, tag(" -> "), parse_name)(input)
        .map(|(input, (source, name))| (input, Wire { source, name }))
}

impl<'a> TryFrom<&'a str> for Wire {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        parse_wire(input).map(|(_, c)| c)
    }
}

struct SignalMap {
    wires: HashMap<WireName, u16>,
}

impl SignalMap {
    fn get_signal(&self, name: &str) -> u16 {
        self.wires[&WireName(name.to_string())]
    }
}

fn try_evaluate_signal(signal: &Signal, signals: &SignalMap) -> Option<u16> {
    match signal {
        Signal::Immediate(value) => Some(*value),
        Signal::Connection(name) => signals.wires.get(name).copied(),
    }
}

fn try_evaluate_gate(gate: &Gate, signals: &SignalMap) -> Option<u16> {
    match gate {
        Gate::Not(signal) => try_evaluate_signal(signal, signals).map(|signal| !signal),
        Gate::And(lhs, rhs) => try_evaluate_signal(lhs, signals)
            .and_then(|lhs| try_evaluate_signal(rhs, signals).map(|rhs| lhs & rhs)),
        Gate::Or(lhs, rhs) => try_evaluate_signal(lhs, signals)
            .and_then(|lhs| try_evaluate_signal(rhs, signals).map(|rhs| lhs | rhs)),
        Gate::LShift(signal, shift) => {
            try_evaluate_signal(signal, signals).map(|signal| signal << shift)
        }
        Gate::RShift(signal, shift) => {
            try_evaluate_signal(signal, signals).map(|signal| signal >> shift)
        }
    }
}

fn try_evaluate_instruction(instruction: &Wire, signals: &SignalMap) -> Option<u16> {
    match &instruction.source {
        WireSource::Value(value) => Some(*value),
        WireSource::Direct(name) => signals.wires.get(name).copied(),
        WireSource::Gate(gate) => try_evaluate_gate(gate, signals),
    }
}

fn evaluate_network(instructions: impl Iterator<Item = Wire>) -> Result<SignalMap, &'static str> {
    let mut signals = SignalMap {
        wires: HashMap::new(),
    };

    let mut instructions = instructions.collect::<Vec<_>>();
    'outer: while !instructions.is_empty() {
        for i in 0..instructions.len() {
            let instruction = &instructions[i];
            if let Some(signal) = try_evaluate_instruction(instruction, &signals) {
                signals.wires.insert(instruction.name.clone(), signal);
                instructions.remove(i);
                continue 'outer;
            }
        }
        return Err("No progress possible");
    }
    Ok(signals)
}

fn main() {
    let file = std::fs::File::open("input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let signals = evaluate_network(
        reader
            .lines()
            .map(Result::unwrap)
            .map(|line| Wire::try_from(line.as_ref()).unwrap()),
    );

    println!("Signal on wire a: {}", signals.unwrap().get_signal("a"));
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

    #[test]
    fn test_evaluate() {
        let signals = evaluate_network(
            vec![
                Wire::try_from("123 -> x").unwrap(),
                Wire::try_from("456 -> y").unwrap(),
                Wire::try_from("x AND y -> d").unwrap(),
                Wire::try_from("x OR y -> e").unwrap(),
                Wire::try_from("x LSHIFT 2 -> f").unwrap(),
                Wire::try_from("y RSHIFT 2 -> g").unwrap(),
                Wire::try_from("NOT x -> h").unwrap(),
                Wire::try_from("NOT y -> i").unwrap(),
            ]
            .into_iter(),
        )
        .unwrap();

        assert_eq!(signals.get_signal("d"), 72);
        assert_eq!(signals.get_signal("e"), 507);
        assert_eq!(signals.get_signal("f"), 492);
        assert_eq!(signals.get_signal("g"), 114);
        assert_eq!(signals.get_signal("h"), 65412);
        assert_eq!(signals.get_signal("i"), 65079);
        assert_eq!(signals.get_signal("x"), 123);
        assert_eq!(signals.get_signal("y"), 456);
    }
}
