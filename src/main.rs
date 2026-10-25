use std::process::ExitCode;

use plush::{cm_to_inch, parse_height_cm, pillow_fabric};

const USAGE: &str =
    "usage: plush HEIGHT_CM (a number above 0; add `in` or `mm`, like 12in or 300mm)";
const EXAMPLE: &str = "example: plush 30";

fn main() -> ExitCode {
    let arg = std::env::args().nth(1);
    if matches!(arg.as_deref(), Some("--help" | "-h")) {
        println!("{USAGE}\n{EXAMPLE}");
        return ExitCode::SUCCESS;
    }
    let Some(height) = arg.as_deref().and_then(parse_height_cm) else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };
    let (width, length) = pillow_fabric(height);
    println!("{} cm = {:.2} in", cm(height), cm_to_inch(height));
    println!("Pillow fabric: {} x {} cm", cm(width), cm(length));
    ExitCode::SUCCESS
}

/// Formats a size to at most 2 decimals with trailing zeros dropped, hiding float noise.
fn cm(value: f64) -> String {
    let text = format!("{value:.2}");
    text.trim_end_matches('0').trim_end_matches('.').to_string()
}
