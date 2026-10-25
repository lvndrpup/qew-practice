const CM_PER_INCH: f64 = 2.54;

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
    positive(number).map(to_cm)
}

/// Reads a finite number above 0; `None` for anything else.
pub fn positive(text: &str) -> Option<f64> {
    text.parse::<f64>()
        .ok()
        .filter(|n| n.is_finite() && *n > 0.0)
}

/// Fabric (width_cm, length_cm) for a pillow `height_cm` by `width_cm`, with `seam_cm` on every edge.
pub fn pillow_fabric(height_cm: f64, width_cm: f64, seam_cm: f64) -> (f64, f64) {
    // Two pieces cut one above the other.
    (width_cm + 2.0 * seam_cm, 2.0 * (height_cm + 2.0 * seam_cm))
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
        assert_eq!(pillow_fabric(30.0, 30.0, 1.0), (32.0, 64.0));
    }
}
