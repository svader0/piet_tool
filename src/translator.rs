use crate::command::Command;
use crate::interpreter::PietProgram;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Translator {
    output_file: String,
    buffer: String,
}

impl Translator {
    pub fn new(output_file: String) -> Self {
        let mut t = Self {
            output_file,
            buffer: String::new(),
        };

        // add the contents of base.f to the buffer
        t.add_base();
        t
    }

    pub fn write(&mut self, line: &str) {
        self.buffer.push_str(line);
        // if line ends with a non-number or a space, add a newline
        if !line.ends_with(|c: char| c.is_numeric() || c.is_whitespace()) {
            self.buffer.push('\n');
        } else {
            self.buffer.push(' ');
        }
    }

    pub fn flush(&mut self) {
        let path = Path::new(&self.output_file);
        let display = path.display();
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        match file.write_all(self.buffer.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => debug!("successfully wrote to {}", display),
        }
        trace!("flushing buffer");
    }

    pub fn add_command(&mut self, command: &Command, context: &mut PietProgram) {
        let forth_command = command.to_forth(context);
        trace!("Adding command to str buffer: {:?}", forth_command);
        self.write(&forth_command);
    }

    fn add_base(&mut self) {
        let bytes = include_bytes!("base.f");
        let base = String::from_utf8_lossy(bytes);
        self.write(&base);
    }
}
