const CM_PER_INCH: f64 = 2.54;
const SEAM_ALLOWANCE_CM: f64 = 1.0;

/// Converts centimetres to inches.
pub fn cm_to_inch(cm: f64) -> f64 {
    cm / CM_PER_INCH
}

/// Reads a height like `30` (cm), `12in` or `300mm`; `None` unless it is a finite number above 0.
pub fn parse_height_cm(text: &str) -> Option<f64> {
    // mm divides by 10 rather than multiplying by 0.1, which is inexact in binary.
    let (number, to_cm): (&str, fn(f64) -> f64) = if let Some(n) = text.strip_suffix("in") {
        (n, |x| x * CM_PER_INCH)
    } else if let Some(n) = text.strip_suffix("mm") {
        (n, |x| x / 10.0)
    } else {
        (text, |x| x)
    };
    number
        .parse::<f64>()
        .ok()
        .filter(|n| n.is_finite() && *n > 0.0)
        .map(to_cm)
}

/// Fabric (width_cm, length_cm) for a square pillow whose side is `height_cm`.
pub fn pillow_fabric(height_cm: f64) -> (f64, f64) {
    let cut_side = height_cm + 2.0 * SEAM_ALLOWANCE_CM;
    // Two squares cut side by side.
    (cut_side, 2.0 * cut_side)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Catches a wrong conversion factor.
    #[test]
    fn hundred_inches_is_254_cm() {
        assert_eq!(cm_to_inch(254.0), 100.0);
    }

    // Catches a missing seam allowance or a single square instead of two.
    #[test]
    fn thirty_cm_pillow_needs_32_by_64() {
        assert_eq!(pillow_fabric(30.0), (32.0, 64.0));
    }
}
