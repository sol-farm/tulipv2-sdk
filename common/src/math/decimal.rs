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

//! Math for preserving precision of token amounts which are limited
//! by the SPL Token program to be at most u64::MAX.
//!
//! Decimals are internally scaled by a WAD (10^18) to preserve
//! precision up to 18 decimal places. Decimals are sized to support
//! both serialization and precise math for the full range of
//! unsigned 64-bit integers. The underlying representation is a
//! u192 rather than u256 to reduce compute cost while losing
//! support for arithmetic operations at the high end of u64 range.

#![allow(clippy::assign_op_pattern)]
#![allow(clippy::ptr_offset_with_cast)]
#![allow(clippy::manual_range_contains)]

use super::common::*;
use super::rate::Rate;

use super::error::MathError;
use super::uint::U192;

use anchor_lang::solana_program::program_error::ProgramError;
use std::{convert::TryFrom, fmt};

/// Large decimal values, precise to 18 digits
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Decimal(pub U192);

impl Decimal {
    /// One
    pub fn one() -> Self {
        Self(Self::wad())
    }

    /// Zero
    pub fn zero() -> Self {
        Self(U192::zero())
    }

    fn wad() -> U192 {
        const DWAD: U192 = U192([1_000_000_000_000_000_000, 0, 0]);
        DWAD
    }
    fn half_wad() -> U192 {
        const HALF_DWAD: U192 = U192([500_000_000_000_000_000, 0, 0]);
        HALF_DWAD
    }

    /// Create scaled decimal from percent value
    pub fn from_percent(percent: u8) -> Self {
        Self(U192::from(percent as u64 * PERCENT_SCALER))
    }

    /// Return raw scaled value if it fits within u128
    #[allow(clippy::wrong_self_convention)]
    pub fn to_scaled_val(&self) -> Result<u128, ProgramError> {
        Ok(u128::try_from(self.0).map_err(|_| MathError::MathOverflow)?)
    }

    /// Create decimal from scaled value
    pub fn from_scaled_val(scaled_val: u128) -> Self {
        Self(U192::from(scaled_val))
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

    /// Ceiling scaled decimal to u64
    pub fn try_ceil_u64(&self) -> Result<u64, ProgramError> {
        let ceil_val = Self::wad()
            .checked_sub(U192::from(1u64))
            .ok_or(MathError::MathOverflow)?
            .checked_add(self.0)
            .ok_or(MathError::MathOverflow)?
            .checked_div(Self::wad())
            .ok_or(MathError::MathOverflow)?;
        Ok(u64::try_from(ceil_val).map_err(|_| MathError::MathOverflow)?)
    }

    /// Floor scaled decimal to u64
    pub fn try_floor_u64(&self) -> Result<u64, ProgramError> {
        let ceil_val = self
            .0
            .checked_div(Self::wad())
            .ok_or(MathError::MathOverflow)?;
        Ok(u64::try_from(ceil_val).map_err(|_| MathError::MathOverflow)?)
    }
}

impl fmt::Display for Decimal {
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

impl From<u64> for Decimal {
    fn from(val: u64) -> Self {
        Self(Self::wad() * U192::from(val))
    }
}

impl From<u128> for Decimal {
    fn from(val: u128) -> Self {
        Self(Self::wad() * U192::from(val))
    }
}

impl From<Rate> for Decimal {
    fn from(val: Rate) -> Self {
        Self(U192::from(val.to_scaled_val()))
    }
}

impl TryAdd for Decimal {
    fn try_add(self, rhs: Self) -> Result<Self, ProgramError> {
        Ok(Self(
            self.0.checked_add(rhs.0).ok_or(MathError::MathOverflow)?,
        ))
    }
}

impl TrySub for Decimal {
    fn try_sub(self, rhs: Self) -> Result<Self, ProgramError> {
        Ok(Self(
            self.0.checked_sub(rhs.0).ok_or(MathError::MathOverflow)?,
        ))
    }
}

impl TryDiv<u64> for Decimal {
    fn try_div(self, rhs: u64) -> Result<Self, ProgramError> {
        Ok(Self(
            self.0
                .checked_div(U192::from(rhs))
                .ok_or(MathError::MathOverflow)?,
        ))
    }
}

impl TryDiv<Rate> for Decimal {
    fn try_div(self, rhs: Rate) -> Result<Self, ProgramError> {
        self.try_div(Self::from(rhs))
    }
}

impl TryDiv<Decimal> for Decimal {
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

impl TryMul<u64> for Decimal {
    fn try_mul(self, rhs: u64) -> Result<Self, ProgramError> {
        Ok(Self(
            self.0
                .checked_mul(U192::from(rhs))
                .ok_or(MathError::MathOverflow)?,
        ))
    }
}

impl TryMul<Rate> for Decimal {
    fn try_mul(self, rhs: Rate) -> Result<Self, ProgramError> {
        self.try_mul(Self::from(rhs))
    }
}

impl TryMul<Decimal> for Decimal {
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
    fn test_scaler() {
        assert_eq!(U192::exp10(SCALE), Decimal::wad());
    }

