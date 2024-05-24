use crate::wire::{Gate, Name, Signal, Source, Wire};
use std::collections::HashMap;

/// Represents a mapping of signal names to their corresponding values.
pub struct SignalMap {
    wires: HashMap<Name, u16>,
}

impl SignalMap {
    /// Retrieves the value of a signal by its name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the signal.
    ///
    /// # Returns
    ///
    /// The value of the signal.
    #[must_use]
    pub fn get_signal(&self, name: &str) -> Option<u16> {
        self.wires.get(&Name(name.to_string())).copied()
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
        Source::Value(value) => Some(*value),
        Source::Direct(name) => signals.wires.get(name).copied(),
        Source::Gate(gate) => try_evaluate_gate(gate, signals),
    }
}

/// Evaluates a network of wires and returns the resulting signal map.
///
/// # Arguments
///
/// * `instructions` - An iterator of `Wire` instructions representing the network.
///
/// # Returns
///
/// A `Result` containing the resulting `SignalMap` if evaluation is successful, or an error message if evaluation fails.
///
/// # Errors
///
/// Returns an error message if no progression is possible (dependency loop)
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

        assert_eq!(signals.get_signal("d"), Some(72));
        assert_eq!(signals.get_signal("e"), Some(507));
        assert_eq!(signals.get_signal("f"), Some(492));
        assert_eq!(signals.get_signal("g"), Some(114));
        assert_eq!(signals.get_signal("h"), Some(65412));
        assert_eq!(signals.get_signal("i"), Some(65079));
        assert_eq!(signals.get_signal("x"), Some(123));
        assert_eq!(signals.get_signal("y"), Some(456));
    }
}
