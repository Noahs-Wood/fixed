// Copyright © 2018–2025 Trevor Spiteri

// This library is free software: you can redistribute it and/or
// modify it under the terms of either
//
//   * the Apache License, Version 2.0 or
//   * the MIT License
//
// at your option.
//
// You should have recieved copies of the Apache License and the MIT
// License along with the library. If not, see
// <https://www.apache.org/licenses/LICENSE-2.0> and
// <https://opensource.org/licenses/MIT>.

use crate::from_str::ParseFixedError;
use crate::traits::{Fixed, FixedSigned, FixedUnsigned, FromFixed, ToFixed};
use crate::types::extra::{LeEqU8, LeEqU16, LeEqU32, LeEqU64, LeEqU128};
use crate::{
    FixedI8, FixedI16, FixedI32, FixedI64, FixedI128, FixedU8, FixedU16, FixedU32, FixedU64,
    FixedU128,
};
use core::fmt::{
    Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, Result as FmtResult, UpperExp,
    UpperHex,
};
use core::iter::{Product, Sum};
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Sub, SubAssign,
};
use core::str::FromStr;

/// Provides saturating arithmetic on fixed-point numbers.
///
/// The underlying value can be retrieved through the `.0` index.
///
/// # Examples
///
/// ```rust
/// use fixed::types::I16F16;
/// use fixed::Saturating;
/// let max = Saturating(I16F16::MAX);
/// let delta = Saturating(I16F16::DELTA);
/// assert_eq!(I16F16::MAX, (max + delta).0);
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Saturating<F>(pub F);

impl<F: Fixed> Saturating<F> {
    /// Zero.
    ///
    /// See also <code>FixedI32::[ZERO][FixedI32::ZERO]</code> and
    /// <code>FixedU32::[ZERO][FixedU32::ZERO]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// assert_eq!(Saturating::<I16F16>::ZERO, Saturating(I16F16::ZERO));
    /// ```
    pub const ZERO: Saturating<F> = Saturating(F::ZERO);

    /// The difference between any two successive representable numbers, <i>Δ</i>.
    ///
    /// See also <code>FixedI32::[DELTA][FixedI32::DELTA]</code> and
    /// <code>FixedU32::[DELTA][FixedU32::DELTA]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// assert_eq!(Saturating::<I16F16>::DELTA, Saturating(I16F16::DELTA));
    /// ```
    pub const DELTA: Saturating<F> = Saturating(F::DELTA);

    /// The smallest value that can be represented.
    ///
    /// See also <code>FixedI32::[MIN][FixedI32::MIN]</code> and
    /// <code>FixedU32::[MIN][FixedU32::MIN]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// assert_eq!(Saturating::<I16F16>::MIN, Saturating(I16F16::MIN));
    /// ```
    pub const MIN: Saturating<F> = Saturating(F::MIN);

    /// The largest value that can be represented.
    ///
    /// See also <code>FixedI32::[MAX][FixedI32::MAX]</code> and
    /// <code>FixedU32::[MAX][FixedU32::MAX]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// assert_eq!(Saturating::<I16F16>::MAX, Saturating(I16F16::MAX));
    /// ```
    pub const MAX: Saturating<F> = Saturating(F::MAX);

    /// [`true`] if the type is signed.
    ///
    /// See also <code>FixedI32::[IS\_SIGNED][FixedI32::IS_SIGNED]</code> and
    /// <code>FixedU32::[IS\_SIGNED][FixedU32::IS_SIGNED]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::{I16F16, U16F16};
    /// use fixed::Saturating;
    /// assert!(Saturating::<I16F16>::IS_SIGNED);
    /// assert!(!Saturating::<U16F16>::IS_SIGNED);
    /// ```
    pub const IS_SIGNED: bool = F::IS_SIGNED;

    /// The number of integer bits.
    ///
    /// See also <code>FixedI32::[INT\_NBITS][FixedI32::INT_NBITS]</code> and
    /// <code>FixedU32::[INT\_NBITS][FixedU32::INT_NBITS]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// assert_eq!(Saturating::<I16F16>::INT_NBITS, I16F16::INT_NBITS);
    /// ```
    pub const INT_NBITS: u32 = F::INT_NBITS;

    /// The number of fractional bits.
    ///
    /// See also <code>FixedI32::[FRAC\_NBITS][FixedI32::FRAC_NBITS]</code> and
    /// <code>FixedU32::[FRAC\_NBITS][FixedU32::FRAC_NBITS]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// assert_eq!(Saturating::<I16F16>::FRAC_NBITS, I16F16::FRAC_NBITS);
    /// ```
    pub const FRAC_NBITS: u32 = F::FRAC_NBITS;

    /// Creates a fixed-point number that has a bitwise representation
    /// identical to the given integer.
    ///
    /// See also <code>FixedI32::[from\_bits][FixedI32::from_bits]</code> and
    /// <code>FixedU32::[from\_bits][FixedU32::from_bits]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// assert_eq!(Saturating::<I16F16>::from_bits(0x1C), Saturating(I16F16::from_bits(0x1C)));
    /// ```
    #[inline]
    pub fn from_bits(bits: F::Bits) -> Saturating<F> {
        Saturating(F::from_bits(bits))
    }

    /// Creates an integer that has a bitwise representation identical
    /// to the given fixed-point number.
    ///
    /// See also <code>FixedI32::[to\_bits][FixedI32::to_bits]</code> and
    /// <code>FixedU32::[to\_bits][FixedU32::to_bits]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let w = Saturating(I16F16::from_bits(0x1C));
    /// assert_eq!(w.to_bits(), 0x1C);
    /// ```
    #[inline]
    pub fn to_bits(self) -> F::Bits {
        self.0.to_bits()
    }

    /// Converts a fixed-point number from big endian to the target’s
    /// endianness.
    ///
    /// See also <code>FixedI32::[from\_be][FixedI32::from_be]</code> and
    /// <code>FixedU32::[from\_be][FixedU32::from_be]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let w = Saturating(I16F16::from_bits(0x1234_5678));
    /// if cfg!(target_endian = "big") {
    ///     assert_eq!(Saturating::from_be(w), w);
    /// } else {
    ///     assert_eq!(Saturating::from_be(w), w.swap_bytes());
    /// }
    /// ```
    #[inline]
    pub fn from_be(w: Self) -> Self {
        Saturating(F::from_be(w.0))
    }

    /// Converts a fixed-point number from little endian to the
    /// target’s endianness.
    ///
    /// See also <code>FixedI32::[from\_le][FixedI32::from_le]</code> and
    /// <code>FixedU32::[from\_le][FixedU32::from_le]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let w = Saturating(I16F16::from_bits(0x1234_5678));
    /// if cfg!(target_endian = "little") {
    ///     assert_eq!(Saturating::from_le(w), w);
    /// } else {
    ///     assert_eq!(Saturating::from_le(w), w.swap_bytes());
    /// }
    /// ```
    #[inline]
    pub fn from_le(w: Self) -> Self {
        Saturating(F::from_le(w.0))
    }

    /// Converts `self` to big endian from the target’s endianness.
    ///
    /// See also <code>FixedI32::[to\_be][FixedI32::to_be]</code> and
    /// <code>FixedU32::[to\_be][FixedU32::to_be]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let w = Saturating(I16F16::from_bits(0x1234_5678));
    /// if cfg!(target_endian = "big") {
    ///     assert_eq!(w.to_be(), w);
    /// } else {
    ///     assert_eq!(w.to_be(), w.swap_bytes());
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn to_be(self) -> Self {
        Saturating(self.0.to_be())
    }

    /// Converts `self` to little endian from the target’s endianness.
    ///
    /// See also <code>FixedI32::[to\_le][FixedI32::to_le]</code> and
    /// <code>FixedU32::[to\_le][FixedU32::to_le]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let w = Saturating(I16F16::from_bits(0x1234_5678));
    /// if cfg!(target_endian = "little") {
    ///     assert_eq!(w.to_le(), w);
    /// } else {
    ///     assert_eq!(w.to_le(), w.swap_bytes());
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn to_le(self) -> Self {
        Saturating(self.0.to_le())
    }

    /// Reverses the byte order of the fixed-point number.
    ///
    /// See also <code>FixedI32::[swap\_bytes][FixedI32::swap_bytes]</code> and
    /// <code>FixedU32::[swap\_bytes][FixedU32::swap_bytes]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let w = Saturating(I16F16::from_bits(0x1234_5678));
    /// let swapped = Saturating(I16F16::from_bits(0x7856_3412));
    /// assert_eq!(w.swap_bytes(), swapped);
    /// ```
    #[inline]
    #[must_use]
    pub fn swap_bytes(self) -> Self {
        Saturating(self.0.swap_bytes())
    }

    /// Creates a fixed-point number from its representation
    /// as a byte array in big endian.
    ///
    /// See also
    /// <code>FixedI32::[from\_be\_bytes][FixedI32::from_be_bytes]</code> and
    /// <code>FixedU32::[from\_be\_bytes][FixedU32::from_be_bytes]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let bytes = [0x12, 0x34, 0x56, 0x78];
    /// assert_eq!(
    ///     Saturating::<I16F16>::from_be_bytes(bytes),
    ///     Saturating::<I16F16>::from_bits(0x1234_5678)
    /// );
    /// ```
    #[inline]
    pub fn from_be_bytes(bytes: F::Bytes) -> Self {
        Saturating(F::from_be_bytes(bytes))
    }

