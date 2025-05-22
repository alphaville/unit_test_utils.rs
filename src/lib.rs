//! Collection of utilities for unit tests
//!
//! This crate offers tools for unit tests, especially for projects involving
//! numerical methods
//!
extern crate num;
use num::{Float, Zero};

fn float_max<T>(a: T, b: T) -> T
where
    T: Float,
{
    if a >= b {
        a
    } else {
        b
    }
}

fn float_min<T>(a: T, b: T) -> T
where
    T: Float,
{
    if a >= b {
        b
    } else {
        a
    }
}

/// Whether two floats are nearly equal (up to specified tolerance)
///
/// ## Arguments
/// - `a` first float
/// - `b` second float
/// - `rel_tol` relative tolerance (must be positive)
/// - `abs_tol` absolute tolerance (must be positive)
///
/// ## Results
///
/// Returns true if and only if `a` is nearly equal to `b`
///
/// In particular, this function will return true if and only if BOTH of the following
/// conditions are satisfied
/// - `a==b`, e.g., if the two floats are identical or both equal to infinity
/// - `|a-b| <= max(abs_tol, rel_tol*max(|a|, |b|))`
///
/// The function will return false if either of `a` or `b` is NaN.
///
/// It works with `f64` and `f32`
///
/// ## Panics
///
/// The function will panic if the specified relative or absolute tolerance is
/// not positive.
///
pub fn nearly_equal<T>(a: T, b: T, rel_tol: T, abs_tol: T) -> bool
where
    T: Float + Zero,
{
    assert!(rel_tol > T::zero(), "relative tolerance nonpositive");
    assert!(abs_tol > T::zero(), "absolute tolerance nonpositive");

    let abs_a = a.abs();
    let abs_b = b.abs();
    let abs_diff = (a - b).abs();

    if a.is_nan() || b.is_nan() {
        false
    } else if a == b || abs_diff <= T::min_positive_value() {
        true
    } else {
        let max_abs_a_b = float_max(abs_a, abs_b);
        abs_diff <= float_min(abs_tol, rel_tol * max_abs_a_b)
    }
}

