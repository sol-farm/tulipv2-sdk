//! This module provides the configuration information for all of the tulip lending optimizer vaults as public constant pubkeys in specific modules
//! that can easily be imported into any program without having to maintain a separate configuration file off-chain for this information.
//!
//! By default all of the configuration modules are  enabled, however this can be disabled/enabled using feature flags to help reduce the size of the dependency
use solana_program::pubkey::Pubkey;
pub mod traits;
pub mod withdraw;

#[cfg(feature = "usdc-optimizer")]
pub mod usdc;

#[cfg(feature = "sol-optimizer")]
pub mod sol;

#[cfg(feature = "ray-optimizer")]
pub mod ray;

#[cfg(feature = "usdt-optimizer")]
pub mod usdt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Platform {
    MangoV3,
    Tulip,
    Solend,
}
