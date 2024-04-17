use crate::interpreter::PietProgram;
use core::panic;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Command {
    Black,
    White,
    Nothing,
    Push,
    Pop,
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Not,
    Greater,
    Pointer,
    Switch,
    Duplicate,
    Roll,
    InNumber,
    InChar,
    OutNumber,
    OutChar,
}

impl Command {
    pub fn get_command(lightness_difference: i8, hue_difference: i8) -> Self {
        match (lightness_difference, hue_difference) {
            (0, 1) => Self::Add,
            (0, 2) => Self::Divide,
            (0, 3) => Self::Greater,
            (0, 4) => Self::Duplicate,
            (0, 5) => Self::InChar,
            (1, 0) => Self::Push,
            (1, 1) => Self::Subtract,
            (1, 2) => Self::Mod,
            (1, 3) => Self::Pointer,
            (1, 4) => Self::Roll,
            (1, 5) => Self::OutNumber,
            (2, 0) => Self::Pop,
            (2, 1) => Self::Multiply,
            (2, 2) => Self::Not,
            (2, 3) => Self::Switch,
            (2, 4) => Self::InNumber,
            (2, 5) => Self::OutChar,
            _ => panic!(
                "Invalid command for : DL{} DH{}",
                lightness_difference, hue_difference
            ),
        }
    }

    pub fn execute(&self, context: &mut PietProgram) {
        trace!("Executing command: {:?}", self);
        match self {
            Self::Push => {
                let value = context.get_current_value();
                context.stack.push(value);
                trace!("Pushed value: {}", value);
            }
            Self::Pop => {
                context.stack.pop();
                trace!("Popped value from stack");
            }
            Self::Add => {
                let a = context.stack.pop();
                let b = context.stack.pop();
                context.stack.push(a + b);
                trace!("Added values: {} + {} = {}", a, b, a + b);
            }
            Self::Subtract => {
                let a = context.stack.pop();
                let b = context.stack.pop();
                context.stack.push(b - a);
                trace!("Subtracted values: {} - {} = {}", b, a, b - a);
            }
            Self::Multiply => {
                let a = context.stack.pop();
                let b = context.stack.pop();
                context.stack.push(a * b);
                trace!("Multiplied values: {} * {} = {}", a, b, a * b);
            }
            Self::Divide => {
                let a = context.stack.pop();
                let b = context.stack.pop();
                context.stack.push(b / a);
                trace!("Divided values: {} / {} = {}", b, a, b / a);
            }
            Self::Mod => {
                let a = context.stack.pop();
                let b = context.stack.pop();
                context.stack.push(b % a);
                trace!("Calculated modulo: {} % {} = {}", b, a, b % a);
            }
            Self::Not => {
                let a = context.stack.pop();
                context.stack.push(if a == 0 { 1 } else { 0 });
                trace!("Negated value: !{}", a);
            }
            Self::Greater => {
                let a = context.stack.pop();
                let b = context.stack.pop();
                context.stack.push(if b > a { 1 } else { 0 });
                trace!(
                    "Compared values: {} > {} = {}",
                    b,
                    a,
                    if b > a { 1 } else { 0 }
                );
            }
            Self::Pointer => {
                let mut a = context.stack.pop();
                while a != 0 {
                    if a > 0 {
                        context.move_pointer_clockwise();
                    } else {
                        context.move_pointer_anticlockwise();
                    }
                    a -= 1;
                }
                trace!("Moved pointer {} steps", a);
            }
            Self::Switch => {
                let mut a = context.stack.pop();
                while a != 0 {
                    context.toggle_codel_chooser();
                    a -= 1;
                }
                trace!("Toggled codel chooser {} times", a);
            }
            Self::Duplicate => {
                let a = context.stack.pop();
                context.stack.push(a);
                context.stack.push(a);
                trace!("Duplicated value: {}", a);
            }
            // Pops the top two values off the stack and "rolls" the remaining stack entries to a depth equal to the second value popped,
            // by a number of rolls equal to the first value popped. A single roll to depth n is defined as burying the top value on the stack n deep and bringing all values
            // above it up by 1 place. A negative number of rolls rolls in the opposite direction. A negative depth is an error and the command is ignored.
            //If a roll is greater than an implementation-dependent maximum stack depth, it is handled as an implementation-dependent error, though simply ignoring the command is recommended.

            // TODO: fix roll
            Self::Roll => {
                let depth = context.stack.pop();
                let mut rolls = context.stack.pop();
                if depth < 0 {
                    return;
                }
                if rolls < 0 {
                    // Adjust rolls to be positive and rotate right
                    rolls = -rolls;
                    context.stack.roll(depth, -(rolls % depth));
                } else {
                    context.stack.roll(depth, rolls % depth);
                }
                trace!("Rolled stack: depth {} rolls {}", depth, rolls);
            }
            Self::InNumber => {
                let value = Self::get_input_number();
                context.stack.push(value);
                trace!("Input number: {}", value);
            }
            Self::InChar => {
                let value = Self::get_input_char();
                context.stack.push(value as i32);
                trace!("Input character: {}", value as char);
            }
            Self::OutNumber => {
                let value = context.stack.pop();
                Self::output_number(value);
                trace!("Output number: {}", value);
            }
            Self::OutChar => {
                let value = context.stack.pop();
                Self::output_char(value as u8);
                trace!("Output character: {}", value as u8 as char);
            }
            _ => panic!("Command not implemented: {:?}", self),
        }
    }

    fn output_char(value: u8) {
        print!("{}", value as char);
    }

    fn output_number(value: i32) {
        print!("{}", value);
    }

    fn get_input_char() -> u8 {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.chars().next().unwrap() as u8
    }

    fn get_input_number() -> i32 {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    }
}