    #[test]
    fn decimal() {
        assert_eq!(Decimal::one(), Decimal(Decimal::wad()));
        assert_eq!(
            Decimal::one().to_string(),
            "1.000000000000000000".to_string()
        );
        assert_eq!(Decimal::half_wad(), U192::from(HALF_WAD));
        assert_eq!(
            Decimal::half_wad().to_string(),
            "500000000000000000".to_string()
        );
        assert_eq!(
            Decimal::from_percent(69).to_string(),
            "0.690000000000000000".to_string()
        );
        assert_eq!(
            Decimal::from_percent(69)
                .to_scaled_val()
                .unwrap()
                .to_string(),
            "690000000000000000".to_string()
        );
        assert_eq!(
            Decimal::from_scaled_val(Decimal::from_percent(69).to_scaled_val().unwrap())
                .to_string(),
            "0.690000000000000000".to_string()
        );
        assert_eq!(
            Decimal::from_percent(69)
                .try_round_u64()
                .unwrap()
                .to_string(),
            "1".to_string()
        );
        assert_eq!(
            Decimal::from_percent(212)
                .try_round_u64()
                .unwrap()
                .to_string(),
            "2".to_string()
        );
        assert_eq!(
            Decimal::from_percent(253)
                .try_round_u64()
                .unwrap()
                .to_string(),
            "3".to_string()
        );
        println!("{}", Decimal::one());

        let a = Decimal::from_scaled_val(100_u128);
        let b = Decimal::from_scaled_val(200_u128);

        let c = a.try_add(b).unwrap();
        assert_eq!(c, Decimal::from_scaled_val(300_u128));

        let c = a.try_sub(b);
        assert_eq!(c.is_err(), true);

        let c = b.try_sub(a).unwrap();
        assert_eq!(c, Decimal::from_scaled_val(100_u128));

        let c = b.try_div(2_u64).unwrap();
        assert_eq!(c, Decimal::from_scaled_val(100_u128));

        let c = b.try_div(b).unwrap();
        assert_eq!(c, Decimal::from_scaled_val(1000000000000000000));

        assert_eq!(b.try_div(0_u64).is_err(), true);
        assert_eq!(b.try_div(Decimal::zero()).is_err(), true);

        let c = b.try_mul(2_u64).unwrap();
        assert_eq!(c, Decimal::from_scaled_val(400_u128));

        let c = b.try_mul(Decimal::from_percent(50)).unwrap();
        assert_eq!(c, Decimal::from_scaled_val(100_u128));

        assert_eq!(
            Decimal::from_percent(69)
                .try_ceil_u64()
                .unwrap()
                .to_string(),
            "1".to_string()
        );
        assert_eq!(
            Decimal::from_percent(212)
                .try_ceil_u64()
                .unwrap()
                .to_string(),
            "3".to_string()
        );
        assert_eq!(
            Decimal::from_percent(253)
                .try_ceil_u64()
                .unwrap()
                .to_string(),
            "3".to_string()
        );

        assert_eq!(
            Decimal::from_percent(69)
                .try_floor_u64()
                .unwrap()
                .to_string(),
            "0".to_string()
        );
        assert_eq!(
            Decimal::from_percent(212)
                .try_floor_u64()
                .unwrap()
                .to_string(),
            "2".to_string()
        );
        assert_eq!(
            Decimal::from_percent(253)
                .try_floor_u64()
                .unwrap()
                .to_string(),
            "2".to_string()
        );
    }

    #[test]
    fn overflows() {
        let a = Decimal(U192::MAX);
        let zero = Decimal::zero();
        assert_eq!(a.try_add(a).is_err(), true);
        assert_eq!(zero.try_sub(a).is_err(), true);
        assert_eq!(a.try_mul(a).is_err(), true);
        assert_eq!(a.try_div(Decimal::from_percent(1)).is_err(), true);
        assert_eq!(a.try_div(Rate::from_percent(1)).is_err(), true);
        assert_eq!(a.try_mul(u64::MAX).is_err(), true);
        assert_eq!(a.try_div(0_u64).is_err(), true);
    }
    #[test]
    fn test_constant_funcs() {
        // ensure constants evaluate to non-constantversion
        assert!(U192::from(WAD) == Decimal::wad());
        assert!(U192::from(HALF_WAD) == Decimal::half_wad());
    }
}
