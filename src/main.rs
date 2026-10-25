use std::process::ExitCode;

use plush::{cm_to_inch, cutting_radius, parse_height_cm, pillow_fabric, positive};

const USAGE: &str = "usage: plush HEIGHT_CM [--width CM | --round] [--seam CM] (a number above 0; add `in` or `mm`, like 12in or 300mm)";
const EXAMPLE: &str = "example: plush 30";
const DEFAULT_SEAM_CM: f64 = 1.0;

struct Options {
    height_cm: f64,
    /// `None` makes a square pillow.
    width_cm: Option<f64>,
    round: bool,
    seam_cm: f64,
}

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1).peekable();
    if matches!(args.peek().map(String::as_str), Some("--help" | "-h")) {
        println!("{USAGE}\n{EXAMPLE}");
        return ExitCode::SUCCESS;
    }
    let Some(o) = parse_args(args) else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };
    let pillow_width = o.width_cm.unwrap_or(o.height_cm);
    let (width, length) = pillow_fabric(o.height_cm, pillow_width, o.seam_cm);
    println!("{} cm = {:.2} in", cm(o.height_cm), cm_to_inch(o.height_cm));
    println!("Pillow fabric: {} x {} cm", cm(width), cm(length));
    if o.round {
        let radius = cutting_radius(o.height_cm, o.seam_cm);
        println!("Cutting radius: {} cm", cm(radius));
    }
    ExitCode::SUCCESS
}

/// `None` on any bad, missing or extra argument.
fn parse_args(mut args: impl Iterator<Item = String>) -> Option<Options> {
    let mut height = None;
    let mut width = None;
    let mut round = false;
    let mut seam = DEFAULT_SEAM_CM;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--seam" => seam = positive(&args.next()?)?,
            "--width" => width = Some(parse_height_cm(&args.next()?)?),
            "--round" => round = true,
            _ if height.is_none() => height = Some(parse_height_cm(&arg)?),
            _ => return None,
        }
    }
    if round && width.is_some() {
        return None;
    }
    Some(Options {
        height_cm: height?,
        width_cm: width,
        round,
        seam_cm: seam,
    })
}

/// Formats a size to at most 2 decimals with trailing zeros dropped, hiding float noise.
fn cm(value: f64) -> String {
    let text = format!("{value:.2}");
    text.trim_end_matches('0').trim_end_matches('.').to_string()
}
