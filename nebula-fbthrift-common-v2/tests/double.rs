use std::io;

use nebula_fbthrift_common_v2::double::Double;

#[test]
fn set_get() -> io::Result<()> {
    assert_eq!(Double(1.0_f64).0, 1.0_f64);

    Ok(())
}