    /// Creates a fixed-point number from its representation
    /// as a byte array in little endian.
    ///
    /// See also
    /// <code>FixedI32::[from\_le\_bytes][FixedI32::from_le_bytes]</code> and
    /// <code>FixedU32::[from\_le\_bytes][FixedU32::from_le_bytes]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let bytes = [0x78, 0x56, 0x34, 0x12];
    /// assert_eq!(
    ///     Saturating::<I16F16>::from_le_bytes(bytes),
    ///     Saturating::<I16F16>::from_bits(0x1234_5678)
    /// );
    /// ```
    #[inline]
    pub fn from_le_bytes(bytes: F::Bytes) -> Self {
        Saturating(F::from_le_bytes(bytes))
    }

    /// Creates a fixed-point number from its representation
    /// as a byte array in native endian.
    ///
    /// See also
    /// <code>FixedI32::[from\_ne\_bytes][FixedI32::from_ne_bytes]</code> and
    /// <code>FixedU32::[from\_ne\_bytes][FixedU32::from_ne_bytes]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let bytes = if cfg!(target_endian = "big") {
    ///     [0x12, 0x34, 0x56, 0x78]
    /// } else {
    ///     [0x78, 0x56, 0x34, 0x12]
    /// };
    /// assert_eq!(
    ///     Saturating::<I16F16>::from_ne_bytes(bytes),
    ///     Saturating::<I16F16>::from_bits(0x1234_5678)
    /// );
    /// ```
    #[inline]
    pub fn from_ne_bytes(bytes: F::Bytes) -> Self {
        Saturating(F::from_ne_bytes(bytes))
    }

    /// Returns the memory representation of this fixed-point
    /// number as a byte array in big-endian byte order.
    ///
    /// See also <code>FixedI32::[to\_be\_bytes][FixedI32::to_be_bytes]</code>
    /// and <code>FixedU32::[to\_be\_bytes][FixedU32::to_be_bytes]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// assert_eq!(
    ///     Saturating::<I16F16>::from_bits(0x1234_5678).to_be_bytes(),
    ///     [0x12, 0x34, 0x56, 0x78]
    /// );
    /// ```
    #[inline]
    pub fn to_be_bytes(self) -> F::Bytes {
        self.0.to_be_bytes()
    }

    /// Returns the memory representation of this fixed-point
    /// number as a byte array in little-endian byte order.
    ///
    /// See also <code>FixedI32::[to\_le\_bytes][FixedI32::to_le_bytes]</code>
    /// and <code>FixedU32::[to\_le\_bytes][FixedU32::to_le_bytes]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// assert_eq!(
    ///     Saturating::<I16F16>::from_bits(0x1234_5678).to_le_bytes(),
    ///     [0x78, 0x56, 0x34, 0x12]
    /// );
    /// ```
    #[inline]
    pub fn to_le_bytes(self) -> F::Bytes {
        self.0.to_le_bytes()
    }

    /// Returns the memory representation of this fixed-point
    /// number as a byte array in native-endian byte order.
    ///
    /// See also <code>FixedI32::[to\_ne\_bytes][FixedI32::to_ne_bytes]</code>
    /// and <code>FixedU32::[to\_ne\_bytes][FixedU32::to_ne_bytes]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let bytes = if cfg!(target_endian = "big") {
    ///     [0x12, 0x34, 0x56, 0x78]
    /// } else {
    ///     [0x78, 0x56, 0x34, 0x12]
    /// };
    /// assert_eq!(
    ///     Saturating::<I16F16>::from_bits(0x1234_5678).to_ne_bytes(),
    ///     bytes
    /// );
    /// ```
    #[inline]
    pub fn to_ne_bytes(self) -> F::Bytes {
        self.0.to_ne_bytes()
    }

    /// Saturating conversion from another number.
    ///
    /// The other number can be:
    ///
    ///   * A fixed-point number. Any extra fractional bits are
    ///     discarded, which rounds towards &minus;∞.
    ///   * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    ///     [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    ///     [`usize`].
    ///   * A floating-point number of type
    ///     <code>[half]::[f16][half::f16]</code>,
    ///     <code>[half]::[bf16][half::bf16]</code>, [`f32`], [`f64`] or
    ///     [`F128`]. For this conversion, the method rounds to the nearest,
    ///     with ties rounding to even.
    ///   * Any other number `src` for which [`ToFixed`] is
    ///     implemented, in which case this method returns
    ///     <code>[Saturating]\(src.[saturating\_to\_fixed][ToFixed::saturating_to_fixed]\())</code>.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_from\_num][FixedI32::saturating_from_num]</code>
    /// and
    /// <code>FixedU32::[saturating\_from\_num][FixedU32::saturating_from_num]</code>.
    ///
    /// # Panics
    ///
    /// For floating-point numbers, panics if the value is NaN.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::{I4F4, I16F16};
    /// use fixed::Saturating;
    ///
    /// let src = I16F16::from_bits(0x1234_5678);
    /// let dst = Saturating::<I4F4>::from_num(src);
    /// assert_eq!(dst, Saturating(I4F4::MAX));
    ///
    /// let src_int = 0x1234_i32;
    /// let dst_int = Saturating::<I4F4>::from_num(src_int);
    /// assert_eq!(dst_int, Saturating(I4F4::MAX));
    ///
    /// let src_float = f64::NEG_INFINITY;
    /// let dst_float = Saturating::<I4F4>::from_num(src_float);
    /// assert_eq!(dst_float, Saturating(I4F4::MIN));
    /// ```
    ///
    /// [`F128`]: crate::F128
    #[inline]
    #[track_caller]
    pub fn from_num<Src: ToFixed>(src: Src) -> Saturating<F> {
        Saturating(src.saturating_to_fixed())
    }

    /// Converts a fixed-point number to another number, saturating the
    /// value on overflow.
    ///
    /// The other number can be:
    ///
    ///   * Another fixed-point number. Any extra fractional bits are
    ///     discarded, which rounds towards &minus;∞.
    ///   * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    ///     [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    ///     [`usize`]. Any fractional bits are discarded, which rounds
    ///     towards &minus;∞.
    ///   * A floating-point number of type
    ///     <code>[half]::[f16][half::f16]</code>,
    ///     <code>[half]::[bf16][half::bf16]</code>, [`f32`], [`f64`] or
    ///     [`F128`]. For this conversion, the method rounds to the nearest,
    ///     with ties rounding to even.
    ///   * Any other type `Dst` for which [`FromFixed`] is
    ///     implemented, in which case this method returns
    ///     <code>Dst::[saturating\_from\_fixed][FromFixed::saturating_from_fixed]\(self.0)</code>.
    ///
    /// See also <code>FixedI32::[saturating\_to\_num][FixedI32::saturating_to_num]</code> and
    /// <code>FixedU32::[saturating\_to\_num][FixedU32::saturating_to_num]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::{I16F16, I2F6, I4F4};
    /// use fixed::Saturating;
    ///
    /// // conversion that fits
    /// let src = Saturating(I4F4::from_num(1.75));
    /// let expected = I16F16::from_num(1.75);
    /// assert_eq!(src.to_num::<I16F16>(), expected);
    ///
    /// // conversion that saturates
    /// let src = Saturating(I4F4::MAX);
    /// assert_eq!(src.to_num::<I2F6>(), I2F6::MAX);
    /// ```
    ///
    /// [`F128`]: crate::F128
    #[inline]
    pub fn to_num<Dst: FromFixed>(self) -> Dst {
        Dst::saturating_from_fixed(self.0)
    }

    /// Parses a string slice containing binary digits to return a fixed-point number.
    ///
    /// Rounding is to the nearest, with ties rounded to even.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_from\_str\_binary][FixedI32::saturating_from_str_binary]</code>
    /// and
    /// <code>FixedU32::[saturating\_from\_str\_binary][FixedU32::saturating_from_str_binary]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I8F8;
    /// use fixed::Saturating;
    /// let max = Saturating(I8F8::MAX);
    /// assert_eq!(Saturating::<I8F8>::from_str_binary("101100111000.1"), Ok(max));
    /// ```
    #[inline]
    pub fn from_str_binary(src: &str) -> Result<Saturating<F>, ParseFixedError> {
        F::saturating_from_str_binary(src).map(Saturating)
    }

    /// Parses a string slice containing octal digits to return a fixed-point number.
    ///
    /// Rounding is to the nearest, with ties rounded to even.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_from\_str\_octal][FixedI32::saturating_from_str_octal]</code>
    /// and
    /// <code>FixedU32::[saturating\_from\_str\_octal][FixedU32::saturating_from_str_octal]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I8F8;
    /// use fixed::Saturating;
    /// let max = Saturating(I8F8::MAX);
    /// assert_eq!(Saturating::<I8F8>::from_str_octal("7165.4"), Ok(max));
    /// ```
    #[inline]
    pub fn from_str_octal(src: &str) -> Result<Saturating<F>, ParseFixedError> {
        F::saturating_from_str_octal(src).map(Saturating)
    }

