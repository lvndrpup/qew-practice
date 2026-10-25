use std::process::ExitCode;

use plush::{cm_to_inch, pillow_fabric};

fn main() -> ExitCode {
    let Some(height) = std::env::args()
        .nth(1)
        .and_then(|a| a.parse::<f64>().ok())
        .filter(|h| h.is_finite() && *h > 0.0)
    else {
        eprintln!("usage: plush HEIGHT_CM (a number above 0)");
        return ExitCode::from(2);
    };
    let (width, length) = pillow_fabric(height);
    println!("{height} cm = {:.2} in", cm_to_inch(height));
    println!("Pillow fabric: {width} x {length} cm");
    ExitCode::SUCCESS
}
