use crate::wire::{Gate, Signal, Wire, WireName, WireSource};
use std::collections::HashMap;

pub struct SignalMap {
    wires: HashMap<WireName, u16>,
}

impl SignalMap {
    pub fn get_signal(&self, name: &str) -> u16 {
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

pub fn evaluate_network(
    instructions: impl Iterator<Item = Wire>,
) -> Result<SignalMap, &'static str> {
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

#[cfg(test)]
mod tests {
    use super::*;

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