/// Asserts that two numbers are nearly equal
///
/// ## Arguments
/// - `a` first float
/// - `b` second float
/// - `rel_tol` relative tolerance (must be positive)
/// - `abs_tol` absolute tolerance (must be positive)
/// - `msg` an error message that will be thrown if the two numbers are not nearly equal
///
/// ## Panics
///
/// The function panics if the two floating-point numbers are not almost equal to one
/// another up to the specified tolerances
pub fn assert_nearly_equal<T>(a: T, b: T, rel_tol: T, abs_tol: T, msg: &'static str)
where
    T: Float + Zero,
{
    assert!(nearly_equal(a, b, rel_tol, abs_tol), "{}", msg);
}

/// Checks whether two arrays are element-wise nearly equal
///
/// ## Arguments
///
/// - `a` first array
/// - `b` second array
/// - `rel_tol` relative tolerance
/// - `abs_tol` absolute tolerance
///
/// ## Returns
///
/// The function returns true if and only if the application of `nearly_equal`
/// on all elements of the two arrays returns true, i.e., if the two arrays
/// are element-wise almost equal
///
/// ## Panics
///
/// The function will panic in the following cases:
/// - if the specified relative or absolute tolerance is not positive and
/// - if the two arrays have different lengths
///
pub fn nearly_equal_array<T>(a: &[T], b: &[T], rel_tol: T, abs_tol: T) -> bool
where
    T: Float + Zero,
{
    assert!(a.len() == b.len());
    for (&a, &b) in a.iter().zip(b.iter()) {
        if !nearly_equal(a, b, rel_tol, abs_tol) {
            return false;
        }
    }
    true
}

/// Asserts that two given arrays are almost equal
pub fn assert_nearly_equal_array<T>(a: &[T], b: &[T], rel_tol: T, abs_tol: T, msg: &'static str)
where
    T: Float + Zero,
{
    assert!(a.len() == b.len());
    a.iter()
        .zip(b.iter())
        .enumerate()
        .for_each(|(idx, (&ai, &bi))| {
            if !nearly_equal(ai, bi, rel_tol, abs_tol) {
                panic!("({}) arrays not equal at entry {}", msg, idx)
            }
        });
}

/// Checks whether a given array contains any `NaN` elements
///
/// ## Arguments
///
/// - `a` an array of floating-point numbers
///
/// ## Returns
///
/// Returns `true` if and only if there is at least one element which is `NaN`
///
/// ## Panics
///
/// No panics
pub fn is_any_nan<T>(a: &[T]) -> bool
where
    T: Float,
{
    for &a in a.iter() {
        if a.is_nan() {
            return true;
        }
    }
    false
}

/// Asserts that no element of an array is `NaN`
///
/// ## Arguments
///
/// - `a` an array of floating-point numbers
/// - `msg` error name
///
/// ## Panics
///
/// This function will panic if any element of the given array is `NaN`
///
pub fn assert_none_is_nan<T>(a: &[T], msg: &str)
where
    T: Float,
{
    for (idx, &a) in a.iter().enumerate() {
        if a.is_nan() {
            panic!("({}) nan at poisition {}", msg, idx);
        }
    }
}

/// Asserts that all elements in an array are greater than or equal a given value
///
/// ## Arguments
///
/// - `a` given array of floating-point numbers
/// - `lim` the lower bound on the array; all elements must be greater than or equal
///    to `lim`, otherwise the function panics
/// - `msg` error message
///
/// ## Panics
///
/// The function panic if there is at least on element in `a` which is smaller than `lim`
///
pub fn assert_all_ge<T>(a: &[T], lim: T, msg: &str)
where
    T: Float + std::fmt::Display,
{
    for (idx, &a) in a.iter().enumerate() {
        if a < lim {
            panic!("({}) array[{}] = {} is lower than {}", msg, idx, a, lim);
        }
    }
}

/// Asserts that all elements in an array are less than or equal a given value
///
/// ## Arguments
///
/// - `a` given array of floating-point numbers
/// - `lim` the upper bound on the array; all elements must be less than or equal
///    to `lim`, otherwise the function panics
/// - `msg` error message
///
/// ## Panics
///
/// The function panic if there is at least on element in `a` which is greater than `lim`
///
pub fn assert_all_le<T>(a: &[T], lim: T, msg: &str)
where
    T: Float + std::fmt::Display,
{
    for (idx, &a) in a.iter().enumerate() {
        if a > lim {
            panic!("({}) array[{}] = {} is greater than {}", msg, idx, a, lim);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infinities() {
        let a = std::f64::INFINITY;
        let b = std::f64::INFINITY;
        assert!(nearly_equal(a, b, 0.1, 0.1));
    }

    #[test]
    fn nans() {
        let a = std::f64::NAN;
        let b = std::f64::NAN;
        let c = 1.0;
        assert!(!nearly_equal(a, b, 0.1, 0.1));
        assert!(!nearly_equal(a, c, 0.1, 0.1));
    }

    #[test]
    #[should_panic]
    fn no_nonpositive_rel_tol() {
        nearly_equal(5.0, 6.0, 0.0, 1e-7);
    }

    #[test]
    #[should_panic]
    fn no_nonpositive_abs_tol() {
        nearly_equal(5.0, 6.0, 0.01, 0.0);
    }

    #[test]
    fn not_nearly_equal() {
        let a = 1e-8;
        let b = 1e-5;
        assert!(!nearly_equal(a, b, 1e-6, 1e-6))
    }

    #[test]
    fn not_nearly_equal_rel_tol() {
        let a = 1e-14;
        let b = 1e-5;
        assert!(!nearly_equal(a, b, 1e-6, 0.1))
    }

    #[test]
    fn really_nearly_equal() {
        let a = 1.;
        let b = 1. + std::f64::MIN_POSITIVE;
        assert!(nearly_equal(
            a,
            b,
            std::f64::MIN_POSITIVE,
            std::f64::MIN_POSITIVE
        ))
    }

    #[test]
    fn absolutely_equal() {
        let a = 5.;
        let b = 5.;
        assert!(nearly_equal(
            a,
            b,
            std::f64::MIN_POSITIVE,
            std::f64::MIN_POSITIVE
        ))
    }

    #[test]
    fn with_f32() {
        let a = 1000.0_f32;
        let b = 1001.0_f32;
        assert!(nearly_equal(a, b, 0.01, 1.0))
    }

    #[test]
    #[should_panic]
    fn assert_numbers_equal() {
        assert_nearly_equal(1.0, 2.0, 0.01, 0.001, "wtf");
    }

    #[test]
    fn arrays_equal() {
        let x = [1.0, 2.0, 3.0];
        let y = [1.0, 2.0 + 1e-7, 3.0 + 9.9999999e-6];
        assert!(nearly_equal_array(&x, &y, 1e-4, 1e-5));
    }

    #[test]
    fn arrays_not_equal() {
        let x = [1.0, 2.0, 3.0];
        let y = [1.0, 2.0 + 1e-7, 3.0 + 1e-4];
        assert!(!nearly_equal_array(&x, &y, 1e-4, 1e-5));
    }

    #[test]
    fn arrays_identical() {
        let x = [1.0, 2.0, 3.0];
        assert!(nearly_equal_array(&x, &x, 1e-4, 1e-5));
    }

    #[test]
    #[should_panic]
    fn assert_arrays_not_equal() {
        let x = [1.0, 2.0, 3.0];
        let y = [1.0, 2.0 + 1e-7, 3.0 + 1e-4];
        assert_nearly_equal_array(&x, &y, 1e-4, 1e-5, "arrays not equal");
    }

    #[test]
    #[should_panic]
    fn assert_arrays_different_lens() {
        let x = [1.0, 2.0, 3.0];
        let y = [1.0, 2.0 + 1e-7];
        assert_nearly_equal_array(&x, &y, 1e-4, 1e-5, "arrays not equal");
    }

    #[test]
    fn any_is_nan() {
        let x: [f64; 2] = [0.0, 1.0];
        assert!(!is_any_nan(&x));

        let y: [f64; 3] = [0.0, std::f64::NAN, 1.0];
        assert!(is_any_nan(&y));
    }

    #[test]
    #[should_panic]
    fn none_is_none_panic() {
        let y: [f64; 3] = [0.0, std::f64::NAN, 1.0];
        assert_none_is_nan(&y, "y");
    }

    #[test]
    fn assert_all_positive() {
        let y = [0.0, 1e-10, 1e-16];
        assert_all_ge(&y, 0., "y");
    }

    #[test]
    #[should_panic]
    fn assert_all_positive_panic() {
        let y = [0.0, 1e-10, -1e-12, 10.0];
        assert_all_ge(&y, 0., "y");
    }

    #[test]
    fn assert_all_le_one_f32() {
        let y = [0.0_f32, 1.0, 0.5, -100.0];
        assert_all_le(&y, 1.0, "y");
    }

    #[test]
    #[should_panic]
    fn assert_all_le_one_panic() {
        let y = [0.0, 1.0, 1.0 + 4e-16, -100.0];
        assert_all_le(&y, 1.0, "y");
    }
}
