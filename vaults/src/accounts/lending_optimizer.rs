//! lending optimizer vault which focuses on obtaining
//! the maximum interest rate possible for any supported asset
use super::{vault_base::VaultBaseV1, InitVaultArgsV1};
use tulipv2_sdk_farms::Farm;
use tulipv2_sdk_common::msg_panic;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_pack::Pack;
use arrform::{arrform, ArrForm};
use tulipv2_sdk_common::{
    traits::vault::TokenizedShares,
    DEFAULT_KEY,
};


use anchor_lang::prelude::*;

#[cfg(not(target_arch = "bpf"))]
use derivative::*;

#[cfg(not(target_arch = "bpf"))]
use type_layout::TypeLayout;

pub const OPTIMIZER_VAULT_ACCOUNT_SIZE: usize = 1648;
/// the offset at which the last_rebase_slot information starts
pub const OPTIMIZER_VAULT_LAST_REBASE_SLOT_OFFSET: u64 = 544;
pub const OPTIMIZER_PLATFORM_CONFIG_ACCOUNT_SIZE: usize = 376;
pub const OPTIMIZER_SPL_LENDING_CONFIG_DATA_ACCOUNT_SIZE: usize = 681;
pub const OPTIMIZER_MANGO_CONFIG_DATA_ACCOUNT_SIZE: usize = 489;

/// a vault type which focuses on obtain the highest
/// interest rate possible for lending any given asset
#[account(zero_copy)]
#[cfg_attr(not(target_arch = "bpf"), derive(Derivative))]
#[cfg_attr(not(target_arch = "bpf"), derive(TypeLayout))]
#[cfg_attr(not(target_arch = "bpf"), derivative(Debug))]
pub struct LendingOptimizerV1 {
    pub base: VaultBaseV1,
    /// the address of the program which is being farmed
    /// for example if we are currently farming Port, this would
    /// be the address of the port lending program
    pub current_farm_program: Pubkey,
    /// address of the platform information account
    pub current_platform_information: Pubkey,
    /// the current number of platforms that can be choosen from
    /// for rebalancing
    pub current_platform_count: u64,
    /// the slot at which a rebase last occured
    pub last_rebase_slot: u64,
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub buffer: [u8; 1000],
}

/// within a lending optimizer vault, this struct denotes
/// configuration informatin for a specific platform the optimizer
/// can deposit into.
#[account] // not zero copy so we dont care about padding
#[cfg_attr(not(target_arch = "bpf"), derive(Derivative))]
#[cfg_attr(not(target_arch = "bpf"), derivative(Debug))]
pub struct LendingPlatformV1 {
    /// the lending optimizer vault this platform account
    /// provides configuration for
    pub vault: Pubkey,
    /// address of the deployed lending program
    pub program_address: Pubkey,
    pub program_type: ProgramType,
    /// stores address information for the config
    /// data account which stores the actual configuration
    pub config_data_address: Pubkey,
    /// index of the account in the logical "pda map"
    pub index: u64,
    /// nonce used by the pda
    pub nonce: u8,
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub buffer: [u8; 256],
}

/// bundles configuration information for a specific
/// spl lending deployment. it can be used for both
/// unmodified and solend modified spl lending program deployments
#[account(zero_copy)]
#[cfg_attr(not(target_arch = "bpf"), derive(Derivative))]
#[cfg_attr(not(target_arch = "bpf"), derive(TypeLayout))]
#[cfg_attr(not(target_arch = "bpf"), derivative(Debug))]
pub struct SplLendingConfig {
    /// address of the collateral mint for the cTokens
    /// issued by the spl lending reserve
    pub collateral_mint: Pubkey,
    /// address of the reserve account
    pub reserve_account: Pubkey,
    /// address of the reserve's liquidity token account
    /// for usdc optimizer vaults this would be the reserves
    /// usdc token account aaddress
    pub reserve_liquidity_account: Pubkey,
    /// address of the lending market this reserve is a part of
    pub lending_market_account: Pubkey,
    /// address of the derived lending market authority
    pub derived_lending_market_authority: Pubkey,
    /// the vault's collateral token account
    pub collateral_token_account: Pubkey,
    /// oracle keys required for this spl lending platform
    /// usually only one is required, but for future compatability
    /// we allow up to 3
    ///
    /// if the first element in the array is a default public key
    /// it means no oracle is required. this can happen with certain platforms
    /// such as port's usdc and pai reserves
    pub oracle_keys: [Pubkey; 3],
    /// index 0 of this array is the oracle program used by index 0 of oracle_keys
    pub oracle_programs: [Pubkey; 3],
    /// the information account this config is for
    pub information_account: Pubkey,
    /// the nonce of this account
    pub nonce: u8,
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub buffer: [u8; 256],
}

