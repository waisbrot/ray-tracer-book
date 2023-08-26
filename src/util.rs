
pub type Float = f64;
pub fn feq(a: &Float, b: &Float) -> bool {
    feq_precision(a, b, 5)
}

pub fn feq_precision(a: &Float, b: &Float, precision: i32) -> bool {
    (a - b).abs() < (1.0 * Float::powi(10.0, -precision))
}