    /// Parses a string slice containing hexadecimal digits to return a fixed-point number.
    ///
    /// Rounding is to the nearest, with ties rounded to even.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_from\_str\_hex][FixedI32::saturating_from_str_hex]</code>
    /// and
    /// <code>FixedU32::[saturating\_from\_str\_hex][FixedU32::saturating_from_str_hex]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I8F8;
    /// use fixed::Saturating;
    /// let max = Saturating(I8F8::MAX);
    /// assert_eq!(Saturating::<I8F8>::from_str_hex("C0F.FE"), Ok(max));
    /// ```
    #[inline]
    pub fn from_str_hex(src: &str) -> Result<Saturating<F>, ParseFixedError> {
        F::saturating_from_str_hex(src).map(Saturating)
    }

    /// Parses an ASCII-byte slice containing decimal digits to return a fixed-point number.
    ///
    /// Rounding is to the nearest, with ties rounded to even.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_from\_ascii][FixedI32::saturating_from_ascii]</code>
    /// and
    /// <code>FixedU32::[saturating\_from\_ascii][FixedU32::saturating_from_ascii]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I8F8;
    /// use fixed::Saturating;
    /// let max = Saturating(I8F8::MAX);
    /// assert_eq!(Saturating::<I8F8>::from_ascii(b"9999"), Ok(max));
    /// ```
    #[inline]
    pub fn from_ascii(src: &[u8]) -> Result<Saturating<F>, ParseFixedError> {
        F::saturating_from_ascii(src).map(Saturating)
    }

    /// Parses an ASCII-byte slice containing binary digits to return a fixed-point number.
    ///
    /// Rounding is to the nearest, with ties rounded to even.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_from\_ascii\_binary][FixedI32::saturating_from_ascii_binary]</code>
    /// and
    /// <code>FixedU32::[saturating\_from\_ascii\_binary][FixedU32::saturating_from_ascii_binary]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I8F8;
    /// use fixed::Saturating;
    /// let max = Saturating(I8F8::MAX);
    /// assert_eq!(Saturating::<I8F8>::from_ascii_binary(b"101100111000.1"), Ok(max));
    /// ```
    #[inline]
    pub fn from_ascii_binary(src: &[u8]) -> Result<Saturating<F>, ParseFixedError> {
        F::saturating_from_ascii_binary(src).map(Saturating)
    }

    /// Parses an ASCII-byte slice containing octal digits to return a fixed-point number.
    ///
    /// Rounding is to the nearest, with ties rounded to even.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_from\_ascii\_octal][FixedI32::saturating_from_ascii_octal]</code>
    /// and
    /// <code>FixedU32::[saturating\_from\_ascii\_octal][FixedU32::saturating_from_ascii_octal]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I8F8;
    /// use fixed::Saturating;
    /// let max = Saturating(I8F8::MAX);
    /// assert_eq!(Saturating::<I8F8>::from_ascii_octal(b"7165.4"), Ok(max));
    /// ```
    #[inline]
    pub fn from_ascii_octal(src: &[u8]) -> Result<Saturating<F>, ParseFixedError> {
        F::saturating_from_ascii_octal(src).map(Saturating)
    }

    /// Parses an ASCII-byte slice containing hexadecimal digits to return a fixed-point number.
    ///
    /// Rounding is to the nearest, with ties rounded to even.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_from\_ascii\_hex][FixedI32::saturating_from_ascii_hex]</code>
    /// and
    /// <code>FixedU32::[saturating\_from\_ascii\_hex][FixedU32::saturating_from_ascii_hex]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I8F8;
    /// use fixed::Saturating;
    /// let max = Saturating(I8F8::MAX);
    /// assert_eq!(Saturating::<I8F8>::from_ascii_hex(b"C0F.FE"), Ok(max));
    /// ```
    #[inline]
    pub fn from_ascii_hex(src: &[u8]) -> Result<Saturating<F>, ParseFixedError> {
        F::saturating_from_ascii_hex(src).map(Saturating)
    }

    /// Returns the integer part.
    ///
    /// Note that since the numbers are stored in two’s complement,
    /// negative numbers with non-zero fractional parts will be
    /// rounded towards &minus;∞, except in the case where there are no
    /// integer bits, for example for the type
    /// <code>[Saturating]&lt;[I0F16]&gt;</code>, where the return value
    /// is always zero.
    ///
    /// See also <code>FixedI32::[int][FixedI32::int]</code> and
    /// <code>FixedU32::[int][FixedU32::int]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// assert_eq!(Saturating(I16F16::from_num(12.25)).int(), Saturating(I16F16::from_num(12)));
    /// assert_eq!(Saturating(I16F16::from_num(-12.25)).int(), Saturating(I16F16::from_num(-13)));
    /// ```
    ///
    /// [I0F16]: crate::types::I0F16
    #[inline]
    #[must_use]
    pub fn int(self) -> Saturating<F> {
        Saturating(self.0.int())
    }

    /// Returns the fractional part.
    ///
    /// Note that since the numbers are stored in two’s complement,
    /// the returned fraction will be non-negative for negative
    /// numbers, except in the case where there are no integer bits,
    /// for example for the type
    /// <code>[Saturating]&lt;[I0F16]&gt;</code>,
    /// where the return value is always equal to `self`.
    ///
    /// See also <code>FixedI32::[frac][FixedI32::frac]</code> and
    /// <code>FixedU32::[frac][FixedU32::frac]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// assert_eq!(Saturating(I16F16::from_num(12.25)).frac(), Saturating(I16F16::from_num(0.25)));
    /// assert_eq!(Saturating(I16F16::from_num(-12.25)).frac(), Saturating(I16F16::from_num(0.75)));
    /// ```
    ///
    /// [I0F16]: crate::types::I0F16
    #[inline]
    #[must_use]
    pub fn frac(self) -> Saturating<F> {
        Saturating(self.0.frac())
    }

    /// Rounds to the next integer towards 0.
    ///
    /// See also
    /// <code>FixedI32::[round\_to\_zero][FixedI32::round_to_zero]</code> and
    /// <code>FixedU32::[round\_to\_zero][FixedU32::round_to_zero]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let three = Saturating(I16F16::from_num(3));
    /// assert_eq!(Saturating(I16F16::from_num(3.9)).round_to_zero(), three);
    /// assert_eq!(Saturating(I16F16::from_num(-3.9)).round_to_zero(), -three);
    /// ```
    #[inline]
    #[must_use]
    pub fn round_to_zero(self) -> Saturating<F> {
        Saturating(self.0.round_to_zero())
    }

    /// Saturating ceil. Rounds to the next integer towards +∞, saturating
    /// on overflow.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_ceil][FixedI32::saturating_ceil]</code> and
    /// <code>FixedU32::[saturating\_ceil][FixedU32::saturating_ceil]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let two_half = Saturating(I16F16::from_num(5) / 2);
    /// assert_eq!(two_half.ceil(), Saturating(I16F16::from_num(3)));
    /// assert_eq!(Saturating(I16F16::MAX).ceil(), Saturating(I16F16::MAX));
    /// ```
    #[inline]
    #[must_use]
    pub fn ceil(self) -> Saturating<F> {
        Saturating(self.0.saturating_ceil())
    }

    /// Saturating floor. Rounds to the next integer towards &minus;∞,
    /// saturating on overflow.
    ///
    /// Overflow can only occur for signed numbers with zero integer
    /// bits.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_floor][FixedI32::saturating_floor]</code> and
    /// <code>FixedU32::[saturating\_floor][FixedU32::saturating_floor]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::{I0F32, I16F16};
    /// use fixed::Saturating;
    /// let two_half = Saturating(I16F16::from_num(5) / 2);
    /// assert_eq!(two_half.floor(), Saturating(I16F16::from_num(2)));
    /// assert_eq!(Saturating(I0F32::MIN).floor(), Saturating(I0F32::MIN));
    /// ```
    #[inline]
    #[must_use]
    pub fn floor(self) -> Saturating<F> {
        Saturating(self.0.saturating_floor())
    }

    /// Saturating round. Rounds to the next integer to the nearest,
    /// with ties rounded away from zero, and saturating on overflow.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_round][FixedI32::saturating_round]</code> and
    /// <code>FixedU32::[saturating\_round][FixedU32::saturating_round]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let two_half = Saturating(I16F16::from_num(5) / 2);
    /// assert_eq!(two_half.round(), Saturating(I16F16::from_num(3)));
    /// assert_eq!((-two_half).round(), Saturating(I16F16::from_num(-3)));
    /// let max = Saturating(I16F16::MAX);
    /// assert_eq!(max.round(), max);
    /// ```
    #[inline]
    #[must_use]
    pub fn round(self) -> Saturating<F> {
        Saturating(self.0.saturating_round())
    }

    /// Saturating round. Rounds to the next integer to the nearest, with ties
    /// rounded to even, and saturating on overflow.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_round\_ties\_even][FixedI32::saturating_round_ties_even]</code>
    /// and
    /// <code>FixedU32::[saturating\_round\_ties\_even][FixedU32::saturating_round_ties_even]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let two_half = Saturating(I16F16::from_num(2.5));
    /// assert_eq!(two_half.round_ties_even(), Saturating(I16F16::from_num(2)));
    /// let three_half = Saturating(I16F16::from_num(3.5));
    /// assert_eq!(three_half.round_ties_even(), Saturating(I16F16::from_num(4)));
    /// let max = Saturating(I16F16::MAX);
    /// assert_eq!(max.round_ties_even(), max);
    /// ```
    #[inline]
    #[must_use]
    pub fn round_ties_even(self) -> Saturating<F> {
        Saturating(self.0.saturating_round_ties_even())
    }