#[account(zero_copy)]
#[cfg_attr(not(target_arch = "bpf"), derive(Derivative))]
#[cfg_attr(not(target_arch = "bpf"), derive(TypeLayout))]
#[cfg_attr(not(target_arch = "bpf"), derivative(Debug))]
pub struct MangoV3Config {
    /// mango group account used to store various markets, etc..
    pub mango_group_account: Pubkey,
    /// the account  used to store information about the vaults positions
    /// it's essentially the spl lending obligation equivalent for mango v3
    pub mango_account: Pubkey,
    pub mango_cache: Pubkey,
    pub mango_root_bank: Pubkey,
    pub mango_node_bank: Pubkey,
    // todo(bonedaddy): not needed right now?
    pub mango_group_token_account: Pubkey,
    /// the information account this config is for
    pub information_account: Pubkey,
    /// the nonce of this account
    pub nonce: u8,
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub buffer: [u8; 256],
}

/// type of lending programs that can be used by this vault
/// the value contained within the enum is the name of the platform
///
/// for example ProgramType::SplUnmodified("SOLEND") would be
/// the spl token lending program deployed by Solend
#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Eq)]
pub enum ProgramType {
    /// indicates this is a standard spl token lending program
    /// which doesn't implement a modified instruction set
    ///
    /// this includes Tulip Protocol and Port Finance. although
    /// tulip uses a modified instruction set, the instructions
    /// the lending optimizer needs to interact with are unmodified
    SplUnmodified,
    /// indicates this is an spl token lending program implementation
    /// which is based off of the solend implementation using a dual
    /// oracle system (switchboard & pyth)
    SplModifiedSolend,
    /// indicates this is an mango v3 lending program implementation
    MangoV3,
    Unknown,
}

impl super::Base for LendingOptimizerV1 {
    fn base(&self) -> VaultBaseV1 {
        self.base
    }
    fn shares_mut(&mut self) -> &mut dyn TokenizedShares {
        &mut self.base
    }
    fn shares(&self) -> &dyn TokenizedShares {
        &self.base
    }
    fn init(&mut self, args: &InitVaultArgsV1) {
        msg_panic!("noop");
    }
    fn farm(&self) -> Farm {
        Farm::from(self.base.farm)
    }
    fn sync_shares(&mut self, mint: &anchor_spl::token::Mint) {
        self.base.sync_shares(mint);
    }
}



/// returns the underlying destination address
/// for a given lending platform
pub fn fetch_underlying_destination(
    config_data_account: &AccountInfo<'_>,
    program_type: ProgramType,
) -> Pubkey {
    match program_type {
        ProgramType::MangoV3 => {
            let loader: AccountLoader<MangoV3Config> =
                AccountLoader::try_from(config_data_account).unwrap();
            {
                let config = loader.load().unwrap();
                config.mango_group_token_account
            }
        }
        ProgramType::SplModifiedSolend | ProgramType::SplUnmodified => {
            let loader: AccountLoader<SplLendingConfig> =
                AccountLoader::try_from(config_data_account).unwrap();
            {
                let config = loader.load().unwrap();
                config.reserve_liquidity_account
            }
        }
        _ => msg_panic!("unsupported program type"),
    }
}


impl Default for ProgramType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<ProgramType> for u64 {
    fn from(val: ProgramType) -> Self {
        match val {
            ProgramType::SplUnmodified => 0_u64,
            ProgramType::SplModifiedSolend => 1_u64,
            ProgramType::MangoV3 => 2_u64,
            ProgramType::Unknown => isize::MAX as u64,
        }
    }
}

impl From<u64> for ProgramType {
    fn from(val: u64) -> Self {
        match val {
            0 => ProgramType::SplUnmodified,
            1 => ProgramType::SplModifiedSolend,
            2 => ProgramType::MangoV3,
            _ => ProgramType::Unknown,
        }
    }
}

impl ToString for ProgramType {
    fn to_string(&self) -> String {
        match self {
            ProgramType::SplUnmodified => String::from("SplUnmodified"),
            ProgramType::SplModifiedSolend => String::from("SplModifiedSolend"),
            ProgramType::MangoV3 => String::from("MangoV3"),
            ProgramType::Unknown => String::from("Unknown"),
        }
    }
}

impl std::str::FromStr for ProgramType {
    type Err = ProgramError;
    fn from_str(s: &str) -> std::result::Result<Self, ProgramError> {
        match s.to_ascii_uppercase().as_str() {
            "SPLUNMODIFIED" => Ok(ProgramType::SplUnmodified),
            "SPLMODIFIEDSOLEND" => Ok(ProgramType::SplModifiedSolend),
            "MANGOV3" => Ok(ProgramType::MangoV3),
            _ => Err(ProgramError::InvalidArgument.into()),
        }
    }
}