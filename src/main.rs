use std::process::ExitCode;

use plush::{cm_to_inch, parse_height_cm, pillow_fabric, positive};

const USAGE: &str =
    "usage: plush HEIGHT_CM [--seam CM] (a number above 0; add `in` or `mm`, like 12in or 300mm)";
const EXAMPLE: &str = "example: plush 30";
const DEFAULT_SEAM_CM: f64 = 1.0;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1).peekable();
    if matches!(args.peek().map(String::as_str), Some("--help" | "-h")) {
        println!("{USAGE}\n{EXAMPLE}");
        return ExitCode::SUCCESS;
    }
    let Some((height, seam)) = parse_args(args) else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };
    let (width, length) = pillow_fabric(height, seam);
    println!("{} cm = {:.2} in", cm(height), cm_to_inch(height));
    println!("Pillow fabric: {} x {} cm", cm(width), cm(length));
    ExitCode::SUCCESS
}

/// Reads `(height_cm, seam_cm)`; `None` on any bad, missing or extra argument.
fn parse_args(mut args: impl Iterator<Item = String>) -> Option<(f64, f64)> {
    let mut height = None;
    let mut seam = DEFAULT_SEAM_CM;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--seam" => seam = positive(&args.next()?)?,
            _ if height.is_none() => height = Some(parse_height_cm(&arg)?),
            _ => return None,
        }
    }
    Some((height?, seam))
}

/// Formats a size to at most 2 decimals with trailing zeros dropped, hiding float noise.
fn cm(value: f64) -> String {
    let text = format!("{value:.2}");
    text.trim_end_matches('0').trim_end_matches('.').to_string()
}
