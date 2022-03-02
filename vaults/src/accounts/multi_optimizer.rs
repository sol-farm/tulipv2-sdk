use super::lending_optimizer::*;
use super::{vault_base::VaultBaseV1, InitVaultArgsV1};
use tulipv2_sdk_common::msg_panic;
use tulipv2_sdk_farms::Farm;

use anchor_lang::prelude::*;
use arrform::{arrform, ArrForm};
use tulipv2_sdk_common::{traits::vault::TokenizedShares, DEFAULT_KEY};
#[cfg(not(target_arch = "bpf"))]
use derivative::*;

use itertools::Itertools;
use std::iter::FromIterator;
#[cfg(not(target_arch = "bpf"))]
use type_layout::TypeLayout;


pub const MULTI_DEPOSIT_OPTIMIZER_ACCOUNT_SIZE: usize = 2072;
pub const MULTI_REBALANCE_TRANSITION_ACCOUNT_SIZE: usize = 408;
/// the max number of allowed slots in between when a standalone vault
/// rebase happens, and a multideposit rebase takes place.
///
/// 360 slots is approximately 3 minutes
pub const STANDALONE_VAULT_REBASE_SLOT_WINDOW: u64 = 360;
pub const MAX_STANDALONE_VAULTS: usize = 6;

#[cfg_attr(not(target_arch = "bpf"), derive(Debug))]
/// wrapper type of Vec<StandaloneVaultCacheV1> intented to be a return type
/// for vaults containing a deposit greater than 0
pub struct ActiveStandaloneVaults(Vec<StandaloneVaultCacheV1>);

#[account(zero_copy)]
#[cfg_attr(not(target_arch = "bpf"), derive(Derivative))]
#[cfg_attr(not(target_arch = "bpf"), derive(TypeLayout))]
#[cfg_attr(not(target_arch = "bpf"), derivative(Debug))]
/// enables depositing into one or more
/// lending optimizer vaults, allowing for
/// multi-platform farming
pub struct MultiDepositOptimizerV1 {
    pub base: VaultBaseV1,
    /// the slot at which a rebase last occured
    pub last_rebase_slot: u64,
    /// standalone vaults providing specific lending functionality
    /// for a given asset to a given paltform. for example Tulip USDC
    /// would be a standalone vault for Tulip USDC lending
    pub standalone_vaults: [StandaloneVaultCacheV1; 6],
    /// the standalone that incoming funds which are in the deposit queue
    /// are to be swept to. this can be adjust by redirecting to a new
    /// standalone vault
    pub target_vault: Pubkey,
    pub state_transition_account: Pubkey,
    /// the minimum balance required to be deposited into a standalone vault
    /// for it to be able to be rebalanced. If this is 1 USDC, then it would be `1000000`
    pub minimum_rebalance_amount: u64,
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub buffer: [u8; 272],
}

#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize)]
#[cfg_attr(not(target_arch = "bpf"), derive(Derivative))]
#[cfg_attr(not(target_arch = "bpf"), derive(TypeLayout))]
#[cfg_attr(not(target_arch = "bpf"), derivative(Debug))]
#[repr(C)]
/// configuration information for a standalone vault
/// that is part of a multi-deposit lending optimizer
pub struct StandaloneVaultCacheV1 {
    /// the address of the member vault account
    /// that is, the address of the LendingOptimiverV1 vault
    pub vault_address: Pubkey,
    /// the amount of underlying asset which has been
    /// depositedi nto this member vault
    pub deposited_balance: u64,
    /// the program type being farmed by this member vault
    pub program_type: ProgramType,
    /// the address of the farm's lending program,
    pub program_address: Pubkey,
    /// address of the vault shares mint
    pub shares_mint: Pubkey,
    /// address of the multi deposit optimizers shares account
    /// for this particular standalone vault
    pub shares_account: Pubkey,
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub _alignment: [u8; 7],
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub buffer: [u64; 6],
}

#[account] // not zero copy so we dont care about padding
#[cfg_attr(not(target_arch = "bpf"), derive(Derivative))]
#[cfg_attr(not(target_arch = "bpf"), derive(TypeLayout))]
#[cfg_attr(not(target_arch = "bpf"), derivative(Debug))]
pub struct RebalanceStateTransitionV1 {
    /// the address of the muli deposit optimizer this transition account belongs too
    pub optimizer_vault: Pubkey,
    /// amount to remove from vault/platform_a
    /// this is used for internal tracking, and will not represent the
    /// final amount that needs to be deducted and added to the standalone cache
    pub vault_removal_amount_a: u64,
    /// once vaultA has had liquidity removed, this represents the value
    /// which was removed, and the value which is to be added
    pub vault_supply_amount_b: u64,
    /// NO LONGER USED
    pub unused_two: u64,
    /// NO LONGER USED
    pub unused_three: u64,
    pub vault_address_a: Pubkey,
    pub vault_address_b: Pubkey,
    pub vault_a_program_type: ProgramType,
    pub vault_b_program_type: ProgramType,
    pub state: RebalanceStates,
    /// the timestamp at which a rebalance last completed at
    pub last_completion_ts: i64,
    #[cfg_attr(not(target_arch = "bpf"), derivative(Debug = "ignore"))]
    pub buffer: [u8; 256],
}