    /// Returns the number of ones in the binary representation.
    ///
    /// See also <code>FixedI32::[count\_ones][FixedI32::count_ones]</code> and
    /// <code>FixedU32::[count\_ones][FixedU32::count_ones]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let w = Saturating(I16F16::from_bits(0x00FF_FF00));
    /// assert_eq!(w.count_ones(), w.0.count_ones());
    /// ```
    #[inline]
    #[doc(alias("popcount", "popcnt"))]
    pub fn count_ones(self) -> u32 {
        self.0.count_ones()
    }

    /// Returns the number of zeros in the binary representation.
    ///
    /// See also <code>FixedI32::[count\_zeros][FixedI32::count_zeros]</code>
    /// and <code>FixedU32::[count\_zeros][FixedU32::count_zeros]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let w = Saturating(I16F16::from_bits(0x00FF_FF00));
    /// assert_eq!(w.count_zeros(), w.0.count_zeros());
    /// ```
    #[inline]
    pub fn count_zeros(self) -> u32 {
        self.0.count_zeros()
    }

    /// Returns the number of leading ones in the binary representation.
    ///
    /// See also <code>FixedI32::[leading\_ones][FixedI32::leading_ones]</code>
    /// and <code>FixedU32::[leading\_ones][FixedU32::leading_ones]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::U16F16;
    /// use fixed::Saturating;
    /// let w = Saturating(U16F16::from_bits(0xFF00_00FF));
    /// assert_eq!(w.leading_ones(), w.0.leading_ones());
    /// ```
    #[inline]
    pub fn leading_ones(self) -> u32 {
        self.0.leading_ones()
    }

    /// Returns the number of leading zeros in the binary representation.
    ///
    /// See also
    /// <code>FixedI32::[leading\_zeros][FixedI32::leading_zeros]</code> and
    /// <code>FixedU32::[leading\_zeros][FixedU32::leading_zeros]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let w = Saturating(I16F16::from_bits(0x00FF_FF00));
    /// assert_eq!(w.leading_zeros(), w.0.leading_zeros());
    /// ```
    #[inline]
    pub fn leading_zeros(self) -> u32 {
        self.0.leading_zeros()
    }

    /// Returns the number of trailing ones in the binary representation.
    ///
    /// See also
    /// <code>FixedI32::[trailing\_ones][FixedI32::trailing_ones]</code> and
    /// <code>FixedU32::[trailing\_ones][FixedU32::trailing_ones]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::U16F16;
    /// use fixed::Saturating;
    /// let w = Saturating(U16F16::from_bits(0xFF00_00FF));
    /// assert_eq!(w.trailing_ones(), w.0.trailing_ones());
    /// ```
    #[inline]
    pub fn trailing_ones(self) -> u32 {
        self.0.trailing_ones()
    }

    /// Returns the number of trailing zeros in the binary representation.
    ///
    /// See also
    /// <code>FixedI32::[trailing\_zeros][FixedI32::trailing_zeros]</code> and
    /// <code>FixedU32::[trailing\_zeros][FixedU32::trailing_zeros]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let w = Saturating(I16F16::from_bits(0x00FF_FF00));
    /// assert_eq!(w.trailing_zeros(), w.0.trailing_zeros());
    /// ```
    #[inline]
    pub fn trailing_zeros(self) -> u32 {
        self.0.trailing_zeros()
    }

    /// Returns the square root.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_sqrt][FixedI32::saturating_sqrt]</code> and
    /// <code>FixedU32::[saturating\_sqrt][FixedU32::saturating_sqrt]</code>.
    ///
    /// # Panics
    ///
    /// Panics if the number is negative.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I0F32;
    /// use fixed::Saturating;
    /// assert_eq!(Saturating(I0F32::lit("0b0.0001")).sqrt().0, I0F32::lit("0b0.01"));
    ///
    /// // This method handles the overflow corner case.
    /// let s = Saturating(I0F32::from_num(0.25));
    /// assert_eq!(s.sqrt().0, I0F32::MAX);
    /// ```
    #[inline]
    #[track_caller]
    pub fn sqrt(self) -> Self {
        Saturating(self.0.saturating_sqrt())
    }

    /// Integer base-2 logarithm, rounded down.
    ///
    /// See also <code>FixedI32::[int\_log2][FixedI32::int_log2]</code> and
    /// <code>FixedU32::[int\_log2][FixedU32::int_log2]</code>.
    ///
    /// # Panics
    ///
    /// Panics if the fixed-point number is ≤&nbsp;0.
    #[inline]
    #[track_caller]
    #[doc(alias("ilog2"))]
    pub fn int_log2(self) -> i32 {
        self.0.int_log2()
    }

    /// Integer base-10 logarithm, rounded down.
    ///
    /// See also <code>FixedI32::[int\_log10][FixedI32::int_log10]</code> and
    /// <code>FixedU32::[int\_log10][FixedU32::int_log10]</code>.
    ///
    /// # Panics
    ///
    /// Panics if the fixed-point number is ≤&nbsp;0.
    #[inline]
    #[track_caller]
    #[doc(alias("ilog10"))]
    pub fn int_log10(self) -> i32 {
        self.0.int_log10()
    }

    /// Integer logarithm to the specified base, rounded down.
    ///
    /// See also <code>FixedI32::[int\_log][FixedI32::int_log]</code> and
    /// <code>FixedU32::[int\_log][FixedU32::int_log]</code>.
    ///
    /// # Panics
    ///
    /// Panics if the fixed-point number is ≤&nbsp;0 or if the base is <&nbsp;2.
    #[inline]
    #[track_caller]
    #[doc(alias("ilog"))]
    pub fn int_log(self, base: u32) -> i32 {
        self.0.int_log(base)
    }

    /// Reverses the order of the bits of the fixed-point number.
    ///
    /// See also <code>FixedI32::[reverse\_bits][FixedI32::reverse_bits]</code>
    /// and <code>FixedU32::[reverse\_bits][FixedU32::reverse_bits]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let i = I16F16::from_bits(0x1234_5678);
    /// assert_eq!(Saturating(i).reverse_bits(), Saturating(i.reverse_bits()));
    /// ```
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn reverse_bits(self) -> Saturating<F> {
        Saturating(self.0.reverse_bits())
    }

    /// Shifts to the left by `n` bits, saturating the truncated bits to the right end.
    ///
    /// See also <code>FixedI32::[rotate\_left][FixedI32::rotate_left]</code>
    /// and <code>FixedU32::[rotate\_left][FixedU32::rotate_left]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let i = I16F16::from_bits(0x00FF_FF00);
    /// assert_eq!(Saturating(i).rotate_left(12), Saturating(i.rotate_left(12)));
    /// ```
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn rotate_left(self, n: u32) -> Saturating<F> {
        Saturating(self.0.rotate_left(n))
    }

    /// Shifts to the right by `n` bits, saturating the truncated bits to the left end.
    ///
    /// See also <code>FixedI32::[rotate\_right][FixedI32::rotate_right]</code>
    /// and <code>FixedU32::[rotate\_right][FixedU32::rotate_right]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let i = I16F16::from_bits(0x00FF_FF00);
    /// assert_eq!(Saturating(i).rotate_right(12), Saturating(i.rotate_right(12)));
    /// ```
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn rotate_right(self, n: u32) -> Saturating<F> {
        Saturating(self.0.rotate_right(n))
    }

    /// Returns [`true`] if the number is zero.
    ///
    /// See also <code>FixedI32::[is\_zero][FixedI32::is_zero]</code> and
    /// <code>FixedU32::[is\_zero][FixedU32::is_zero]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// assert!(Saturating(I16F16::ZERO).is_zero());
    /// assert!(!Saturating(I16F16::from_num(4.3)).is_zero());
    /// ```
    #[inline]
    pub fn is_zero(self) -> bool {
        self.0.is_zero()
    }

    /// Returns the distance from `self` to `other`.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_dist][FixedI32::saturating_dist]</code> and
    /// <code>FixedU32::[saturating\_dist][FixedU32::saturating_dist]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// type Wr = Saturating<I16F16>;
    /// assert_eq!(Wr::from_num(-1).dist(Wr::from_num(4)), Wr::from_num(5));
    /// assert_eq!(Wr::MIN.dist(Wr::MAX), Wr::MAX);
    /// ```
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn dist(self, other: Saturating<F>) -> Saturating<F> {
        Saturating(self.0.saturating_dist(other.0))
    }

    /// Returns the mean of `self` and `other`.
    ///
    /// See also <code>FixedI32::[mean][FixedI32::mean]</code> and
    /// <code>FixedU32::[mean][FixedU32::mean]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let three = Saturating(I16F16::from_num(3));
    /// let four = Saturating(I16F16::from_num(4));
    /// assert_eq!(three.mean(four), Saturating(I16F16::from_num(3.5)));
    /// assert_eq!(three.mean(-four), Saturating(I16F16::from_num(-0.5)));
    /// ```
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn mean(self, other: Saturating<F>) -> Saturating<F> {
        Saturating(self.0.mean(other.0))
    }

