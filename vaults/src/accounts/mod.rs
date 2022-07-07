//! provides the core data structures used by v2 vaults, most importantly the `VaultBaseV1`.

#![deny(unused_must_use)]
#![deny(clippy::all)]
#![allow(clippy::bool_assert_comparison)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::extra_unused_lifetimes)]
#![allow(clippy::field_reassign_with_default)]

use self::vault_base::VaultBaseV1;
use anchor_lang::prelude::*;

use anchor_spl::token::Mint;

use tulipv2_sdk_common::traits::vault::TokenizedShares;
use tulipv2_sdk_farms::Farm;

pub mod lending_optimizer;
pub mod multi_optimizer;
pub mod orca_vault;
pub mod quarry_vault;
pub mod raydium_vault;
pub mod tracking;
pub mod vault_base;

/// Base trait defines a trait that must be implemented
/// by all custom vaults and is used to define shared methods
/// of accessing the underlying base vault
pub trait Base {
    /// returns a copy of the underlying base vault, used for accessing
    /// information about the vault base
    fn base(&self) -> VaultBaseV1;
    /// returns a mutable reference to the TokenizedShares trait
    /// allowing for managing of the underlying vault base
    fn shares_mut(&mut self) -> &mut dyn TokenizedShares;
    /// returns a reference to the TokenizedShares trait
    fn shares(&self) -> &dyn TokenizedShares;
    /// initializes the vault, and vault base applying generic configurations
    fn init(&mut self, args: &InitVaultArgsV1);
    /// returns base's farm type
    fn farm(&self) -> Farm;
    /// used to sync a vaults shares against the shares mint current supply
    fn sync_shares(&mut self, mint: &Mint);
}

/// shared arguments used for stage one initialization of all vaults
#[derive(Default)]
pub struct InitVaultArgsV1;

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone)]
pub struct InitFeeArgsV1;

/// derive the address of a vault, using the tag as an additional seed. while the
/// tag can be up any series of `u8`'s up to 32 bytes in length, it is intended to be used
/// with strings. there is no hard requirements for the format of this tag, however it is
/// recommended that if the tag is 31 bytes or less, you use a newline delimiter (\n)
/// so that UI, and other clients can render the data correctly
pub fn derive_vault_address(farm: &tulipv2_sdk_farms::Farm, tag: [u8; 32]) -> (Pubkey, u8) {
    let parts: [u64; 2] = (*farm).into();
    Pubkey::find_program_address(
        &[
            b"v1",
            &parts[0].to_le_bytes()[..],
            &parts[1].to_le_bytes()[..],
            &tag[..],
        ],
        &crate::ID,
    )
}

/// derive the vault base  address, which is the main pda
/// signing pda used by the vault itself.
pub fn derive_pda_address(vault: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[vault.as_ref()], &crate::ID)
}

/// derives the address used for the shares token mint
/// which uses the vault address, and the mint of the underlying
/// asset as the seeds
pub fn derive_shares_mint_address(vault: &Pubkey, underlying_mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[vault.as_ref(), underlying_mint.as_ref()], &crate::ID)
}

/// derive the address used for the underlying token mint withdraw queue
/// there is no corresponding function for deposit queue as it uses an ATA
pub fn derive_withdraw_queue_address(vault: &Pubkey, underlying_mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"withdrawqueue", vault.as_ref(), underlying_mint.as_ref()],
        &crate::ID,
    )
}

/// derive the address used for the underlying token mint compound queue
/// there is no corresponding function for deposit queue as it uses an ATA
pub fn derive_compound_queue_address(vault: &Pubkey, underlying_mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"compoundqueue", vault.as_ref(), underlying_mint.as_ref()],
        &crate::ID,
    )
}

/// derive the address used to store tracking account information in
pub fn derive_tracking_address(
    vault: &Pubkey,
    owner: &Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"tracking", vault.as_ref(), owner.as_ref()], program_id)
}

pub fn derive_ephemeral_tracking_address(
    vault: &Pubkey,
    owner: &Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"ephemeraltracking", vault.as_ref(), owner.as_ref()],
        program_id,
    )
}

/// derive the address used by the tracking account for signing
pub fn derive_tracking_pda_address(tracking_account: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[tracking_account.as_ref()], program_id)
}