#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RebalanceStates {
    Inactive = 0,
    Started = 1,
    VaultARemoved = 2,
    VaultABRebalanced = 3,
}

impl super::Base for MultiDepositOptimizerV1 {
    fn base(&self) -> VaultBaseV1 {
        self.base
    }
    fn shares_mut(&mut self) -> &mut dyn TokenizedShares {
        &mut self.base
    }
    fn shares(&self) -> &dyn TokenizedShares {
        &self.base
    }
    /// unlike the majority of other vault implementations
    /// the lending optimizer initializes deposits, withdraws
    /// and compoudning to disabled
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

impl RebalanceStateTransitionV1 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn current_state(&self) -> RebalanceStates {
        self.state
    }
    pub fn next_state(&self) -> RebalanceStates {
        match self.state {
            RebalanceStates::Inactive => RebalanceStates::Started,
            RebalanceStates::Started => RebalanceStates::VaultARemoved,
            RebalanceStates::VaultARemoved => RebalanceStates::VaultABRebalanced,
            RebalanceStates::VaultABRebalanced => RebalanceStates::Inactive,
        }
    }
    /// returns the removal amount
    pub fn remove_amount(&self) -> u64 {
        self.vault_removal_amount_a
    }
    /// returns the supply amount
    pub fn supply_amount(&self) -> u64 {
        self.vault_supply_amount_b
    }
}

impl MultiDepositOptimizerV1 {
    pub fn new() -> Self {
        Self::default()
    }
    /// returns the top two standalone vaults sorted in descending order of total_deposited_balance
    /// returns InsufficientFunds if no vaults have deposits
    pub fn top_two_deposits(&self) -> Result<[StandaloneVaultCacheV1; 2]> {
        // filter our standalone vaults with a default_key as the address
        // indicating this is nota currently used vault
        let mut vaults = self.active_deposits();
        if vaults.is_empty() {
            return Err(ProgramError::InsufficientFunds.into());
        }
        vaults.sort();

        let top_two = vaults.top_two();
        // it is possible that one of the two has a deposited_balance of 0
        // but if both have a deposited balance of 0, it means we're depositing
        // into no vaults
        if top_two[0].deposited_balance == 0 && top_two[1].deposited_balance == 0 {
            return Err(ProgramError::InsufficientFunds.into());
        }
        Ok(top_two)
    }
    /// returns the bottom two standalone vaults sorted in ascendign order of total_deposited_balance
    /// returns an InsufficientFunds if no vaults have deposits
    pub fn bottom_two_deposits(&self) -> Result<[StandaloneVaultCacheV1; 2]> {
        // filter our standalone vaults with a default_key as the address
        // indicating this is nota currently used vault
        let mut vaults = self.active_deposits();
        if vaults.is_empty() {
            return Err(ProgramError::InsufficientFunds.into());
        }
        vaults.sort();

        let top_two = vaults.bottom_two();
        // it is possible that one of the two has a deposited_balance of 0
        // but if both have a deposited balance of 0, it means we're depositing
        // into no vaults
        if top_two[0].deposited_balance == 0 && top_two[1].deposited_balance == 0 {
            return Err(ProgramError::InsufficientFunds.into());
        }
        Ok(top_two)
    }
    /// returns an object containing all standalone vaults with active deposits
    pub fn active_deposits(&self) -> ActiveStandaloneVaults {
        self.standalone_vaults
            .clone()
            .iter()
            .filter(|x| {
                // deference the double pointer
                (*x).vault_address.ne(&DEFAULT_KEY) && (*x).deposited_balance > 0
            })
            .collect::<ActiveStandaloneVaults>()
    }
    /// returns the number of standalone vaults which are activated
    /// in that they have a non DEFAULT_KEY vault_address
    pub fn non_default_vaults(&self) -> u64 {
        let mut count = 0;
        for standalone in self.standalone_vaults.iter() {
            if standalone.vault_address.ne(&DEFAULT_KEY) {
                count += 1
            }
        }
        count
    }