    /// Compute the hypotenuse of a right triange.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_hypot][FixedI32::saturating_hypot]</code> and
    /// <code>FixedU32::[saturating\_hypot][FixedU32::saturating_hypot]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I8F8;
    /// use fixed::Saturating;
    /// type Sa = Saturating<I8F8>;
    /// // hypot(3, 4) == 5
    /// assert_eq!(Sa::from_num(3).hypot(Sa::from_num(4)), Sa::from_num(5));
    /// // hypot(88, 105) == 137, which saturates
    /// assert_eq!(Sa::from_num(88).hypot(Sa::from_num(105)), Sa::MAX);
    /// ```
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn hypot(self, other: Saturating<F>) -> Saturating<F> {
        Saturating(self.0.saturating_hypot(other.0))
    }

    /// Returns the reciprocal (inverse), 1/`self`.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_recip][FixedI32::saturating_recip]</code> and
    /// <code>FixedU32::[saturating\_recip][FixedU32::saturating_recip]</code>.
    ///
    /// # Panics
    ///
    /// Panics if `self` is zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I8F24;
    /// use fixed::Saturating;
    /// let quarter = Saturating(I8F24::from_num(0.25));
    /// let frac_1_512 = Saturating(I8F24::ONE / 512);
    /// assert_eq!(quarter.recip(), Saturating(I8F24::from_num(4)));
    /// assert_eq!(frac_1_512.recip(), Saturating(I8F24::MAX));
    /// ```
    #[inline]
    #[track_caller]
    #[must_use]
    pub fn recip(self) -> Saturating<F> {
        Saturating(self.0.saturating_recip())
    }

    /// Returns the next multiple of `other`.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_next\_multiple\_of][FixedI32::saturating_next_multiple_of]</code>
    /// and
    /// <code>FixedU32::[saturating\_next\_multiple\_of][FixedU32::saturating_next_multiple_of]</code>.
    ///
    /// # Panics
    ///
    /// Panics if `other` is zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let one_point_5 = Saturating::<I16F16>::from_num(1.5);
    /// let four = Saturating::<I16F16>::from_num(4);
    /// let four_point_5 = Saturating::<I16F16>::from_num(4.5);
    /// assert_eq!(four.next_multiple_of(one_point_5), four_point_5);
    ///
    /// let max = Saturating::<I16F16>::MAX;
    /// let max_minus_delta = max - Saturating::<I16F16>::DELTA;
    /// assert_eq!(max.next_multiple_of(max_minus_delta), max_minus_delta * 2);
    /// ```
    #[inline]
    #[track_caller]
    #[must_use]
    pub fn next_multiple_of(self, other: Saturating<F>) -> Saturating<F> {
        Saturating(self.0.saturating_next_multiple_of(other.0))
    }

    /// Multiply and add. Returns `self` × `mul` + `add`.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_mul\_add][FixedI32::saturating_mul_add]</code>
    /// and
    /// <code>FixedU32::[saturating\_mul\_add][FixedU32::saturating_mul_add]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let half = Saturating(I16F16::from_num(0.5));
    /// let three = Saturating(I16F16::from_num(3));
    /// let four = Saturating(I16F16::from_num(4));
    /// let max = Saturating(I16F16::MAX);
    /// assert_eq!(three.mul_add(half, four), Saturating(I16F16::from_num(5.5)));
    /// assert_eq!(max.mul_add(three, max), max * 4);
    /// ```
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn mul_add(self, mul: Saturating<F>, add: Saturating<F>) -> Saturating<F> {
        Saturating(self.0.saturating_mul_add(mul.0, add.0))
    }

    /// Adds `self` to the product `a`&nbsp;×&nbsp;`b`.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_add\_prod][FixedI32::saturating_add_prod]</code>
    /// and
    /// <code>FixedU32::[saturating\_add\_prod][FixedU32::saturating_add_prod]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let half = Saturating(I16F16::from_num(0.5));
    /// let three = Saturating(I16F16::from_num(3));
    /// let four = Saturating(I16F16::from_num(4));
    /// let max = Saturating(I16F16::MAX);
    /// assert_eq!(four.add_prod(three, half), Saturating(I16F16::from_num(5.5)));
    /// assert_eq!(max.add_prod(max, three), max * 4);
    /// ```
    #[inline]
    #[must_use]
    pub fn add_prod(self, a: Saturating<F>, b: Saturating<F>) -> Saturating<F> {
        Saturating(self.0.saturating_add_prod(a.0, b.0))
    }

    /// Multiply and accumulate. Adds (`a` × `b`) to `self`.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_mul\_acc][FixedI32::saturating_mul_acc]</code>
    /// and
    /// <code>FixedU32::[saturating\_mul\_acc][FixedU32::saturating_mul_acc]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let mut acc = Saturating(I16F16::from_num(3));
    /// acc.mul_acc(Saturating(I16F16::from_num(4)), Saturating(I16F16::from_num(0.5)));
    /// assert_eq!(acc, Saturating(I16F16::from_num(5)));
    ///
    /// acc = Saturating(I16F16::MAX);
    /// acc.mul_acc(Saturating(I16F16::MAX), Saturating(I16F16::from_num(3)));
    /// assert_eq!(acc, Saturating(I16F16::MAX) * 4);
    /// ```
    #[inline]
    pub fn mul_acc(&mut self, a: Saturating<F>, b: Saturating<F>) {
        self.0.saturating_mul_acc(a.0, b.0);
    }

    /// Euclidean division.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_div\_euclid][FixedI32::saturating_div_euclid]</code>
    /// and
    /// <code>FixedU32::[saturating\_div\_euclid][FixedU32::saturating_div_euclid]</code>.
    ///
    /// # Panics
    ///
    /// Panics if the divisor is zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let num = Saturating(I16F16::from_num(7.5));
    /// let den = Saturating(I16F16::from_num(2));
    /// assert_eq!(num.div_euclid(den), Saturating(I16F16::from_num(3)));
    /// let quarter = Saturating(I16F16::from_num(0.25));
    /// let max = Saturating(I16F16::MAX);
    /// assert_eq!(max.div_euclid(quarter), max);
    /// ```
    #[inline]
    #[track_caller]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn div_euclid(self, divisor: Saturating<F>) -> Saturating<F> {
        Saturating(self.0.saturating_div_euclid(divisor.0))
    }

    /// Remainder for Euclidean division.
    ///
    /// See also <code>FixedI32::[rem\_euclid][FixedI32::rem_euclid]</code> and
    /// <code>FixedU32::[rem\_euclid][FixedU32::rem_euclid]</code>.
    ///
    /// # Panics
    ///
    /// Panics if the divisor is zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let num = Saturating(I16F16::from_num(7.5));
    /// let den = Saturating(I16F16::from_num(2));
    /// assert_eq!(num.rem_euclid(den), Saturating(I16F16::from_num(1.5)));
    /// assert_eq!((-num).rem_euclid(den), Saturating(I16F16::from_num(0.5)));
    /// ```
    #[inline]
    #[track_caller]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn rem_euclid(self, divisor: Saturating<F>) -> Saturating<F> {
        Saturating(self.0.rem_euclid(divisor.0))
    }

    /// Euclidean division by an integer.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_div\_euclid\_int][FixedI32::saturating_div_euclid_int]</code>
    /// and
    /// <code>FixedU32::[saturating\_div\_euclid\_int][FixedU32::saturating_div_euclid_int]</code>.
    ///
    /// # Panics
    ///
    /// Panics if the divisor is zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let num = Saturating(I16F16::from_num(7.5));
    /// assert_eq!(num.div_euclid_int(2), Saturating(I16F16::from_num(3)));
    /// let min = Saturating(I16F16::MIN);
    /// let max = Saturating(I16F16::MAX);
    /// assert_eq!(min.div_euclid_int(-1), max);
    /// ```
    #[inline]
    #[track_caller]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn div_euclid_int(self, divisor: F::Bits) -> Saturating<F> {
        Saturating(self.0.saturating_div_euclid_int(divisor))
    }

    /// Remainder for Euclidean division.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_rem\_euclid\_int][FixedI32::saturating_rem_euclid_int]</code>
    /// and
    /// <code>FixedU32::[saturating\_rem\_euclid\_int][FixedU32::saturating_rem_euclid_int]</code>.
    ///
    /// # Panics
    ///
    /// Panics if the divisor is zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// let num = Saturating(I16F16::from_num(7.5));
    /// assert_eq!(num.rem_euclid_int(2), Saturating(I16F16::from_num(1.5)));
    /// assert_eq!((-num).rem_euclid_int(2), Saturating(I16F16::from_num(0.5)));
    /// ```
    #[inline]
    #[track_caller]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn rem_euclid_int(self, divisor: F::Bits) -> Saturating<F> {
        Saturating(self.0.saturating_rem_euclid_int(divisor))
    }

    /// Unbounded shift left. Computes `self << rhs`, without bounding the value
    /// of `rhs`.
    ///
    /// See also
    /// <code>FixedI32::[unbounded\_shl][FixedI32::unbounded_shl]</code> and
    /// <code>FixedU32::[unbounded\_shl][FixedU32::unbounded_shl]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// type Sa = Saturating<I16F16>;
    /// let num = Sa::from_num(1.5);
    /// assert_eq!(num.unbounded_shl(5), Saturating(num.0 << 5));
    /// assert_eq!(num.unbounded_shl(32), Sa::ZERO);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn unbounded_shl(self, rhs: u32) -> Saturating<F> {
        Saturating(self.0.unbounded_shl(rhs))
    }

    /// Unbounded shift right. Computes `self >> rhs`, without bounding the
    /// value of `rhs`.
    ///
    /// See also
    /// <code>FixedI32::[unbounded\_shr][FixedI32::unbounded_shr]</code> and
    /// <code>FixedU32::[unbounded\_shr][FixedU32::unbounded_shr]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// type Sa = Saturating<I16F16>;
    /// let num = Sa::from_num(1.5);
    /// assert_eq!(num.unbounded_shr(5), Saturating(num.0 >> 5));
    /// assert_eq!(num.unbounded_shr(32), Sa::ZERO);
    /// assert_eq!((-num).unbounded_shr(5), Saturating((-num.0) >> 5));
    /// assert_eq!((-num).unbounded_shr(32), -Sa::DELTA);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[inline]
    pub fn unbounded_shr(self, rhs: u32) -> Saturating<F> {
        Saturating(self.0.unbounded_shr(rhs))
    }

    /// Linear interpolation between `start` and `end`.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_lerp][FixedI32::saturating_lerp]</code> and
    /// <code>FixedU32::[saturating\_lerp][FixedU32::saturating_lerp]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// type Sa = Saturating<I16F16>;
    /// assert_eq!(Sa::from_num(0.5).lerp(Sa::ZERO, Sa::MAX), Sa::MAX / 2);
    /// assert_eq!(Sa::from_num(1.5).lerp(Sa::ZERO, Sa::MAX), Sa::MAX + Sa::MAX / 2);
    /// ```
    #[inline]
    #[must_use]
    pub fn lerp(self, start: Saturating<F>, end: Saturating<F>) -> Saturating<F> {
        Saturating(self.0.saturating_lerp(start.0, end.0))
    }

    /// Inverse linear interpolation between `start` and `end`.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_inv\_lerp][FixedI32::saturating_inv_lerp]</code> and
    /// <code>FixedU32::[saturating\_inv\_lerp][FixedU32::saturating_inv_lerp]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// type Sa = Saturating<I16F16>;
    /// assert_eq!(
    ///     Sa::from_num(25).inv_lerp(Sa::from_num(20), Sa::from_num(40)),
    ///     Sa::from_num(0.25)
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn inv_lerp(self, start: Saturating<F>, end: Saturating<F>) -> Saturating<F> {
        Saturating(self.0.saturating_inv_lerp(start.0, end.0))
    }

    /// Saturating round. Rounds to the next integer to the nearest, with ties
    /// rounded to even, and saturating on overflow.
    #[inline]
    #[must_use]
    #[deprecated(since = "1.28.0", note = "renamed to `round_ties_even`")]
    pub fn round_ties_to_even(self) -> Saturating<F> {
        self.round_ties_even()
    }
}

