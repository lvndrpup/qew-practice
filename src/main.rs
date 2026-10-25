use std::process::ExitCode;

use plush::{cm_to_inch, pillow_fabric};

const USAGE: &str = "usage: plush HEIGHT_CM (a number above 0)";
const EXAMPLE: &str = "example: plush 30";

fn main() -> ExitCode {
    let arg = std::env::args().nth(1);
    if matches!(arg.as_deref(), Some("--help" | "-h")) {
        println!("{USAGE}\n{EXAMPLE}");
        return ExitCode::SUCCESS;
    }
    let Some(height) = arg
        .and_then(|a| a.parse::<f64>().ok())
        .filter(|h| h.is_finite() && *h > 0.0)
    else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };
    let (width, length) = pillow_fabric(height);
    println!("{height} cm = {:.2} in", cm_to_inch(height));
    println!("Pillow fabric: {width} x {length} cm");
    ExitCode::SUCCESS
}
