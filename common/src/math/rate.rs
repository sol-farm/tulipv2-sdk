// Copyright 2020 Solana Foundation.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License;
// You may obtain the license via the LICENSE file in the repository root;
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Math for preserving precision of ratios and percentages.
//!
//! Usages and their ranges include:
//!   - Collateral exchange ratio <= 5.0
//!   - Loan to value ratio <= 0.9
//!   - Max borrow rate <= 2.56
//!   - Percentages <= 1.0
//!
//! Rates are internally scaled by a WAD (10^18) to preserve
//! precision up to 18 decimal places. Rates are sized to support
//! both serialization and precise math for the full range of
//! unsigned 8-bit integers. The underlying representation is a
//! u128 rather than u192 to reduce compute cost while losing
//! support for arithmetic operations at the high end of u8 range.

#![allow(clippy::assign_op_pattern)]
#![allow(clippy::ptr_offset_with_cast)]
#![allow(clippy::reversed_empty_ranges)]
#![allow(clippy::manual_range_contains)]

use super::common::*;
use super::decimal::Decimal;
use super::error::*;

use anchor_lang::solana_program::program_error::ProgramError;
use std::{convert::TryFrom, fmt};
use uint::construct_uint;

// U128 with 128 bits consisting of 2 x 64-bit words
construct_uint! {
    pub struct U128(2);
}

/// Small decimal values, precise to 18 digits
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Rate(pub U128);

impl Rate {
    /// One
    pub fn one() -> Self {
        Self(Self::wad())
    }

    /// Zero
    pub fn zero() -> Self {
        Self(U128::from(0))
    }

    fn wad() -> U128 {
        const RWAD: U128 = U128([1_000_000_000_000_000_000, 0]);
        RWAD
    }

    fn half_wad() -> U128 {
        const HALF_RWAD: U128 = U128([500_000_000_000_000_000, 0]);
        HALF_RWAD
    }

    /// Create scaled decimal from percent value
    pub fn from_percent(percent: u8) -> Self {
        Self(U128::from(percent as u64 * PERCENT_SCALER))
    }

    /// Return raw scaled value
    #[allow(clippy::wrong_self_convention)]
    pub fn to_scaled_val(&self) -> u128 {
        self.0.as_u128()
    }

    /// Create decimal from scaled value
    pub fn from_scaled_val(scaled_val: u64) -> Self {
        Self(U128::from(scaled_val))
    }

    /// Create decimal from double scaled???
    pub fn from_scaled_val_big(scaled_val_big: u128) -> Self {
        Self(U128::from(scaled_val_big))
    }

    /// Round scaled decimal to u64
    pub fn try_round_u64(&self) -> Result<u64, ProgramError> {
        let rounded_val = Self::half_wad()
            .checked_add(self.0)
            .ok_or(MathError::MathOverflow)?
            .checked_div(Self::wad())
            .ok_or(MathError::MathOverflow)?;
        Ok(u64::try_from(rounded_val).map_err(|_| MathError::MathOverflow)?)
    }
    /// Floor scaled decimal to u64
    pub fn try_floor_u64(&self) -> Result<u64, ProgramError> {
        let ceil_val = self
            .0
            .checked_div(Self::wad())
            .ok_or(MathError::MathOverflow)?;
        Ok(u64::try_from(ceil_val).map_err(|_| MathError::MathOverflow)?)
    }
    /// Calculates base^exp
    pub fn try_pow(&self, mut exp: u64) -> Result<Rate, ProgramError> {
        let mut base = *self;
        let mut ret = if exp % 2 != 0 {
            base
        } else {
            Rate(Self::wad())
        };

        while exp > 0 {
            exp /= 2;
            base = base.try_mul(base)?;

            if exp % 2 != 0 {
                ret = ret.try_mul(base)?;
            }
        }

        Ok(ret)
    }
}

impl fmt::Display for Rate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut scaled_val = self.0.to_string();
        if scaled_val.len() <= SCALE {
            scaled_val.insert_str(0, &vec!["0"; SCALE - scaled_val.len()].join(""));
            scaled_val.insert_str(0, "0.");
        } else {
            scaled_val.insert(scaled_val.len() - SCALE, '.');
        }
        f.write_str(&scaled_val)
    }
}

impl TryFrom<Decimal> for Rate {
    type Error = ProgramError;
    fn try_from(decimal: Decimal) -> Result<Self, Self::Error> {
        Ok(Self(U128::from(decimal.to_scaled_val()?)))
    }
}

impl TryAdd for Rate {
    fn try_add(self, rhs: Self) -> Result<Self, ProgramError> {
        Ok(Self(
            self.0.checked_add(rhs.0).ok_or(MathError::MathOverflow)?,
        ))
    }
}

