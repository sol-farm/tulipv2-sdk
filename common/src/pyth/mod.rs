//! provides support for reading pyth price feeds from tulip's pyth price manager program
//! this should in theory work with any pyth v2 price account out there, however it is only tested
//! with the accounts that tulip publishes prices for via our own price feed program.

use crate::math::{common::TryDiv, decimal::Decimal};
use anchor_lang::prelude::ProgramError;
use bytemuck::{cast_slice, from_bytes, try_cast_slice, Pod, PodCastError, Zeroable};
use solana_sdk::account::Account;
use std::mem::size_of;

/// after this many slots consider a price update as being stale and thus invalid
// 30 slots translates to a period of around 15s depending on slot times
pub const STALE_AFTER_SLOTS_ELAPSED: u64 = 120;

pub const MAGIC: u32 = 0xa1b2c3d4;
pub const VERSION_2: u32 = 2;
pub const VERSION: u32 = VERSION_2;
pub const MAP_TABLE_SIZE: usize = 640;
pub const PROD_ACCT_SIZE: usize = 512;
pub const PROD_HDR_SIZE: usize = 48;
pub const PROD_ATTR_SIZE: usize = PROD_ACCT_SIZE - PROD_HDR_SIZE;

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct AccKey {
    pub val: [u8; 32],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub enum AccountType {
    Unknown,
    Mapping,
    Product,
    Price,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub enum PriceStatus {
    Unknown,
    Trading,
    Halted,
    Auction,
}

impl std::fmt::Display for PriceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name().as_str())
    }
}

impl PriceStatus {
    pub fn name(&self) -> String {
        match self {
            PriceStatus::Unknown => String::from("unknown"),
            PriceStatus::Trading => String::from("trading"),
            PriceStatus::Halted => String::from("halted"),
            PriceStatus::Auction => String::from("auction"),
        }
    }
}

impl Default for PriceStatus {
    fn default() -> Self {
        PriceStatus::Trading
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub enum CorpAction {
    NoCorpAct,
}

impl Default for CorpAction {
    fn default() -> Self {
        CorpAction::NoCorpAct
    }
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct PriceInfo {
    pub price: i64,
    pub conf: u64,
    pub status: PriceStatus,
    pub corp_act: CorpAction,
    pub pub_slot: u64,
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct PriceComp {
    publisher: AccKey,
    agg: PriceInfo,
    latest: PriceInfo,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub enum PriceType {
    Unknown,
    Price,
    #[allow(clippy::upper_case_acronyms)]
    TWAP,
    Volatility,
}

impl Default for PriceType {
    fn default() -> Self {
        PriceType::Price
    }
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct Price {
    pub magic: u32,       // Pyth magic number.
    pub ver: u32,         // Program version.
    pub atype: u32,       // Account type.
    pub size: u32,        // Price account size.
    pub ptype: PriceType, // Price or calculation type.
    pub expo: i32,        // Price exponent.
    pub num: u32,         // Number of component prices.
    pub unused: u32,
    pub curr_slot: u64,        // Currently accumulating price slot.
    pub valid_slot: u64,       // Valid slot-time of agg. price.
    pub twap: i64,             // Time-weighted average price.
    pub avol: u64,             // Annualized price volatility.
    pub drv0: i64,             // Space for future derived values.
    pub drv1: i64,             // Space for future derived values.
    pub drv2: i64,             // Space for future derived values.
    pub drv3: i64,             // Space for future derived values.
    pub drv4: i64,             // Space for future derived values.
    pub drv5: i64,             // Space for future derived values.
    pub prod: AccKey,          // Product account key.
    pub next: AccKey,          // Next Price account in linked list.
    pub agg_pub: AccKey,       // Quoter who computed last aggregate price.
    pub agg: PriceInfo,        // Aggregate price info.
    pub comp: [PriceComp; 32], // Price components one per quoter.
}

#[cfg(target_endian = "little")]
unsafe impl Zeroable for Price {}

#[cfg(target_endian = "little")]
unsafe impl Pod for Price {}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Product {
    pub magic: u32,                 // pyth magic number
    pub ver: u32,                   // program version
    pub atype: u32,                 // account type
    pub size: u32,                  // price account size
    pub px_acc: AccKey,             // first price account in list
    pub attr: [u8; PROD_ATTR_SIZE], // key/value pairs of reference attr.
}

#[cfg(target_endian = "little")]
unsafe impl Zeroable for Product {}

#[cfg(target_endian = "little")]
unsafe impl Pod for Product {}

pub fn load<T: Pod>(data: &[u8]) -> Result<&T, PodCastError> {
    let size = size_of::<T>();
    Ok(from_bytes(cast_slice::<u8, u8>(try_cast_slice(
        &data[0..size],
    )?)))
}

/// returns the price contained in a price feed account, without
/// validating price staleness
pub fn get_pyth_price(pyth_price_info: &Account) -> Result<Decimal, ProgramError> {
    let pyth_price =
        load::<Price>(&pyth_price_info.data).map_err(|_| ProgramError::InvalidAccountData)?;
    if pyth_price.ptype as u32 != PriceType::Price as u32 {
        return Err(ProgramError::Custom(u32::MAX - 1));
    }

    let price: u64 = match pyth_price.agg.price.checked_abs() {
        Some(val) => match val.try_into() {
            Ok(val) => val,
            Err(_) => return Err(ProgramError::InvalidArgument),
        },
        None => return Err(ProgramError::Custom(u32::MAX - 2)),
    };

    // @TODO: handle the case that the exponent is positive?
    let pyth_exponent = match pyth_price.expo.checked_abs() {
        Some(val) => match val.try_into() {
            Ok(val) => val,
            Err(_) => return Err(ProgramError::InvalidArgument),
        },
        None => return Err(ProgramError::Custom(u32::MAX - 2)),
    };

    let pyth_decimals = match 10u64.checked_pow(pyth_exponent) {
        Some(val) => val,
        None => return Err(ProgramError::Custom(u32::MAX - 2)),
    };
    let market_price = Decimal::from(price).try_div(pyth_decimals)?;
    Ok(market_price)
}

#[cfg(test)]
mod test {

    use super::*;
    use anchor_client::solana_client::rpc_client::RpcClient;
    use anchor_lang::solana_program;
    use static_pubkey::static_pubkey;
    #[test]
    fn test_get_pyth_price_account() {
        let rpc = RpcClient::new("https://ssc-dao.genesysgo.net".to_string());
        let tulip_price_account_key =
            static_pubkey!("5RHxy1NbUR15y34uktDbN1a2SWbhgHwkCZ75yK2RJ1FC");
        let tulip_price_account = rpc.get_account(&tulip_price_account_key).unwrap();
        let price = get_pyth_price(&tulip_price_account).unwrap();
        println!("price {:#?}", price);
    }
}