/// derive the address used as the temporary account
/// for temporarily storing share tokens during a withdrawal process
pub fn derive_tracking_queue_address(tracking_pda: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"queue", tracking_pda.as_ref()], program_id)
}

/// derives the address used to store configuration data information
pub fn derive_lending_platform_config_data_address(platform_address: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"v1", platform_address.as_ref()], &crate::ID)
}
/// derives the address used to store lending platform
/// information in
pub fn derive_lending_platform_information_account(vault: &Pubkey, index: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[vault.as_ref(), &index.to_le_bytes()[..]], &crate::ID)
}

/// derives the address used by a lending optimizer vault for storing mango
/// account information in
pub fn derive_mango_account_address(vault: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[vault.as_ref(), b"mango"], &crate::ID)
}

/// for double dip orca vaults, we need an account that holds onto
/// the non double dip farm tokens before we revert them.
pub fn derive_tracking_orca_dd_queue_address(
    tracking_pda: &Pubkey,
    vault: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"ddwithdrawqueue", tracking_pda.as_ref(), vault.as_ref()],
        &crate::ID,
    )
}

/// derive the address used to store state transition information
/// during multi deposit vault rebalancing
pub fn derive_multi_deposit_state_transition_address(vault: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"statetransition", vault.as_ref()], &crate::ID)
}

/// derive the address used by quarry vaults to store configuration information
pub fn derive_quarry_vault_config_data_address(vault: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"config", vault.as_ref()], &crate::ID)
}