impl TrySub for Rate {
    fn try_sub(self, rhs: Self) -> Result<Self, ProgramError> {
        Ok(Self(
            self.0.checked_sub(rhs.0).ok_or(MathError::MathOverflow)?,
        ))
    }
}

impl TryDiv<u64> for Rate {
    fn try_div(self, rhs: u64) -> Result<Self, ProgramError> {
        Ok(Self(
            self.0
                .checked_div(U128::from(rhs))
                .ok_or(MathError::MathOverflow)?,
        ))
    }
}

impl TryDiv<Rate> for Rate {
    fn try_div(self, rhs: Self) -> Result<Self, ProgramError> {
        Ok(Self(
            self.0
                .checked_mul(Self::wad())
                .ok_or(MathError::MathOverflow)?
                .checked_div(rhs.0)
                .ok_or(MathError::MathOverflow)?,
        ))
    }
}

impl TryMul<u64> for Rate {
    fn try_mul(self, rhs: u64) -> Result<Self, ProgramError> {
        Ok(Self(
            self.0
                .checked_mul(U128::from(rhs))
                .ok_or(MathError::MathOverflow)?,
        ))
    }
}

impl TryMul<Rate> for Rate {
    fn try_mul(self, rhs: Self) -> Result<Self, ProgramError> {
        Ok(Self(
            self.0
                .checked_mul(rhs.0)
                .ok_or(MathError::MathOverflow)?
                .checked_div(Self::wad())
                .ok_or(MathError::MathOverflow)?,
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn checked_pow() {
        assert_eq!(Rate::one(), Rate::one().try_pow(u64::MAX).unwrap());
    }

    #[test]
    fn rate() {
        assert_eq!(Rate::one(), Rate(Rate::wad()));
        assert_eq!(Rate::one().to_string(), "1.000000000000000000".to_string());
        assert_eq!(Rate::half_wad(), U128::from(HALF_WAD));
        assert_eq!(
            Rate::half_wad().to_string(),
            "500000000000000000".to_string()
        );
        assert_eq!(
            Rate::from_percent(69).to_string(),
            "0.690000000000000000".to_string()
        );
        assert_eq!(
            Rate::from_percent(69).to_scaled_val().to_string(),
            "690000000000000000".to_string()
        );
        assert_eq!(
            Rate::from_scaled_val(Rate::from_percent(69).to_scaled_val() as u64).to_string(),
            "0.690000000000000000".to_string()
        );
        assert_eq!(
            Rate::from_percent(69).try_round_u64().unwrap().to_string(),
            "1".to_string()
        );
        assert_eq!(
            Rate::from_percent(212).try_round_u64().unwrap().to_string(),
            "2".to_string()
        );
        assert_eq!(
            Rate::from_percent(253).try_round_u64().unwrap().to_string(),
            "3".to_string()
        );
        println!("{}", Rate::one());

        let a = Rate::from_scaled_val(100_u64);
        let b = Rate::from_scaled_val(200_u64);

        let c = a.try_add(b).unwrap();
        assert_eq!(c, Rate::from_scaled_val(300_u64));

        let c = a.try_sub(b);
        assert_eq!(c.is_err(), true);

        let c = b.try_sub(a).unwrap();
        assert_eq!(c, Rate::from_scaled_val(100_u64));

        let c = b.try_div(2_u64).unwrap();
        assert_eq!(c, Rate::from_scaled_val(100_u64));

        let c = b.try_div(b).unwrap();
        assert_eq!(c, Rate::from_scaled_val(1000000000000000000));

        assert_eq!(b.try_div(0_u64).is_err(), true);
        assert_eq!(b.try_div(Rate::zero()).is_err(), true);

        let c = b.try_mul(2_u64).unwrap();
        assert_eq!(c, Rate::from_scaled_val(400_u64));

        let c = b.try_mul(Rate::from_percent(50)).unwrap();
        assert_eq!(c, Rate::from_scaled_val(100_u64));
    }

    #[test]
    fn overflows() {
        let a = Rate(U128::MAX);
        let zero = Rate::zero();
        assert_eq!(a.try_add(a).is_err(), true);
        assert_eq!(zero.try_sub(a).is_err(), true);
        assert_eq!(a.try_mul(a).is_err(), true);
        assert_eq!(a.try_div(Rate::from_percent(1)).is_err(), true);
        assert_eq!(a.try_mul(u64::MAX).is_err(), true);
        assert_eq!(a.try_div(0_u64).is_err(), true);
    }
    #[test]
    fn test_constant_funcs() {
        println!("wad {:?}", U128::from(WAD).0);
        println!("half wad {:?}", U128::from(HALF_WAD).0);
        // ensure constants evaluate to non-constantversion
        assert!(U128::from(WAD) == Rate::wad());
        assert!(U128::from(HALF_WAD) == Rate::half_wad());
    }
}
