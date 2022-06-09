//! lending vault information
pub mod deposit;
pub mod register_deposit_tracking;
pub mod utils;
pub mod withdraw;
pub mod withdraw_deposit_tracking;

#[cfg(feature = "usdc-optimizer")]
pub mod usdc;

#[cfg(feature = "sol-optimizer")]
pub mod sol;

#[cfg(feature = "ray-optimizer")]
pub mod ray;

#[cfg(feature = "usdt-optimizer")]
pub mod usdt;