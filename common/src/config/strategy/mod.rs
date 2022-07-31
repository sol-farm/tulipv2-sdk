//! tulip strategy vault configurations

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
