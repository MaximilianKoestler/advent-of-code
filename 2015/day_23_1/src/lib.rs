use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Register {
    A,
    B,
}

pub type RegisterValue = u32;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i32),
    Jie(Register, i32),
    Jio(Register, i32),
}

mod parsers {
    use super::{Instruction, Register};

    use nom::{
        branch::alt,
        bytes::complete::tag,
        combinator::map,
        sequence::{preceded, separated_pair},
        IResult,
    };

    fn register(input: &str) -> IResult<&str, Register> {
        alt((
            map(tag("a"), |_| Register::A),
            map(tag("b"), |_| Register::B),
        ))(input)
    }

    fn offset(input: &str) -> IResult<&str, i32> {
        nom::character::complete::i32(input)
    }

    fn instruction_hlf(input: &str) -> IResult<&str, Instruction> {
        map(preceded(tag("hlf "), register), |register| {
            Instruction::Hlf(register)
        })(input)
    }

    fn instruction_tpl(input: &str) -> IResult<&str, Instruction> {
        map(preceded(tag("tpl "), register), |register| {
            Instruction::Tpl(register)
        })(input)
    }

    fn instruction_inc(input: &str) -> IResult<&str, Instruction> {
        map(preceded(tag("inc "), register), |register| {
            Instruction::Inc(register)
        })(input)
    }

    fn instruction_jmp(input: &str) -> IResult<&str, Instruction> {
        map(preceded(tag("jmp "), offset), |offset| {
            Instruction::Jmp(offset)
        })(input)
    }

    fn instruction_jie(input: &str) -> IResult<&str, Instruction> {
        map(
            preceded(tag("jie "), separated_pair(register, tag(", "), offset)),
            |(register, offset)| Instruction::Jie(register, offset),
        )(input)
    }

    fn instruction_jio(input: &str) -> IResult<&str, Instruction> {
        map(
            preceded(tag("jio "), separated_pair(register, tag(", "), offset)),
            |(register, offset)| Instruction::Jio(register, offset),
        )(input)
    }

    pub fn instruction(input: &str) -> IResult<&str, Instruction> {
        alt((
            instruction_hlf,
            instruction_tpl,
            instruction_inc,
            instruction_jmp,
            instruction_jie,
            instruction_jio,
        ))(input)
    }
}

impl<'a> TryFrom<&'a str> for Instruction {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        parsers::instruction(input).map(|(_, c)| c)
    }
}

pub struct Processor<'a> {
    instructions: &'a [Instruction],
    program_counter: i32,
    registers: HashMap<Register, RegisterValue>,
}

impl<'a> Processor<'a> {
    #[must_use]
    pub fn new(instructions: &'a [Instruction]) -> Self {
        Processor {
            instructions,
            program_counter: 0,
            registers: HashMap::new(),
        }
    }

    fn step(&mut self) -> bool {
        if let Ok(pc) = usize::try_from(self.program_counter) {
            if let Some(instruction) = self.instructions.get(pc) {
                match instruction {
                    Instruction::Hlf(r) => *self.registers.entry(*r).or_insert(0) /= 2,
                    Instruction::Tpl(r) => *self.registers.entry(*r).or_insert(0) *= 3,
                    Instruction::Inc(r) => *self.registers.entry(*r).or_insert(0) += 1,
                    Instruction::Jmp(o) => self.program_counter += o - 1,
                    Instruction::Jie(r, o) => {
                        if self.registers.get(r).unwrap_or(&0) % 2 == 0 {
                            self.program_counter += o - 1;
                        }
                    }
                    Instruction::Jio(r, o) => {
                        if self.registers.get(r) == Some(&1) {
                            self.program_counter += o - 1;
                        }
                    }
                }
                self.program_counter += 1;
                false
            } else {
                true
            }
        } else {
            true
        }
    }

    pub fn run(&mut self) {
        while !self.step() {}
    }

    #[must_use]
    pub fn get_register(&self, register: Register) -> RegisterValue {
        self.registers.get(&register).map_or(0, |v| *v)
    }

    pub fn set_register(&mut self, register: Register, value: RegisterValue) {
        self.registers.insert(register, value);
    }

