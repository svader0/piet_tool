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
    about = "Piet interpreter with Forth code translation.",
    color(ColorChoice::Always)
)]
struct Args {
    /// The location of a valid input file
    #[arg()]
    input_file: String,

    /// The input string to be processed. Each character is pushed onto the stack prior to program execution.
    #[arg(short, long, requires("input_file"), default_value = "")]
    input_string: String,

    /// Translate the program to Forth code
    #[arg(short, long, default_value_t = false)]
    translate: bool,

    /// The location of the output file (if translating)
    #[arg(short, long, requires("translate"), default_value = "out.f")]
    output_file: String,

    /// Codel Size
    #[arg(short, long, default_value_t = 1)]
    codel_size: i32,

    /// Debug output
    #[arg(short, long, default_value_t = false)]
    debug: bool,
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
*/

fn main() {
    let args = Args::parse();
    if args.debug {
        env::set_var("RUST_LOG", "trace");
    }
    pretty_env_logger::init();
    let codel_size = args.codel_size;
    let grid = load_image(&args.input_file, codel_size as u32);
    let input_string = args.input_string;
    let translate = args.translate;
    let output_file = args.output_file;

    let start_time = std::time::Instant::now();

    let mut program = PietProgram::new(grid, input_string);
    if translate {
        program.execute(Some(output_file));
    } else {
        program.execute(None);
    }

    let elapsed = start_time.elapsed();
    println!("\nExecution completed in: {:?}", elapsed);
}

pub fn load_image(path: &str, codel_size: u32) -> Vec<Vec<PietColor>> {
    let img = image::open(path).expect("Failed to open image");
    let img = img.to_rgb8();
    let (width, height) = img.dimensions();
    let mut result = vec![
        vec![PietColor::default(); width as usize / codel_size as usize];
        height as usize / codel_size as usize
    ];
    for y in (0..height).step_by(codel_size as usize) {
        for x in (0..width).step_by(codel_size as usize) {
            let mut codel = vec![PietColor::default(); codel_size as usize];
            for i in 0..codel_size {
                let pixel = img.get_pixel(x, y + i);
                let rgb = [pixel[0], pixel[1], pixel[2]];
                codel[i as usize] = PietColor::from_rgb(&rgb);
            }
            result[y as usize / codel_size as usize][x as usize / codel_size as usize] = codel[0];
        }
    }
    trace!("Loaded image with dimensions: {}x{}", width, height);
    trace!("Codel size: {}", codel_size);
    trace!("Size of grid: {}x{}", result[0].len(), result.len());
    result
}
