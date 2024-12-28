// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;
use std::cmp::PartialEq;

#[derive(Debug, Clone, Copy)]
pub struct SaturatingU16(u16);

// Methods
impl SaturatingU16 {
    pub fn new(set_value: u16) -> SaturatingU16 {
        SaturatingU16(set_value)
    }
}


// Overloading Generic Trait From/Into
impl From<u16> for SaturatingU16 {
    fn from(new_value: u16) -> Self {
        SaturatingU16(new_value.into())
    }
}

impl From<u8> for SaturatingU16 {
    fn from(new_value: u8) -> Self {
        SaturatingU16(new_value.into())
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(new_value: &u16) -> Self {
        let n: u16 = *new_value;
        SaturatingU16(n)
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(new_value: &u8) -> Self {
        let n: u8 = *new_value;
        SaturatingU16(n.into())
    }
}

// Operator Overloading: Add, Eq -----------

// Add Trait Overload: u16, &SaturatingU16, &u16, SaturatingU16
impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &Self) -> Self::Output {
        SaturatingU16(self.0.saturating_add(rhs.0))
    }
}

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: SaturatingU16) -> Self::Output {
        SaturatingU16(self.0.saturating_add(rhs.0))
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: u16) -> Self::Output {
        SaturatingU16(self.0.saturating_add(rhs))
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &u16) -> Self::Output {
        SaturatingU16(self.0.saturating_add(*rhs))
    }
}

// PartialEq Overload: Self, u16
impl PartialEq for SaturatingU16 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}