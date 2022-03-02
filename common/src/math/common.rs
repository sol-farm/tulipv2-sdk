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

//! Common module for Decimal and Rate

use anchor_lang::solana_program::program_error::ProgramError;

/// Scale of precision
pub const SCALE: usize = 18;
/// Identity
pub const WAD: u64 = 1_000_000_000_000_000_000;
/// Half of identity
pub const HALF_WAD: u64 = 500_000_000_000_000_000;
/// Scale for percentages
pub const PERCENT_SCALER: u64 = 10_000_000_000_000_000;

/// Try to subtract, return an error on underflow
pub trait TrySub: Sized {
    /// Subtract
    fn try_sub(self, rhs: Self) -> Result<Self, ProgramError>;
}

/// Try to subtract, return an error on overflow
pub trait TryAdd: Sized {
    /// Add
    fn try_add(self, rhs: Self) -> Result<Self, ProgramError>;
}

/// Try to divide, return an error on overflow or divide by zero
pub trait TryDiv<RHS>: Sized {
    /// Divide
    fn try_div(self, rhs: RHS) -> Result<Self, ProgramError>;
}

/// Try to multiply, return an error on overflow
pub trait TryMul<RHS>: Sized {
    /// Multiply
    fn try_mul(self, rhs: RHS) -> Result<Self, ProgramError>;
}
