use crate::interpreter::PietProgram;
use core::panic;
use std::borrow::Cow;

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
                let err = "Attempted to add with empty stack. Ignoring.";
                let a = match context.stack.pop() {
                    Some(a) => a,
                    None => {
                        debug!("{}", err);
                        return;
                    }
                };
                let b = match context.stack.pop() {
                    Some(b) => b,
                    None => {
                        debug!("{}", err);
                        return;
                    }
                };
                context.stack.push(a + b);
                trace!("Added values: {} + {} = {}", a, b, a + b);
            }
            Self::Subtract => {
                let err = "Attempted to subtract with empty stack. Ignoring.";
                let a = match context.stack.pop() {
                    Some(a) => a,
                    None => {
                        debug!("{}", err);
                        return;
                    }
                };
                let b = match context.stack.pop() {
                    Some(b) => b,
                    None => {
                        context.stack.push(a);
                        debug!("{}", err);
                        return;
                    }
                };
                context.stack.push(b - a);
                trace!("Subtracted values: {} - {} = {}", b, a, b - a);
            }
            Self::Multiply => {
                let err = "Attempted to multiply with empty stack. Ignoring.";
                let a = match context.stack.pop() {
                    Some(a) => a,
                    None => {
                        debug!("{}", err);
                        return;
                    }
                };
                let b = match context.stack.pop() {
                    Some(b) => b,
                    None => {
                        context.stack.push(a);
                        debug!("{}", err);
                        return;
                    }
                };
                context.stack.push(a * b);
                trace!("Multiplied values: {} * {} = {}", a, b, a * b);
            }
            Self::Divide => {
                let err = "Attempted to divide with empty stack. Ignoring.";
                let a = match context.stack.pop() {
                    Some(a) => a,
                    None => {
                        debug!("{}", err);
                        return;
                    }
                };
                let b = match context.stack.pop() {
                    Some(b) => b,
                    None => {
                        context.stack.push(a);
                        debug!("{}", err);
                        return;
                    }
                };
                if b != 0 {
                    context.stack.push(b / a);
                    trace!("Divided values: {} / {} = {}", b, a, b / a);
                } else {
                    trace!("Attempted to divide by zero. Ignoring.");
                }
            }
            Self::Mod => {
                let err = "Attempted to modulo with empty stack. Ignoring.";
                let a = match context.stack.pop() {
                    Some(a) => a,
                    None => {
                        debug!("{}", err);
                        return;
                    }
                };
                let b = match context.stack.pop() {
                    Some(b) => b,
                    None => {
                        context.stack.push(a);
                        debug!("{}", err);
                        return;
                    }
                };
                if b != 0 {
                    context.stack.push(b % a);
                    trace!("Modulo values: {} % {} = {}", b, a, b % a);
                } else {
                    trace!("Attempted to modulo by zero. Ignoring.");
                }
            }
            Self::Not => {
                let a = match context.stack.pop() {
                    Some(a) => a,
                    None => {
                        debug!("Attempted to negate with empty stack. Ignoring.");
                        return;
                    }
                };
                context.stack.push(if a == 0 { 1 } else { 0 });
                trace!("Negated value: !{}", a);
            }
            Self::Greater => {
                let err = "Attempted to compare with empty stack. Ignoring.";
                let a = match context.stack.pop() {
                    Some(a) => a,
                    None => {
                        debug!("{}", err);
                        return;
                    }
                };
                let b = match context.stack.pop() {
                    Some(b) => b,
                    None => {
                        context.stack.push(a);
                        debug!("{}", err);
                        return;
                    }
                };
                context.stack.push(if b > a { 1 } else { 0 });
                trace!(
                    "Compared values: {} > {} = {}",
                    b,
                    a,
                    if b > a { 1 } else { 0 }
                );
            }
            Self::Pointer => {
                let err = "Attempted to move pointer with empty stack. Ignoring.";
                let mut a = match context.stack.pop() {
                    Some(a) => a,
                    None => {
                        debug!("{}", err);
                        return;
                    }
                };
                let msg_a = a;
                while a != 0 {
                    if a > 0 {
                        context.move_pointer_clockwise();
                    } else {
                        context.move_pointer_anticlockwise();
                    }
                    a -= 1;
                }
                trace!("Moved pointer {} steps", msg_a);
            }
            Self::Switch => {
                let mut a = match context.stack.pop() {
                    Some(a) => a,
                    None => {
                        debug!("Attempted to toggle codel chooser with empty stack. Ignoring.");
                        return;
                    }
                };
                while a != 0 {
                    context.toggle_codel_chooser();
                    a -= 1;
                }
                trace!("Toggled codel chooser {} times", a);
            }
            Self::Duplicate => {
                let a = match context.stack.pop() {
                    Some(a) => a,
                    None => {
                        debug!("Attempted to duplicate with empty stack. Ignoring.");
                        return;
                    }
                };
                context.stack.push(a);
                context.stack.push(a);
                trace!("Duplicated value: {}", a);
            }
            // Pops the top two values off the stack and "rolls" the remaining stack entries to a depth equal to the second value popped,
            // by a number of rolls equal to the first value popped. A single roll to depth n is defined as burying the top value on the stack n deep and bringing all values
            // above it up by 1 place. A negative number of rolls rolls in the opposite direction. A negative depth is an error and the command is ignored.
            //If a roll is greater than an implementation-dependent maximum stack depth, it is handled as an implementation-dependent error, though simply ignoring the command is recommended.
            Self::Roll => {
                context.stack.roll();
            }
            Self::InNumber => {
                context.stack.in_number();
            }
            Self::InChar => {
                context.stack.in_char();
            }
            Self::OutNumber => {
                let value = match context.stack.pop() {
                    Some(value) => value,
                    None => {
                        debug!("Attempted to output with empty stack. Ignoring.");
                        return;
                    }
                };
                Self::output_number(value);
                trace!("Output number: {}", value);
            }
            Self::OutChar => {
                let value = match context.stack.pop() {
                    Some(value) => value,
                    None => {
                        debug!("Attempted to output with empty stack. Ignoring.");
                        return;
                    }
                };

                if value < 0 || value > char::MAX as i32 {
                    debug!(
                        "Attempted to output invalid character: {}. Ignoring.",
                        value
                    );
                    return;
                }
                let c = value as u8;

                Self::output_char(c);
                trace!("Output character: {}", c as char);
            }
            _ => panic!("Command not implemented: {:?}", self),
        }
    }

    pub fn to_forth(&self, context: &mut PietProgram) -> Cow<'static, str> {
        match self {
            Self::Push => {
                let value = context.get_current_value();
                value.to_string().into()
            }
            Self::Pop => "DROP".into(),
            Self::Add => "+".into(),
            Self::Subtract => "-".into(),
            Self::Multiply => "*".into(),
            Self::Divide => "/".into(),
            Self::Mod => "%".into(),
            Self::Not => "NOT".into(),
            Self::Greater => "GREATER".into(),
            Self::Pointer => "DROP".into(),
            Self::Switch => "DROP".into(),
            Self::Duplicate => "DUP".into(),
            Self::Roll => "PIET-ROLL".into(),
            Self::InNumber => "INTEGER-INPUT".into(),
            Self::InChar => "KEY".into(),
            Self::OutNumber => ".".into(),
            Self::OutChar => "EMIT".into(),
            _ => panic!("Command not implemented: {:?}", self),
        }
    }

    fn output_char(value: u8) {
        print!("{}", value as char);
    }

    fn output_number(value: i32) {
        print!("{}", value);
    }
}