impl<F: FixedSigned> Saturating<F> {
    /// Returns the bit pattern of `self` reinterpreted as an unsigned
    /// fixed-point number of the same size.
    ///
    /// See also <code>FixedI32::[cast\_unsigned][FixedU32::cast_unsigned]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::{I16F16, U16F16};
    /// use fixed::Saturating;
    ///
    /// let n = Saturating(-I16F16::DELTA);
    /// assert_eq!(n.cast_unsigned(), Saturating(U16F16::MAX));
    /// ```
    #[must_use]
    #[inline]
    pub fn cast_unsigned(self) -> Saturating<F::Unsigned> {
        Saturating(self.0.cast_unsigned())
    }

    /// Returns the number of bits required to represent the value.
    ///
    /// The number of bits required includes an initial one for
    /// negative numbers, and an initial zero for non-negative
    /// numbers.
    ///
    /// See also <code>FixedI32::[signed\_bits][FixedI32::signed_bits]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I4F4;
    /// use fixed::Saturating;
    /// assert_eq!(Saturating(I4F4::from_num(-3)).signed_bits(), 7);      // “_101.0000”
    /// assert_eq!(Saturating(I4F4::from_num(-1)).signed_bits(), 5);      // “___1.0000”
    /// assert_eq!(Saturating(I4F4::from_num(-0.0625)).signed_bits(), 1); // “____.___1”
    /// assert_eq!(Saturating(I4F4::from_num(0)).signed_bits(), 1);       // “____.___0”
    /// assert_eq!(Saturating(I4F4::from_num(0.0625)).signed_bits(), 2);  // “____.__01”
    /// assert_eq!(Saturating(I4F4::from_num(1)).signed_bits(), 6);       // “__01.0000”
    /// assert_eq!(Saturating(I4F4::from_num(3)).signed_bits(), 7);       // “_011.0000”
    /// ```
    #[inline]
    pub fn signed_bits(self) -> u32 {
        self.0.signed_bits()
    }

    /// Returns [`true`] if the number is >&nbsp;0.
    ///
    /// See also <code>FixedI32::[is\_positive][FixedI32::is_positive]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// assert!(Saturating(I16F16::from_num(4.3)).is_positive());
    /// assert!(!Saturating(I16F16::ZERO).is_positive());
    /// assert!(!Saturating(I16F16::from_num(-4.3)).is_positive());
    /// ```
    #[inline]
    pub fn is_positive(self) -> bool {
        self.0.is_positive()
    }

    /// Returns [`true`] if the number is <&nbsp;0.
    ///
    /// See also <code>FixedI32::[is\_negative][FixedI32::is_negative]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// assert!(!Saturating(I16F16::from_num(4.3)).is_negative());
    /// assert!(!Saturating(I16F16::ZERO).is_negative());
    /// assert!(Saturating(I16F16::from_num(-4.3)).is_negative());
    /// ```
    #[inline]
    pub fn is_negative(self) -> bool {
        self.0.is_negative()
    }

    /// Saturating absolute value. Returns the absolute value, saturating
    /// on overflow.
    ///
    /// Overflow can only occur when trying to find the absolute value
    /// of the minimum value.
    ///
    /// See also <code>FixedI32::[saturating\_abs][FixedI32::saturating_abs]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::I16F16;
    /// use fixed::Saturating;
    /// assert_eq!(Saturating(I16F16::from_num(-5)).abs(), Saturating(I16F16::from_num(5)));
    /// assert_eq!(Saturating(I16F16::MIN).abs(), Saturating(I16F16::MAX));
    /// ```
    #[inline]
    #[must_use]
    pub fn abs(self) -> Saturating<F> {
        Saturating(self.0.saturating_abs())
    }

    /// Returns a number representing the sign of `self`.
    ///
    /// # Warning
    ///
    /// Using this method when 1 and &minus;1 cannot be represented is
    /// almost certainly a bug, however, this is allowed and gives the
    /// following saturated results.
    ///
    ///   * When there are no integer bits, for example for the type
    ///     <code>[Saturating]&lt;[I0F16]&gt;</code>, the return value is zero
    ///     when `self` is zero, [`MIN`] when `self` is negative, and [`MAX`]
    ///     when `self` is positive.
    ///   * When there is one integer bit, for example for the type
    ///     <code>[Saturating]&lt;[I1F15]&gt;</code>, the return value is zero
    ///     when `self` is zero, &minus;1 when `self` is negative, and [`MAX`]
    ///     when `self` is positive.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_signum][FixedI32::saturating_signum]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::{I0F32, I1F31, I16F16};
    /// use fixed::Saturating;
    /// assert_eq!(Saturating(<I16F16>::from_num(-3.9)).signum(), Saturating(I16F16::NEG_ONE));
    /// assert_eq!(Saturating(<I16F16>::ZERO).signum(), Saturating(I16F16::ZERO));
    /// assert_eq!(Saturating(<I16F16>::from_num(3.9)).signum(), Saturating(I16F16::ONE));
    ///
    /// assert_eq!(Saturating(<I1F31>::from_num(0.5)).signum(), Saturating(I1F31::MAX));
    /// assert_eq!(Saturating(<I0F32>::from_num(0.25)).signum(), Saturating(I0F32::MAX));
    /// assert_eq!(Saturating(<I0F32>::from_num(-0.5)).signum(), Saturating(I0F32::MIN));
    /// ```
    ///
    /// [I0F16]: crate::types::I0F16
    /// [I1F15]: crate::types::I1F15
    /// [`MAX`]: Self::MAX
    /// [`MIN`]: Self::MIN
    #[inline]
    #[must_use]
    pub fn signum(self) -> Saturating<F> {
        Saturating(self.0.saturating_signum())
    }

    /// Addition with an unsigned fixed-point number.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_add\_unsigned][FixedI32::saturating_add_unsigned]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::{I16F16, U16F16};
    /// use fixed::Saturating;
    /// assert_eq!(
    ///     Saturating::<I16F16>::from_num(-5).add_unsigned(U16F16::from_num(3)),
    ///     Saturating::<I16F16>::from_num(-2)
    /// );
    /// assert_eq!(
    ///     Saturating::<I16F16>::ZERO.add_unsigned(U16F16::MAX),
    ///     Saturating::<I16F16>::MAX
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn add_unsigned(self, rhs: F::Unsigned) -> Saturating<F> {
        Saturating(self.0.saturating_add_unsigned(rhs))
    }

