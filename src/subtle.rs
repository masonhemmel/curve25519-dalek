// -*- mode: rust; -*-
//
// To the extent possible under law, the authors have waived all copyright and
// related or neighboring rights to curve25519-dalek, using the Creative
// Commons "CC0" public domain dedication.  See
// <http://creativecommons.org/publicdomain/zero/.0/> for full details.
//
// Authors:
// - Isis Agora Lovecruft <isis@patternsinthevoid.net>
// - Henry de Valence <hdevalence@hdevalence.ca>

//! Constant-time traits and utility functions.

use core::ops::Neg;

/// Trait for items which can be conditionally assigned in constant time.
pub trait CTAssignable {
    /// If `choice == 1u8`, assign `other` to `self`.
    /// Otherwise, leave `self` unchanged.
    /// Executes in constant time.
    fn conditional_assign(&mut self, other: &Self, choice: u8);
}

/// Trait for items whose equality to another item may be tested in constant time.
pub trait CTEq {
    /// Determine if two items are equal in constant time.
    ///
    /// # Returns
    ///
    /// `1u8` if the two items are equal, and `0u8` otherwise.
    fn ct_eq(&self, other: &Self) -> u8;
}

/// Trait for items which can be conditionally negated in constant time.
///
/// Note: it is not necessary to implement this trait, as a generic
/// implementation is provided.
pub trait CTNegatable
{
    /// Conditionally negate an element if `choice == 1u8`.
    fn conditional_negate(&mut self, choice: u8);
}

impl<T> CTNegatable for T
    where T: CTAssignable, for<'a> &'a T: Neg<Output=T>
{
    fn conditional_negate(&mut self, choice: u8) {
        // Need to cast to eliminate mutability
        let self_neg: T = -(self as &T);
        self.conditional_assign(&self_neg, choice);
    }
}

/// Check equality of two bytes in constant time.
///
/// # Return
///
/// Returns `1u8` if `a == b` and `0u8` otherwise.
#[inline(always)]
pub fn bytes_equal_ct(a: u8, b: u8) -> u8 {
    let mut x: u8;

    x  = !(a ^ b);
    x &= x >> 4;
    x &= x >> 2;
    x &= x >> 1;
    x
}

/// Test if a byte is non-zero in constant time.
///
/// ```rust,ignore
/// let mut x: u8;
/// x = 0;
/// assert!(byte_is_nonzero(x));
/// x = 3;
/// assert!(byte_is_nonzero(x) == 1);
/// ```
///
/// # Return
///
/// * If b != 0, returns 1u8.
/// * If b == 0, returns 0u8.
#[inline(always)]
pub fn byte_is_nonzero(b: u8) -> u8 {
    let mut x = b;
    x |= x >> 4;
    x |= x >> 2;
    x |= x >> 1;
    (x & 1)
}

/// Check equality of two 32-byte arrays in constant time.
///
/// # Return
///
/// Returns `1u8` if `a == b` and `0u8` otherwise.
#[inline(always)]
pub fn arrays_equal_ct(a: &[u8; 32], b: &[u8; 32]) -> u8 {
    let mut x: u8 = 0;

    for i in 0..32 {
        x |= a[i] ^ b[i];
    }
    bytes_equal_ct(x, 0)
}

/// Conditionally assign an `other` `u8` to this `this` `u8`, in constant time.
///
/// # Inputs
///
/// * If `choice == 1u8`, assign `other` to `this`.
/// * Otherwise, if `choice == 0u8` leave `this` unchanged.
#[inline(always)]
pub fn conditional_assign_u8(this: &mut u8, other: &u8, choice: &u8) {
    let mask: u8 = -choice;

    this ^= mask & (this ^ other);
}

/// Conditionally assign an `other` `i8` to this `this` `i8`, in constant time.
///
/// # Inputs
///
/// * If `choice == 1u8`, assign `other` to `this`.
/// * Otherwise, if `choice == 0u8` leave `this` unchanged.
#[inline(always)]
pub fn conditional_assign_i8(this: &mut i8, other: &i8, choice: &u8) {
    let mask: u8 = -choice;

    this ^= (mask as i8) & (this ^ other);
}

/// Compute the absolute value of `this` `i8` in constant time.
///
/// # Returns
///
/// An `u8` whose value is the absolute value of `this`, i.e. `|this|`.
pub fn abs_i8(this: &i8) -> u8 {
    let mask: u8 = this.is_negative() as u8;
    let negative: i8 = -this;
    let mut absolute: i8 = *this;

    conditional_assign_i8(&mut absolute, &negative, &(-mask));
    absolute as u8
}
