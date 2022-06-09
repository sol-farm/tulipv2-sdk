# tulipv2-sdk

Rust sdk for on-chain (cpi) and off-chain (client-side) usage of Tulip V2 vaults, as well as sparse support for Tulip V1 lending programs.

# V1 Support

Within the `v1` folder you will find a single `tulipv1-sdk` crate, which provides selected functions, accounts, etc... of v1

# Examples

* See [tulipv2-sdk-examples](https://github.com/sol-farm/tulipv2-sdk-examples)

# Warning

> **Unaudited, possibly untested code** developers take no responsibility for any (financial, physical, emotional, etc..) damage  that results from usage of these libraries.


# Usage

## Lending Optimizers

### Off-Chain Clients

For now this wont compile, but should serve as a sufficient example to get start

```rust
use tulipv2_sdk_common;
use tulipv2_sdk_farm;
use solana_sdk::signature::Keypair;
fn main() {
    let rpc = .. // create rpc client
    let kp = Keypair::new();
    let registration_trait = tulipv2_sdk_common::config::lending::usdc::multi_deposit::ProgramConfig::register_deposit_tracking_ix(kp.pubkey());
    let registration_ix = registration_trait.instruction(tulipv2_sdk_farms::Farm::Lending{name: tulipv2_sdk_farms::lending::Lending::MULTI_DEPOSIT}).unwrap();
    let hold_creation_ix = spl_associated_token_account::create_associated_token_account(
        &kp.pubkey(),
        &registration_trait.deposit_tracking_pda(),
        &registration_trait.shares_mint(),
    );
    let mut tx = Transaction::new_with_payer(&[hold_creation_ix, registration_ix], Some(&kp.pubkey()));
    tx.sign(&vec![&kp], rpc.get_latest_blockhash().unwrap();)
    let sig = rpc.send_and_confirm_transaction(&x).unwrap();
    println!("sent deposit tracking registration {}", sig);
}
```