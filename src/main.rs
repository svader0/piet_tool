use clap::Parser;
/// Piet interpreter with Forth code generation
#[derive(Parser, Debug)]
#[command(
    version,
    author,
    long_about = "Piet interpreter with Forth code generation."
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
pub mod evaluator;

use crate::color::PietColor;

fn main() {
    let args = Args::parse();
    let _grid = load_image(&args.input_file);
    let _codel_size = args.codel_size;
    let _input_string = args.input_string;
    let _translate = args.translate;
    let _output_file = args.output_file;
    // let mut interpreter = Interpreter::new(grid, codel_size, input_string);
}

pub fn load_image(path: &str) -> Vec<Vec<PietColor>> {
    let img = image::open(path).expect("Failed to open image");
    let img = img.to_rgb8();
    let (width, height) = img.dimensions();
    let mut result = vec![vec![PietColor::White; width as usize]; height as usize];
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let rgb = [pixel[0], pixel[1], pixel[2]];
            result[y as usize][x as usize] = PietColor::from_rgb(&rgb);
        }
    }
    result
}
