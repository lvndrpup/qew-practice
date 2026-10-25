const CM_PER_INCH: f64 = 2.54;

/// Converts centimetres to inches.
pub fn cm_to_inch(cm: f64) -> f64 {
    cm / CM_PER_INCH
}

/// Fabric (width_cm, length_cm) for a square pillow whose side is `height_cm`.
pub fn pillow_fabric(height_cm: f64) -> (f64, f64) {
    todo!()
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
