# tulipv2-sdk

# Warning

> **Unaudited, possibly untested sdk** Tulip Protocol takes no responsibility for any  (financial, physical, emotional, etc..) damage  that results from usage of this sdk, nor makes any guarantee as to the correctness of the sdk. Users of these crates take full responsibility for any mishaps that results from the usage of this sdk.


> **all crate versions before 0.9.17 are broken and will not work**

# Overview

`tulipv2-sdk` is a set of crates for interacting with Tulip V2 vaults, and V1 lending programs via CPI, however it is also usable with off-chain rust clients. The goal of this crate is to provide an easy to use sdk for working with the Tulip Protocol, while also serving as a sort of "configuration file".

There are 4 main crates which are detailed below

# Crates

## [`common`](https://crates.io/crates/tulipv2-sdk-common)

The `common` folder contains a crate `tulipv2-sdk-common` which bundles together common functionality used by all the other crates, while also containing a module called `config` which provides configuration variables for easy configuration management. The configuration sub-modules can be enabled/disabled with feature flags enabling you to import only the required dependencies.

### `common/config/deposit_tracking`

Provides configuration variables, traits, and instructions for deposit tracking account management

### `common/config/levfarm`

This module contains files in the form of `ray_X.rs` for Raydium leveraged yield farms, and `orca_X.rs` for Orca leveraged yield farms. Additionally a submodule `common/config/levfarm/reserves` provides configuration variables for all of the lending reserves.

### `common/config/strategy`

This module contains configuration information for the V2 strategy vaults


## [`farms`](https://crates.io/crates/tulipv2-sdk-farms)

The `farms` folder contains a crate `tulipv2-sdk-farms` which provides an enum named `Farm`, which is used to describe different platforms (ie Raydium) and farms within those platforms (ie RAY-USDC). In addition to this the farm key itself is used to enable deterministic derivation of vault addresses that are self describing.

The wire representation of the farm type / farm key, is a 2 element slice of u64's, where the first element (farm identifier) is the protocol, and the second element is the particular vault for that protocol. 


## [`lending`](https://crates.io/crates/tulipv2-sdk-lending)

The `lending` folder contains a crate `tulipv2-sdk-lending` which provides very basic support for creating instructions, and issuing CPI calls to Tulip's V1 lending program. It allows for the lending of assets through the `Obligation` account, while also allowing the caller to refresh obligations and refresh reserves.

## [`leveraged`](https://crates.io/crates/tulipv2-sdk-leveraged)

The `leveraged` folder contains a crate `tulipv2-sdk-leveraged` which provides accounts, instructions, and various helper functions for interacting with Tulip's leveraged yield farms

## [`vaults`](https://crates.io/crates/tulipv2-sdk-vaults)

The `vaults` folder contains a crate `tulipv2-sdk-vaults` which provides all v2 vault account types, and associated helper functions, etc..

# Examples


The `examples` folder contains an anchor program that provides usage examples for all of the crates, including lending, leverage farming, v1/v2 vaults, etc.. In addition to this, anchor unit tests are provided which show how to invoke the example program instructions.

The example program uses mainnet account cloning to simulate real-world usage, however due to some peculiarities with mainnet cloned accounts and localnet, certain tests will fail, namely:

* raydium vault deposit/withdraw
* v2 strategy vault withdrawals

As such the anchor unit tests will catch these errors masking them to prevent confusion. All such instances are documented both in the anchor unit test comments
