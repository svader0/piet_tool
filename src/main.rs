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

    /// Translate the program to Forth code
    #[arg(short = 'f', long, default_value_t = false)]
    translate: bool,

    /// The location of the output file (if translating)
    #[arg(short, long, requires("translate"), default_value = "out.f")]
    output_file: String,

    /// Debug level (0 = error, 1 = info, 2 = debug, 3 = trace)
    #[arg(short, long, default_value_t = 0)]
    debug: i32,

    /// Codel Size
    #[arg(short, long, default_value_t = 1)]
    codel_size: i32,

    /// Max Execution Steps. (-1 for infinite.)
    #[arg(short, long, default_value_t = -1)]
    max_steps: i32,
}

pub mod color;
pub mod command;
pub mod interpreter;
pub mod stack;
pub mod translator;

use crate::color::PietColor;

fn main() {
    let args = Args::parse();
    let codel_size = args.codel_size;
    let translate = args.translate;
    let output_file = args.output_file;
    let max_steps = args.max_steps;
    match args.debug {
        1 => env::set_var("RUST_LOG", "info"),
        2 => env::set_var("RUST_LOG", "debug"),
        3 => env::set_var("RUST_LOG", "trace"),
        _ => env::set_var("RUST_LOG", "error"),
    }
    pretty_env_logger::init();
    let grid = load_image(&args.input_file, codel_size as u32);
    let start_time = std::time::Instant::now();

    let mut program = PietProgram::new(grid);
    if translate {
        program.execute(Some(output_file), max_steps);
    } else {
        program.execute(None, max_steps);
    }

    let elapsed = start_time.elapsed();
    debug!("Execution completed in: {:?}", elapsed);
}

pub fn load_image(path: &str, codel_size: u32) -> Vec<Vec<PietColor>> {
    let start_time = std::time::Instant::now();
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
                match PietColor::from_rgb(&rgb) {
                    Ok(color) => codel[i as usize] = color,
                    Err(_) => {
                        error!("Invalid color detected at ({:?}, {:?}): {:?}", x, y, rgb);
                        std::process::exit(1);
                    }
                }
            }
            result[y as usize / codel_size as usize][x as usize / codel_size as usize] = codel[0];
        }
    }
    let elapsed = start_time.elapsed();
    debug!(
        "Loaded image with dimensions: {}x{} in {:?}",
        width, height, elapsed
    );
    debug!(
        "Size of grid: {}x{}. Codel size: {}",
        result[0].len(),
        result.len(),
        codel_size
    );
    result
}
