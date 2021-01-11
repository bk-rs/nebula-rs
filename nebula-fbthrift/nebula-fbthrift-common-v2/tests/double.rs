use float_cmp::approx_eq;

use nebula_fbthrift_common_v2::double::Double;

#[test]
fn set_get() {
    assert!(approx_eq!(f64, Double(1.0_f64).0, 1.0_f64));
}