    /// Subtraction with an unsigned fixed-point number.
    ///
    /// See also
    /// <code>FixedI32::[saturating\_sub\_unsigned][FixedI32::saturating_sub_unsigned]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::{I16F16, U16F16};
    /// use fixed::Saturating;
    /// assert_eq!(
    ///     Saturating::<I16F16>::from_num(3).sub_unsigned(U16F16::from_num(5)),
    ///     Saturating::<I16F16>::from_num(-2)
    /// );
    /// assert_eq!(
    ///     Saturating::<I16F16>::ZERO.sub_unsigned(U16F16::MAX),
    ///     Saturating::<I16F16>::MIN
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn sub_unsigned(self, rhs: F::Unsigned) -> Saturating<F> {
        Saturating(self.0.saturating_sub_unsigned(rhs))
    }
}

impl<F: FixedUnsigned> Saturating<F> {
    /// Returns the bit pattern of `self` reinterpreted as a signed fixed-point
    /// number of the same size.
    ///
    /// See also <code>FixedU32::[cast\_signed][FixedU32::cast_signed]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::{I16F16, U16F16};
    /// use fixed::Saturating;
    ///
    /// let n = Saturating(U16F16::MAX);
    /// assert_eq!(n.cast_signed(), Saturating(-I16F16::DELTA));
    /// ```
    #[must_use]
    #[inline]
    pub fn cast_signed(self) -> Saturating<F::Signed> {
        Saturating(self.0.cast_signed())
    }

    /// Returns the number of bits required to represent the value.
    ///
    /// See also
    /// <code>FixedU32::[significant\_bits][FixedU32::significant_bits]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::U4F4;
    /// use fixed::Saturating;
    /// assert_eq!(Saturating(U4F4::from_num(0)).significant_bits(), 0);      // “____.____”
    /// assert_eq!(Saturating(U4F4::from_num(0.0625)).significant_bits(), 1); // “____.___1”
    /// assert_eq!(Saturating(U4F4::from_num(1)).significant_bits(), 5);      // “___1.0000”
    /// assert_eq!(Saturating(U4F4::from_num(3)).significant_bits(), 6);      // “__11.0000”
    /// ```
    #[inline]
    pub fn significant_bits(self) -> u32 {
        self.0.significant_bits()
    }

    /// Returns [`true`] if the fixed-point number is
    /// 2<sup><i>k</i></sup> for some integer <i>k</i>.
    ///
    /// See also
    /// <code>FixedU32::[is\_power\_of\_two][FixedU32::is_power_of_two]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::U16F16;
    /// use fixed::Saturating;
    /// assert!(Saturating(U16F16::from_num(0.5)).is_power_of_two());
    /// assert!(Saturating(U16F16::from_num(4)).is_power_of_two());
    /// assert!(!Saturating(U16F16::from_num(5)).is_power_of_two());
    /// ```
    #[inline]
    pub fn is_power_of_two(self) -> bool {
        self.0.is_power_of_two()
    }

    /// Returns the highest one in the binary representation, or zero
    /// if `self` is zero.
    ///
    /// If `self`&nbsp;>&nbsp;0, the highest one is equal to the largest power
    /// of two that is ≤&nbsp;`self`.
    ///
    /// See also <code>FixedU32::[highest\_one][FixedU32::highest_one]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::U16F16;
    /// use fixed::Saturating;
    /// type Sa = Saturating<U16F16>;
    /// assert_eq!(Sa::from_bits(0b11_0010).highest_one(), Sa::from_bits(0b10_0000));
    /// assert_eq!(Sa::from_num(0.3).highest_one(), Sa::from_num(0.25));
    /// assert_eq!(Sa::from_num(4).highest_one(), Sa::from_num(4));
    /// assert_eq!(Sa::from_num(6.5).highest_one(), Sa::from_num(4));
    /// assert_eq!(Sa::ZERO.highest_one(), Sa::ZERO);
    /// ```
    #[inline]
    #[must_use]
    pub fn highest_one(self) -> Saturating<F> {
        Saturating(self.0.highest_one())
    }

    /// Addition with an signed fixed-point number.
    ///
    /// See also
    /// <code>FixedU32::[saturating\_add\_signed][FixedU32::saturating_add_signed]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::{I16F16, U16F16};
    /// use fixed::Saturating;
    /// assert_eq!(
    ///     Saturating::<U16F16>::from_num(5).add_signed(I16F16::from_num(-3)),
    ///     Saturating::<U16F16>::from_num(2)
    /// );
    /// assert_eq!(
    ///     Saturating::<U16F16>::ZERO.add_signed(-I16F16::DELTA),
    ///     Saturating::<U16F16>::ZERO
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn add_signed(self, rhs: F::Signed) -> Saturating<F> {
        Saturating(self.0.saturating_add_signed(rhs))
    }

    /// Subtraction with an signed fixed-point number.
    ///
    /// See also
    /// <code>FixedU32::[saturating\_sub\_signed][FixedU32::saturating_sub_signed]</code>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::{I16F16, U16F16};
    /// use fixed::Saturating;
    /// assert_eq!(
    ///     Saturating::<U16F16>::from_num(5).sub_signed(I16F16::from_num(-3)),
    ///     Saturating::<U16F16>::from_num(8)
    /// );
    /// assert_eq!(
    ///     Saturating::<U16F16>::ZERO.sub_signed(I16F16::DELTA),
    ///     Saturating::<U16F16>::ZERO
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn sub_signed(self, rhs: F::Signed) -> Saturating<F> {
        Saturating(self.0.saturating_sub_signed(rhs))
    }
}

impl<F: Fixed> Display for Saturating<F> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl<F: Fixed> Debug for Saturating<F> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Debug::fmt(&self.0, f)
    }
}

impl<F: Fixed> Binary for Saturating<F> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Binary::fmt(&self.0, f)
    }
}

impl<F: Fixed> Octal for Saturating<F> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Octal::fmt(&self.0, f)
    }
}

impl<F: Fixed> LowerHex for Saturating<F> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        LowerHex::fmt(&self.0, f)
    }
}

impl<F: Fixed> UpperHex for Saturating<F> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        UpperHex::fmt(&self.0, f)
    }
}

impl<F: Fixed> LowerExp for Saturating<F> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        LowerExp::fmt(&self.0, f)
    }
}

impl<F: Fixed> UpperExp for Saturating<F> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        UpperExp::fmt(&self.0, f)
    }
}

impl<F: Fixed> From<F> for Saturating<F> {
    /// Saturates a fixed-point number.
    #[inline]
    fn from(src: F) -> Saturating<F> {
        Saturating(src)
    }
}

impl<F: Fixed> FromStr for Saturating<F> {
    type Err = ParseFixedError;
    /// Parses a string slice containing decimal digits to return a fixed-point number.
    ///
    /// Rounding is to the nearest, with ties rounded to even.
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        F::saturating_from_str(s).map(Saturating)
    }
}

macro_rules! op {
    ($saturating:ident, $Op:ident $op:ident, $OpAssign:ident $op_assign:ident) => {
        impl<F: Fixed> $Op<Saturating<F>> for Saturating<F> {
            type Output = Saturating<F>;
            #[inline]
            fn $op(self, other: Saturating<F>) -> Saturating<F> {
                Saturating((self.0).$saturating(other.0))
            }
        }
        impl<F: Fixed> $Op<Saturating<F>> for &Saturating<F> {
            type Output = Saturating<F>;
            #[inline]
            fn $op(self, other: Saturating<F>) -> Saturating<F> {
                Saturating((self.0).$saturating(other.0))
            }
        }
        impl<F: Fixed> $Op<&Saturating<F>> for Saturating<F> {
            type Output = Saturating<F>;
            #[inline]
            fn $op(self, other: &Saturating<F>) -> Saturating<F> {
                Saturating((self.0).$saturating(other.0))
            }
        }
        impl<F: Fixed> $Op<&Saturating<F>> for &Saturating<F> {
            type Output = Saturating<F>;
            #[inline]
            fn $op(self, other: &Saturating<F>) -> Saturating<F> {
                Saturating((self.0).$saturating(other.0))
            }
        }
        impl<F: Fixed> $OpAssign<Saturating<F>> for Saturating<F> {
            #[inline]
            fn $op_assign(&mut self, other: Saturating<F>) {
                self.0 = (self.0).$saturating(other.0);
            }
        }
        impl<F: Fixed> $OpAssign<&Saturating<F>> for Saturating<F> {
            #[inline]
            fn $op_assign(&mut self, other: &Saturating<F>) {
                self.0 = (self.0).$saturating(other.0);
            }
        }
        impl<F: Fixed> $OpAssign<F> for Saturating<F> {
            #[inline]
            fn $op_assign(&mut self, other: F) {
                self.0 = (self.0).$saturating(other);
            }
        }
        impl<F: Fixed> $OpAssign<&F> for Saturating<F> {
            #[inline]
            fn $op_assign(&mut self, other: &F) {
                self.0 = (self.0).$saturating(*other);
            }
        }
    };
}

