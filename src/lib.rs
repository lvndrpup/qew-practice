/// Converts centimetres to inches.
pub fn cm_to_inch(cm: f64) -> f64 {
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
}