/// parses a formatted named as given by [vault_base::VaultBaseV1::formatted_name()]
/// into it's two parts, the farm name, and the tag
pub fn parse_formatted_name(formatted_name: &str) -> (String, String) {
    let parts: Vec<_> = formatted_name.split('-').collect();
    // set capacity to parts length
    let mut farm_name_parts = Vec::with_capacity(parts.len());
    let mut tag = String::new();
    for (_idx, part) in parts.iter().enumerate() {
        if (*part).contains("tag") {
            tag = match part.to_string().strip_prefix("tag(") {
                Some(part) => match part.strip_suffix(')') {
                    Some(part) => part.to_owned(),
                    None => panic!("failed to parse tag"),
                },
                None => panic!("failed to parse tag"),
            };
            break;
        } else {
            farm_name_parts.push(*part);
        }
    }
    let mut farm_name = String::with_capacity(std::mem::size_of_val(&farm_name_parts));
    for (idx, part) in farm_name_parts.iter().enumerate() {
        farm_name.push_str(*part);
        if idx < farm_name_parts.len() - 1 {
            farm_name.push('-');
        }
    }
    (farm_name, tag)
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use tulipv2_sdk_common::DEFAULT_KEY;
    use tulipv2_sdk_farms::Farm;

    use super::*;
    #[test]
    fn tracking_addresses() {
        let vault = DEFAULT_KEY;
        let owner = DEFAULT_KEY;
        let pid = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
        let (track, track_nonce) = derive_tracking_address(&vault, &owner, &pid);
        let (ephtrack, ephtrack_nonce) = derive_ephemeral_tracking_address(&vault, &owner, &pid);
        println!("track {}, track nonce {}", track, track_nonce);
        println!("ephtrack {}, ephtrack_nonce {}", ephtrack, ephtrack_nonce);
        assert_eq!(
            track.to_string(),
            "AMfs9gQR74UyJ52oMmpuxw2eL1qjLF5M87cFZTX2sQxH".to_string()
        );
        assert_eq!(track_nonce, 255);
        assert_eq!(
            ephtrack.to_string(),
            "2eMFBzgUq7sKuvvs8MZphyz3KUPAjC5o3R3vMMMTeMft".to_string()
        );
        assert_eq!(ephtrack_nonce, 255,);
        let (track_pda, track_pda_nonce) = derive_tracking_pda_address(&track, &pid);
        println!(
            "track pda {}, track pda nonce {}",
            track_pda, track_pda_nonce
        );
        assert_eq!(
            track_pda.to_string(),
            "6riZUKJKjGgic92kso1azrgUsjwRj8yfPwgg4PEZEcnt".to_string()
        );
        assert_eq!(track_pda_nonce, 255);
        let (track_queue, track_queue_nonce) = derive_tracking_queue_address(&track_pda, &pid);
        println!(
            "track queue {}, track queue nonce {}",
            track_queue, track_queue_nonce
        );
        assert_eq!(
            track_queue.to_string(),
            "JCtJ5HEvXpmWWsFPTVRACfwBiEMQV7r1gbFR57DNRmkS".to_string()
        );
        assert_eq!(track_queue_nonce, 255);
    }

    #[test]
    fn vault_addresses() {
        let farm = Farm::Raydium {
            name: tulipv2_sdk_farms::raydium::Raydium::ALEPHUSDC,
        };
        let (vault, vault_nonce) = derive_vault_address(&farm, [0_u8; 32]);
        println!("vault {}, vault nonce {}", vault, vault_nonce);
        assert_eq!(
            vault.to_string(),
            "8gjgbcRew7zZJkQ1cdHxMjGHzm5yXwtkgyGtFLTD6f1e".to_string()
        );
        assert_eq!(vault_nonce, 255);

        let (pda, pda_nonce) = derive_pda_address(&vault);
        println!("pda {}, pda nonce {}", pda, pda_nonce);
        assert_eq!(
            pda.to_string(),
            "3jk82ez7bVnxCCiRVV6qXGHh72Y2Q7LJdSGcAsyCf8jv".to_string()
        );
        assert_eq!(pda_nonce, 255);

        let (shares, shares_nonce) = derive_shares_mint_address(&vault, &DEFAULT_KEY);
        println!("shares {}, shares nonce {}", shares, shares_nonce);
        assert_eq!(
            shares.to_string(),
            "2Ldw8EAsKa7WiX4YdMQRx8SBCx6HRk8wfzRrBZcKhTmL".to_string()
        );
        assert_eq!(shares_nonce, 253);

        let (queue, queue_nonce) = derive_withdraw_queue_address(&vault, &DEFAULT_KEY);
        println!("queue {}, queue nonce {}", queue, queue_nonce);
        assert_eq!(
            queue.to_string(),
            "3NFBCpxhfGACbGL8GFf5RMhSbdaEjGiqzMpS3vC8sjAH".to_string()
        );
        assert_eq!(queue_nonce, 253);

        let (c_queue, c_queue_nonce) = derive_compound_queue_address(&vault, &DEFAULT_KEY);
        println!("c queue {}, c queue nonce {}", c_queue, c_queue_nonce);
        assert_eq!(
            c_queue.to_string(),
            "4x6xMfuuBzvHTFz18N3oB7TviwJQ1JRhQEewnsuUWqrF".to_string()
        );
        assert_eq!(c_queue_nonce, 255);

        let (state_addr, state_nonce) = derive_multi_deposit_state_transition_address(&vault);
        assert_eq!(
            state_addr.to_string(),
            "CTt8rN8UhatQXnj7MfXZjw7ZzTsvpr2HYse5p2BgQb31".to_string(),
        );
        println!("addr {}, nonce {}", state_addr, state_nonce);

        let (config_addr, nonce) = derive_quarry_vault_config_data_address(&vault);
        println!("addr {}, nonce {}", config_addr, nonce);
        assert_eq!(
            config_addr.to_string(),
            "9mjm5ryiWg94qsd4Chz8v7RDKRQrswj7u653C6xehMXs".to_string()
        );
        assert_eq!(nonce, 255);
    }
    #[test]
    fn test_parse_formatted_name() {
        let with_tag_1 = "LENDING-MULTI_DEPOSIT-tag(usdcv1)".to_string();
        let with_tag_2 = "LENDING-MULTI_DEPOSIT-tag(rayv1)".to_string();
        let with_no_tag_1 = "RAYDIUM-TULIP-USDC-tag()".to_string();
        let with_no_tag_2 = "RAYDIUM-RAY-USDC-tag()".to_string();

        let tests = vec![
            // want_farm, want_tag, sample
            ("LENDING-MULTI_DEPOSIT", "usdcv1", with_tag_1),
            ("LENDING-MULTI_DEPOSIT", "rayv1", with_tag_2),
            ("RAYDIUM-TULIP-USDC", "", with_no_tag_1),
            ("RAYDIUM-RAY-USDC", "", with_no_tag_2),
        ];
        for test in tests {
            let (got_farm_name, got_tag) = parse_formatted_name(&test.2);
            assert_eq!(&got_farm_name, test.0);
            assert_eq!(&got_tag, test.1);
        }
    }
}