    /// returns true if there is free space to add a standalone vault
    pub fn free_standalone_space(&self) -> bool {
        self.standalone_vaults[self.standalone_vaults.len() - 1]
            .vault_address
            .eq(&DEFAULT_KEY)
    }
    /// returns the number of available free slots for standalone vaults
    pub fn free_standalone_space_count(&self) -> usize {
        let mut count = 0;
        for vault in self.standalone_vaults.iter() {
            if vault.vault_address.eq(&DEFAULT_KEY) {
                count += 1
            }
        }
        count
    }
    pub fn standalone_exists(&self, vault: &Pubkey) -> bool {
        for standalone in self.standalone_vaults.iter() {
            if standalone.vault_address.eq(vault) {
                return true;
            }
        }
        false
    }
}

impl Default for MultiDepositOptimizerV1 {
    fn default() -> Self {
        Self {
            base: Default::default(),
            last_rebase_slot: 0,
            standalone_vaults: [StandaloneVaultCacheV1::default(); 6],
            buffer: [0_u8; 272],
            target_vault: DEFAULT_KEY,
            state_transition_account: DEFAULT_KEY,
            minimum_rebalance_amount: 0,
        }
    }
}

impl Default for StandaloneVaultCacheV1 {
    fn default() -> Self {
        Self {
            deposited_balance: 0,
            vault_address: DEFAULT_KEY,
            program_address: DEFAULT_KEY,
            program_type: ProgramType::Unknown,
            shares_account: DEFAULT_KEY,
            shares_mint: DEFAULT_KEY,
            _alignment: [0_u8; 7],
            buffer: [0_u64; 6],
        }
    }
}

impl Default for RebalanceStates {
    fn default() -> Self {
        Self::Inactive
    }
}

impl Default for RebalanceStateTransitionV1 {
    fn default() -> Self {
        Self {
            optimizer_vault: DEFAULT_KEY,
            vault_removal_amount_a: 0,
            vault_supply_amount_b: 0,
            unused_two: 0,
            unused_three: 0,
            vault_address_a: DEFAULT_KEY,
            vault_address_b: DEFAULT_KEY,
            vault_a_program_type: ProgramType::Unknown,
            vault_b_program_type: ProgramType::Unknown,
            state: RebalanceStates::Inactive,
            last_completion_ts: 0,
            buffer: [0_u8; 256],
        }
    }
}

impl ActiveStandaloneVaults {
    /// initializes active vaults wrapper object with capacity of 6
    pub fn new() -> ActiveStandaloneVaults {
        ActiveStandaloneVaults(Vec::with_capacity(MAX_STANDALONE_VAULTS))
    }
    /// push a vault into the vec
    pub fn add(&mut self, elem: StandaloneVaultCacheV1) {
        self.0.push(elem);
    }
    /// returns the number of active deposits
    pub fn len(&self) -> usize {
        self.0.len()
    }
    /// sort the vec by deposited_balance
    pub fn sort(&mut self) {
        self.0 = std::mem::take(&mut self.0)
            .into_iter()
            .dedup_by(|a, b| a.vault_address.eq(&b.vault_address))
            .sorted_unstable_by_key(|x| x.deposited_balance)
            .collect();
    }
    /// return top two vaults by balance
    /// the first element is the highest balance of the two
    ///
    /// WARN: must be sorted
    pub fn top_two(&self) -> [StandaloneVaultCacheV1; 2] {
        let height = self.0.len();
        [self.0[height - 1], self.0[height - 2]]
    }
    /// return the bottom two vaults with a non 0 balance
    /// WARN: must be sorted
    pub fn bottom_two(&self) -> [StandaloneVaultCacheV1; 2] {
        // the index of the first non zero deposited balance vault
        // handle the best case scenario where we have a deposit into all vaults first
        // as its the shorest path
        let mut first_non_zero_idx = 0;
        for (idx, vault) in self.0.iter().enumerate() {
            if vault.deposited_balance > 0 {
                first_non_zero_idx = idx;
                break;
            }
        }
        let worst = self.0[first_non_zero_idx];
        // edge case for when we only have 1 deposit
        let second_worst = if first_non_zero_idx == self.0.len() - 1 {
            StandaloneVaultCacheV1::default()
        } else {
            self.0[first_non_zero_idx + 1]
        };
        [worst, second_worst]
    }
    pub fn vaults(&'_ self) -> &'_ Vec<StandaloneVaultCacheV1> {
        &self.0
    }
    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }
}

impl Default for ActiveStandaloneVaults {
    fn default() -> Self {
        Self(Vec::with_capacity(MAX_STANDALONE_VAULTS))
    }
}

impl<'a> FromIterator<&'a StandaloneVaultCacheV1> for ActiveStandaloneVaults {
    fn from_iter<I: IntoIterator<Item = &'a StandaloneVaultCacheV1>>(iter: I) -> Self {
        let mut c = ActiveStandaloneVaults::new();
        for i in iter {
            c.add(*i);
        }
        c
    }
}

