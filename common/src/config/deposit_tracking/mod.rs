//! The `deposit_tracking` module provides functionality for interacting with
//! the deposit tracking account, used to track rewards, and hold onto funds before
//! a either withdraws the tokenized shares, or "unlocks" (burns) the tokenized shares
//! to retrieve the underlying asset.
//!
//! The `traits` submodule provides a set of traits used to easily return accounts needed
//! for various instructions, while the `derivations` submodule contains functions used
//! to derive deposit tracking addresses

pub mod derivations;
pub mod issue_shares;
pub mod register;
pub mod traits;
pub mod withdraw;