    #[allow(dead_code)]
    fn program_counter(&self) -> Option<u32> {
        usize::try_from(self.program_counter).ok().and_then(|pc| {
            if pc < self.instructions.len() {
                u32::try_from(pc).ok()
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            Instruction::try_from("hlf a").unwrap(),
            Instruction::Hlf(Register::A)
        );
        assert_eq!(
            Instruction::try_from("tpl a").unwrap(),
            Instruction::Tpl(Register::A)
        );
        assert_eq!(
            Instruction::try_from("inc a").unwrap(),
            Instruction::Inc(Register::A)
        );

        assert_eq!(Instruction::try_from("jmp 1").unwrap(), Instruction::Jmp(1));
        assert_eq!(
            Instruction::try_from("jmp +1").unwrap(),
            Instruction::Jmp(1)
        );
        assert_eq!(
            Instruction::try_from("jmp -1").unwrap(),
            Instruction::Jmp(-1)
        );

        assert_eq!(
            Instruction::try_from("jie a, 1").unwrap(),
            Instruction::Jie(Register::A, 1)
        );

        assert_eq!(
            Instruction::try_from("jio b, -1").unwrap(),
            Instruction::Jio(Register::B, -1)
        );
    }

    #[test]
    fn test_step_example() {
        let program = "inc a
                             jio a, +2
                             tpl a
                             inc a";
        let instructions: Vec<_> = program
            .lines()
            .map(|l| Instruction::try_from(l.trim()).unwrap())
            .collect();

        let mut processor = Processor::new(&instructions);
        assert_eq!(processor.get_register(Register::A), 0);
        assert_eq!(processor.get_register(Register::B), 0);

        assert!(!processor.step());
        assert_eq!(processor.get_register(Register::A), 1);

        assert!(!processor.step());
        assert_eq!(processor.get_register(Register::A), 1);

        assert!(!processor.step());
        assert_eq!(processor.get_register(Register::A), 2);

        assert!(processor.step());
    }

    #[test]
    fn test_run_example() {
        let program = "inc a
                             jio a, +2
                             tpl a
                             inc a";
        let instructions: Vec<_> = program
            .lines()
            .map(|l| Instruction::try_from(l.trim()).unwrap())
            .collect();

        let mut processor = Processor::new(&instructions);
        processor.run();
        assert_eq!(processor.get_register(Register::A), 2);
    }

    #[test]
    fn test_jump_noop() {
        let program = "jmp +1
                             inc a";
        let instructions: Vec<_> = program
            .lines()
            .map(|l| Instruction::try_from(l.trim()).unwrap())
            .collect();

        let mut processor = Processor::new(&instructions);

        assert!(!processor.step());
        assert_eq!(processor.program_counter(), Some(1));

        assert!(!processor.step());
        assert_eq!(processor.get_register(Register::A), 1);
        assert_eq!(processor.program_counter(), None);
    }

    #[test]
    fn test_jump_forward() {
        let program = "jmp +2
                             inc a
                             inc a";
        let instructions: Vec<_> = program
            .lines()
            .map(|l| Instruction::try_from(l.trim()).unwrap())
            .collect();

        let mut processor = Processor::new(&instructions);

        assert!(!processor.step());
        assert_eq!(processor.program_counter(), Some(2));

        assert!(!processor.step());
        assert_eq!(processor.get_register(Register::A), 1);
        assert_eq!(processor.program_counter(), None);
    }

    #[test]
    fn test_jump_endless_loop() {
        let program = "jmp +0";
        let instructions: Vec<_> = program
            .lines()
            .map(|l| Instruction::try_from(l.trim()).unwrap())
            .collect();

        let mut processor = Processor::new(&instructions);

        assert!(!processor.step());
        assert_eq!(processor.program_counter(), Some(0));

        assert!(!processor.step());
        assert_eq!(processor.program_counter(), Some(0));

        assert!(!processor.step());
        assert_eq!(processor.program_counter(), Some(0));
    }

    #[test]
    fn test_jump_backwards() {
        let program = "inc a
                             jmp -1";
        let instructions: Vec<_> = program
            .lines()
            .map(|l| Instruction::try_from(l.trim()).unwrap())
            .collect();

        let mut processor = Processor::new(&instructions);

        assert!(!processor.step());
        assert_eq!(processor.get_register(Register::A), 1);

        assert!(!processor.step());
        assert_eq!(processor.program_counter(), Some(0));

        assert!(!processor.step());
        assert_eq!(processor.get_register(Register::A), 2);

        assert!(!processor.step());
        assert_eq!(processor.program_counter(), Some(0));
    }

    #[test]
    fn test_conditional_jumps() {
        let program = "inc a
                             jio a, +2
                             inc a,
                             inc a,
                             inc a,
                             jie a, +100
                             jio a, +100
                             inc a
                             jie a, +2                             
                             inc b
                             inc b
                             inc b";
        let instructions: Vec<_> = program
            .lines()
            .map(|l| Instruction::try_from(l.trim()).unwrap())
            .collect();

        let mut processor = Processor::new(&instructions);
        processor.run();
        assert_eq!(processor.get_register(Register::A), 4);
        assert_eq!(processor.get_register(Register::B), 2);
    }
}
