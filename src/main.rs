use std::process::ExitCode;

use plush::{
    BOLT_WIDTH_CM, cm_to_inch, cutting_radius, fabric_metres, parse_height_cm, pillow_fabric,
    positive, stuffing_grams,
};

const USAGE: &str = "usage: plush HEIGHT_CM [HEIGHT_CM ...] [--width CM | --round] [--seam CM] (a number above 0; add `in` or `mm`, like 12in or 300mm)";
const EXAMPLE: &str = "example: plush 30";
const DEFAULT_SEAM_CM: f64 = 1.0;

struct Options {
    heights_cm: Vec<f64>,
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
    for (i, &height) in o.heights_cm.iter().enumerate() {
        if i > 0 {
            println!();
        }
        print_block(&o, height);
    }
    ExitCode::SUCCESS
}

/// Prints the pattern for one height.
fn print_block(o: &Options, height_cm: f64) {
    let pillow_width = o.width_cm.unwrap_or(height_cm);
    let (width, length) = pillow_fabric(height_cm, pillow_width, o.seam_cm);
    println!("{} cm = {:.2} in", cm(height_cm), cm_to_inch(height_cm));
    println!("Pillow fabric: {} x {} cm", cm(width), cm(length));
    match fabric_metres(width, length) {
        Some(m) => println!("Buy: {m:.2} m of {BOLT_WIDTH_CM} cm wide fabric"),
        None => println!(
            "Buy: too wide for {BOLT_WIDTH_CM} cm fabric (needs {} cm)",
            cm(width)
        ),
    }
    if !o.round && o.width_cm.is_none() {
        println!("Stuffing: about {} g", stuffing_grams(height_cm));
    }
    if o.round {
        let radius = cutting_radius(height_cm, o.seam_cm);
        println!("Cutting radius: {} cm", cm(radius));
    }
}

/// `None` on any bad, missing or extra argument.
fn parse_args(mut args: impl Iterator<Item = String>) -> Option<Options> {
    let mut heights = Vec::new();
    let mut width = None;
    let mut round = false;
    let mut seam = DEFAULT_SEAM_CM;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--seam" => seam = positive(&args.next()?)?,
            "--width" => width = Some(parse_height_cm(&args.next()?)?),
            "--round" => round = true,
            _ => heights.push(parse_height_cm(&arg)?),
        }
    }
    if heights.is_empty() || (round && width.is_some()) {
        return None;
    }
    Some(Options {
        heights_cm: heights,
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