macro_rules! op_bitwise {
    ($Op:ident $op:ident, $OpAssign:ident $op_assign:ident) => {
        impl<F> $Op<Saturating<F>> for Saturating<F>
        where
            F: $Op<F, Output = F>,
        {
            type Output = Saturating<F>;
            #[inline]
            fn $op(self, other: Saturating<F>) -> Saturating<F> {
                Saturating((self.0).$op(other.0))
            }
        }
        impl<F> $Op<Saturating<F>> for &Saturating<F>
        where
            for<'a> &'a F: $Op<F, Output = F>,
        {
            type Output = Saturating<F>;
            #[inline]
            fn $op(self, other: Saturating<F>) -> Saturating<F> {
                Saturating((self.0).$op(other.0))
            }
        }
        impl<F> $Op<&Saturating<F>> for Saturating<F>
        where
            for<'a> F: $Op<&'a F, Output = F>,
        {
            type Output = Saturating<F>;
            #[inline]
            fn $op(self, other: &Saturating<F>) -> Saturating<F> {
                Saturating((self.0).$op(&other.0))
            }
        }
        impl<F> $Op<&Saturating<F>> for &Saturating<F>
        where
            for<'a, 'b> &'a F: $Op<&'b F, Output = F>,
        {
            type Output = Saturating<F>;
            #[inline]
            fn $op(self, other: &Saturating<F>) -> Saturating<F> {
                Saturating((self.0).$op(&other.0))
            }
        }
        impl<F> $OpAssign<Saturating<F>> for Saturating<F>
        where
            F: $OpAssign<F>,
        {
            #[inline]
            fn $op_assign(&mut self, other: Saturating<F>) {
                (self.0).$op_assign(other.0);
            }
        }
        impl<F> $OpAssign<&Saturating<F>> for Saturating<F>
        where
            for<'a> F: $OpAssign<&'a F>,
        {
            #[inline]
            fn $op_assign(&mut self, other: &Saturating<F>) {
                (self.0).$op_assign(&other.0);
            }
        }
        impl<F> $OpAssign<F> for Saturating<F>
        where
            F: $OpAssign<F>,
        {
            #[inline]
            fn $op_assign(&mut self, other: F) {
                (self.0).$op_assign(other);
            }
        }
        impl<F> $OpAssign<&F> for Saturating<F>
        where
            for<'a> F: $OpAssign<&'a F>,
        {
            #[inline]
            fn $op_assign(&mut self, other: &F) {
                (self.0).$op_assign(other);
            }
        }
    };
}

impl<F: Fixed> Neg for Saturating<F> {
    type Output = Saturating<F>;
    #[inline]
    fn neg(self) -> Saturating<F> {
        Saturating((self.0).saturating_neg())
    }
}

impl<F: Fixed> Neg for &Saturating<F> {
    type Output = Saturating<F>;
    #[inline]
    fn neg(self) -> Saturating<F> {
        Saturating((self.0).saturating_neg())
    }
}
op! { saturating_add, Add add, AddAssign add_assign }
op! { saturating_sub, Sub sub, SubAssign sub_assign }
op! { saturating_mul, Mul mul, MulAssign mul_assign }
op! { saturating_div, Div div, DivAssign div_assign }
op! { rem, Rem rem, RemAssign rem_assign }

impl<F> Not for Saturating<F>
where
    F: Not<Output = F>,
{
    type Output = Saturating<F>;
    #[inline]
    fn not(self) -> Saturating<F> {
        Saturating((self.0).not())
    }
}
impl<F> Not for &Saturating<F>
where
    for<'a> &'a F: Not<Output = F>,
{
    type Output = Saturating<F>;
    #[inline]
    fn not(self) -> Saturating<F> {
        Saturating((self.0).not())
    }
}
op_bitwise! { BitAnd bitand, BitAndAssign bitand_assign }
op_bitwise! { BitOr bitor, BitOrAssign bitor_assign }
op_bitwise! { BitXor bitxor, BitXorAssign bitxor_assign }

impl<F: Fixed> Sum<Saturating<F>> for Saturating<F> {
    fn sum<I>(iter: I) -> Saturating<F>
    where
        I: Iterator<Item = Saturating<F>>,
    {
        iter.fold(Saturating(F::ZERO), Add::add)
    }
}

impl<'a, F: 'a + Fixed> Sum<&'a Saturating<F>> for Saturating<F> {
    fn sum<I>(iter: I) -> Saturating<F>
    where
        I: Iterator<Item = &'a Saturating<F>>,
    {
        iter.fold(Saturating(F::ZERO), Add::add)
    }
}

impl<F: Fixed> Product<Saturating<F>> for Saturating<F> {
    fn product<I>(mut iter: I) -> Saturating<F>
    where
        I: Iterator<Item = Saturating<F>>,
    {
        match iter.next() {
            None => Saturating(1.saturating_to_fixed()),
            Some(first) => iter.fold(first, Mul::mul),
        }
    }
}

impl<'a, F: 'a + Fixed> Product<&'a Saturating<F>> for Saturating<F> {
    fn product<I>(mut iter: I) -> Saturating<F>
    where
        I: Iterator<Item = &'a Saturating<F>>,
    {
        match iter.next() {
            None => Saturating(1.saturating_to_fixed()),
            Some(first) => iter.fold(*first, Mul::mul),
        }
    }
}

// The following cannot be implemented for Saturating<F> where F: Fixed,
// otherwise there will be a conflicting implementation error. For
// example we cannot implement both these without triggering E0119:
//
//     impl<F: Fixed> Op<F::Bits> for Saturating<F> { /* ... */ }
//     impl<F: Fixed> Op<&F::Bits> for Saturating<F> { /* ... */ }
//
// To work around this, we provide implementations like this:
//
//     impl<Frac> Op<i8> for Saturating<FixedI8<Frac>> { /* ... */ }
//     impl<Frac> Op<&i8> for Saturating<FixedI8<Frac>> { /* ... */ }
//     impl<Frac> Op<i16> for Saturating<FixedI16<Frac>> { /* ... */ }
//     impl<Frac> Op<&i16> for Saturating<FixedI16<Frac>> { /* ... */ }
//     ...

macro_rules! op_bits {
    (
        $Fixed:ident($Bits:ident $(, $LeEqU:ident)*)::$saturating:ident,
        $Op:ident $op:ident,
        $OpAssign:ident $op_assign:ident
    ) => {
        impl<Frac $(: $LeEqU)*> $Op<$Bits> for Saturating<$Fixed<Frac>> {
            type Output = Saturating<$Fixed<Frac>>;
            #[inline]
            fn $op(self, other: $Bits) -> Saturating<$Fixed<Frac>> {
                Saturating((self.0).$saturating(other))
            }
        }
        impl<Frac $(: $LeEqU)*> $Op<$Bits> for &Saturating<$Fixed<Frac>> {
            type Output = Saturating<$Fixed<Frac>>;
            #[inline]
            fn $op(self, other: $Bits) -> Saturating<$Fixed<Frac>> {
                Saturating((self.0).$saturating(other))
            }
        }
        impl<Frac $(: $LeEqU)*> $Op<&$Bits> for Saturating<$Fixed<Frac>> {
            type Output = Saturating<$Fixed<Frac>>;
            #[inline]
            fn $op(self, other: &$Bits) -> Saturating<$Fixed<Frac>> {
                Saturating((self.0).$saturating(*other))
            }
        }
        impl<Frac $(: $LeEqU)*> $Op<&$Bits> for &Saturating<$Fixed<Frac>> {
            type Output = Saturating<$Fixed<Frac>>;
            #[inline]
            fn $op(self, other: &$Bits) -> Saturating<$Fixed<Frac>> {
                Saturating((self.0).$saturating(*other))
            }
        }
        impl<Frac $(: $LeEqU)*> $OpAssign<$Bits> for Saturating<$Fixed<Frac>> {
            #[inline]
            fn $op_assign(&mut self, other: $Bits) {
                self.0 = (self.0).$saturating(other);
            }
        }
        impl<Frac $(: $LeEqU)*> $OpAssign<&$Bits> for Saturating<$Fixed<Frac>> {
            #[inline]
            fn $op_assign(&mut self, other: &$Bits) {
                self.0 = (self.0).$saturating(*other);
            }
        }
    };
}

macro_rules! ops {
    ($Fixed:ident($Bits:ident, $LeEqU:ident)) => {
        op_bits! { $Fixed($Bits)::saturating_mul_int, Mul mul, MulAssign mul_assign }
        op_bits! { $Fixed($Bits)::saturating_div_int, Div div, DivAssign div_assign }
        op_bits! { $Fixed($Bits, $LeEqU)::rem, Rem rem, RemAssign rem_assign }
    };
}
ops! { FixedI8(i8, LeEqU8) }
ops! { FixedI16(i16, LeEqU16) }
ops! { FixedI32(i32, LeEqU32) }
ops! { FixedI64(i64, LeEqU64) }
ops! { FixedI128(i128, LeEqU128) }
ops! { FixedU8(u8, LeEqU8) }
ops! { FixedU16(u16, LeEqU16) }
ops! { FixedU32(u32, LeEqU32) }
ops! { FixedU64(u64, LeEqU64) }
ops! { FixedU128(u128, LeEqU128) }
