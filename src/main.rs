use std::env;

use clap::{ColorChoice, Parser};
use interpreter::PietProgram;

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

#[derive(Parser, Debug)]
#[command(
    version,
    author,
    long_about = "Piet interpreter with Forth code translation.",
    color(ColorChoice::Always)
)]
struct Args {
    /// The location of a valid input file
    #[arg()]
    input_file: String,

    /// The input string to be processed.
    #[arg(short, long, requires("input_file"), default_value = "")]
    input_string: String,

    /// Translate the program to Forth code
    #[arg(short, long, default_value_t = false)]
    translate: bool,

    /// The location of the output file (if translating)
    #[arg(short, long, requires("translate"), default_value = "out.forth")]
    output_file: String,

    /// Codel Size
    #[arg(short, long, default_value_t = 1)]
    codel_size: i32,
}

pub mod color;
pub mod command;
pub mod interpreter;
pub mod stack;
pub mod translator;

use crate::color::PietColor;

/*
    known issues:
    - roll command is not implemented correctly
    - when you start on white you start on DL 0, DH 0 which is not a valid command
*/

fn main() {
    env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    let args = Args::parse();
    let grid = load_image(&args.input_file);
    let codel_size = args.codel_size;
    let input_string = args.input_string;
    let translate = args.translate;
    let output_file = args.output_file;

    let mut program = PietProgram::new(grid, input_string);
    if translate {
        program.execute(Some(output_file));
    } else {
        program.execute(None);
    }
}

pub fn load_image(path: &str) -> Vec<Vec<PietColor>> {
    let img = image::open(path).expect("Failed to open image");
    let img = img.to_rgb8();
    let (width, height) = img.dimensions();
    let mut result = vec![vec![PietColor::default(); width as usize]; height as usize];
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let rgb = [pixel[0], pixel[1], pixel[2]];
            result[y as usize][x as usize] = PietColor::from_rgb(&rgb);
        }
    }
    result
}
