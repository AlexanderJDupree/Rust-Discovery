//! This crate defines methods and test for calculating modular exponentiation
//! without overflow.

/// Checked modular exponentiation. Computes x ** y (mod m) returning None if m == 0
pub fn checked_modexp(x: u32, y: u32, m: u32) -> Option<u32> {
    if m == 0 {
        return None;
    }
    return Some(modexp(x, y, m));
}

/// Calculates x ** y (mod m) without overflow
pub fn modexp(x: u32, y: u32, m: u32) -> u32 {
    if x == 0 {
        return 0;
    }
    if y == 0 {
        return 1;
    }

    // Use larger container for modmultiply to prevent overflow
    let m64 = m as u64;
    let mut z = modexp(x, y / 2, m) as u64;
    z = z * z % m64;

    if y & 1 == 1 {
        // y is odd
        z = z * x as u64 % m64;
    }
    return z as u32;
}

#[test]
fn test_modexp_trivial() {
    assert_eq!(modexp(2, 20, 17), 16);
}

#[test]
fn test_checked_modexp_trivial() {
    let result = checked_modexp(2, 20, 17).unwrap();
    assert_eq!(result, 16);
}

#[test]
fn test_modexp_x_is_zero() {
    assert_eq!(modexp(0, 1, 0), 0);
}

#[test]
fn test_modexp_y_is_zero() {
    assert_eq!(modexp(1, 0, 0), 1);
}

#[test]
fn test_checked_modexp_m_is_zero() {
    assert!(checked_modexp(1, 1, 0).is_none());
}

#[test]
fn test_modexp_does_not_overflow() {
    assert_eq!(modexp(12345678, 876754321, u32::MAX), 3351228723);
}
