use std::process::ExitCode;

use plush::{cm_to_inch, pillow_fabric};

fn main() -> ExitCode {
    let Some(height) = std::env::args().nth(1).and_then(|a| a.parse::<f64>().ok()) else {
        eprintln!("usage: plush HEIGHT_CM");
        return ExitCode::from(2);
    };
    let (width, length) = pillow_fabric(height);
    println!("{height} cm = {:.2} in", cm_to_inch(height));
    println!("Pillow fabric: {width} x {length} cm");
    ExitCode::SUCCESS
}
