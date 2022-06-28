use phuquocdog_runtime::{
    opaque::Block, opaque::SessionKeys, AccountId, BabeConfig, Balance, BalancesConfig,
    GenesisConfig, GrandpaConfig, ImOnlineConfig, OctopusAppchainConfig, OctopusLposConfig,
    SessionConfig, Signature, SudoConfig, SystemConfig, WASM_BINARY,

};
use phuquocdog_runtime::constants::currency::{OCTS, UNITS as PHUQUOCDOG};
use beefy_primitives::crypto::AuthorityId as BeefyId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_octopus_appchain::AuthorityId as OctopusId;
use sc_chain_spec::ChainSpecExtension;
use serde::{Deserialize, Serialize};
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{sr25519, Pair, Public, crypto::UncheckedInto};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use sc_service::{ChainType, Properties};
use hex_literal::hex;
// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// Block numbers with known hashes.
    pub fork_blocks: sc_client_api::ForkBlocks<Block>,
    /// Known bad block hashes.
    pub bad_blocks: sc_client_api::BadBlocks<Block>,
    /// The light sync state extension used by the sync-state rpc.
    pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;


fn session_keys(
    babe: BabeId,
    grandpa: GrandpaId,
    im_online: ImOnlineId,
    beefy: BeefyId,
    octopus: OctopusId,
) -> SessionKeys {
    SessionKeys { babe, grandpa, im_online, beefy, octopus }
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(
    seed: &str,
    stash_amount: Balance
) -> (AccountId, BabeId, GrandpaId, ImOnlineId, BeefyId, OctopusId, Balance) {
    (
        get_account_id_from_seed::<sr25519::Public>(seed),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<ImOnlineId>(seed),
        get_from_seed::<BeefyId>(seed),
        get_from_seed::<OctopusId>(seed),
        stash_amount,
    )
}

pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "dev",
        ChainType::Development,
        move || {
            genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![authority_keys_from_seed("Alice", 100 * OCTS)],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    (
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        // Balance amount
                        20_000_000 * PHUQUOCDOG,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Bob"),
                        // Balance amount
                        40_000_000 * PHUQUOCDOG,

                    )
                    
                ],
                // Appchain config
                appchain_config(
                    // Relay Contract
                    "",
                    // Asset Id by Name
                    "usdc.dev",
                    // Premined Amount
                    875_000_000 * PHUQUOCDOG,
                    // Era Payout
                    68_493 * PHUQUOCDOG,
                ),
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        None,
        // Properties
        None,
        // Extensions
        Default::default(),
    ))
}


pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
    let properties = phuquocdog_properties();
    Ok(ChainSpec::from_genesis(
        // Name
        "Local Testnet Phu Quoc Dog",
        // ID
        "local_phuquocdog",
        ChainType::Local,
        move || {
            genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![
                    authority_keys_from_seed("Alice", 100 * OCTS), 
                    authority_keys_from_seed("Bob", 100 * OCTS)
                ],
                // Sudo account
                // 5Fk6QsYKvDXxdXumGdHnNQ7V7FziREy6qn8WjDLEWF8WsbU3
                hex!["a2bf32e50edd79c181888da41c80c67c191e9e6b29d3f2efb102ca0e2b53c558"].into(),
                // Pre-funded accounts
                vec![
                    (
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        // Balance amount
                        20_000_000 * PHUQUOCDOG,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Bob"),
                        // Balance amount
                        40_000_000 * PHUQUOCDOG,

                    ),
                    (

                        get_account_id_from_seed::<sr25519::Public>("Charlie"),
                        // Balance amount
                        60_000_000 * PHUQUOCDOG,

                    ),

                ],
                // Appchain config
                appchain_config(
                    // Relay Contract
                    "",
                    // Asset Id by Name
                    "usdc.testnet",
                    // Premined Amount
                    800_000_000 * PHUQUOCDOG,
                    // Era Payout
                    68_493 * PHUQUOCDOG,
                ),
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        Some("phuquocdog-local-testnet"),
        None,
        // Properties
        Some(properties),
        // Extensions
        Default::default(),
    ))
}

pub fn quark_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
    let properties = phuquocdog_properties();
    Ok(ChainSpec::from_genesis(
        // Name
        "Quark Phu Quoc Dog",
        // ID
        "quark_phuquocdog",
        ChainType::Local,
        move || {
            genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![
                    
                    (
                        // 5FNCTJVDxfFnmUYKHqbJHjUi7UFbZ6pzC39sL6E5RVpB4vc9
                        hex!["920c238572e2b31c2efd19dad1a5674c8188388d9a30d0d01847759a5dc64069"].into(),
                        // 5CyLUbfTe941tZDvQQ5AYPXZ6zzqwS987DTwFGnZ3yPFX5wB
                        hex!["2824087e4d670acc6f2ac4251736b7fb581b5bff414437b6abc88dc118ea8d5c"].unchecked_into(),
                        // 5Fm7Lc3XDxxbH4LBKxn1tf44P1R5M5cm2vmuLZbUnPFLfu5p
                        hex!["a3859016b0b17b7ed6a5b2efcb4ce0e2b6b56ec8594d416c0ea3685929f0a15c"].unchecked_into(),
                        // 5CahSqUXepwzCkbC7KNUSghUcuJxPDPKiQ4ow144Gb9qBPsX
                        hex!["16dffa9a82c7bb62f0f9929407223bf156458a4e7970ec4007ab2da7fb389f7d"].unchecked_into(),
                        // 5Hcep686fjo1arF4Tyu8it19TcR2v4RTqSt7iMupaHdfZadS
                        hex!["030cb659f738fdbf04ddc618e0ac8969ee61eee96ef785b216014ef30861c1c1ee"].unchecked_into(),
                        // 5ED8WWRaKpXAFkb3YyQqi9dBtxZ2JutcjKBrKZmcUNdDeqSM
                        hex!["5ee56ae307a313fa92b07f288f0612ce9f49dc2e3e5c866e9c69157fc107ca5e"].unchecked_into(),
                        100*OCTS
                    ),
                    (
                        // 5DP3mCevjzqrYhJgPpQFkpoERKg55K422u5KiRGPQaoJEgRH
                        hex!["3a39a8d0654e0f52b2ee8202ed3488e7a82650dde0daadaddbc8ea825e408d13"].into(),
                        // 5G4AdD8rQ6MHp2K1L7vF1E43eX69JMZDQ1vknonsALwGQMwW
                        hex!["b087cc20818f98e543c55989afccd3ec28c57e425dae970d9dd63cad806c1f6d"].unchecked_into(),
                        // 5FKFid7kAaVFkfbpShH8dzw3wJipiuGPruTzc6WB2WKMviUX
                        hex!["8fcd640390db86812092a0b2b244aac9d8375be2c0a3434eb9062b58643c60fb"].unchecked_into(),
                        // 5DknzWSQVCpo7bNf2NnBsjb529K2WVpvGv6Q3kn9RgcFgoeQ
                        hex!["4acf560d0aa80158ee06971c0ebbf4e6a1a407e6de2df16a003a765b73e63d7b"].unchecked_into(),
                        // 5DuJnuVcofovauVDYQ9ctmaqPhMzKRy7S9fBt6AtK86gJz9x
                        hex!["0385231c2ab517834c0becc1d19ff97fc6c860c85aff9e5f62e8f453f5e0869756"].unchecked_into(),
                        // 5Gx1jpcSKmGyQSpgijuCY8hFS9EiHhSV93RqZPARbridYbVe
                        hex!["d812a53b3947f3c1ab20788695aa36eaffd75eeadf470b879894a9d28e3bce46"].unchecked_into(),
                        100*OCTS
                    ),
                    (
                    // 5DJQ1NXeThmu2N5yQHZUsY64Lmgm95nnchpRWi1nSBU2rgod
                        hex!["36ad94b252606800bc80869baf453663ac2e9276e83f0401107384c053552f3e"].into(),
                        // 5FHCHVMPD9VfpzMcGVyL7gqkq2Rd9NomkHFHP8BzP8isUBnh
                        hex!["8e3b579b007999dce44a28bb266f73b54e6f7ec219c495ae23fe0dc3c101e158"].unchecked_into(),
                        // 5EUsrdaXAAJ87Y7yCRdrYKeyHdTYbSr9tJFCYEy12CNap2v2
                        hex!["6ae80477725a1e4f3194fac59286662ea491c9461cb54909432228351be3474a"].unchecked_into(),
                        // 5GRarw8oivnRh5ViPC9kH6ztbPNiyrfb61BitYz2YzhoqS4L
                        hex!["c0dd89e234665e119ac8396af69c37d1956ffbf4a0173c21ee5872fea2366026"].unchecked_into(),
                        // 5Ey5JKNwCckkMjv4mmA4cKHVQXmnzpc5cuqsJNUuwEJsHcMm
                        hex!["022f7af97db2197324f179c8c890681dac54a904612831eb31f43d1dea996feffa"].unchecked_into(),
                        // 5HN18NBLtYzmsvADDv85714HDcu7VBortKsYsG1WEbf2bKtZ
                        hex!["ea5e7a30648ac759d13ed2ca72ec2bc678438cbc1712717364c77fe638dd9c20"].unchecked_into(),
                        100*OCTS
                    ),


                ],
                // Sudo account
                // 5Fk6QsYKvDXxdXumGdHnNQ7V7FziREy6qn8WjDLEWF8WsbU3
                hex!["a2bf32e50edd79c181888da41c80c67c191e9e6b29d3f2efb102ca0e2b53c558"].into(),
                // Pre-funded accounts
                vec![
                    (
                        //key "royal novel glad piece waste napkin little pioneer decline fancy train sell"
                        //5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV
                        hex!["a2bf32e50edd79c181888da41c80c67c191e9e6b29d3f2efb102ca0e2b53c558"].into(),  
                        // Balance amount
                        20_000_000 * PHUQUOCDOG,
                    ),
                    (
                        //senior success sample upper sample rifle bird suit piano across nest remove
                        //5CfSTwTG3zLvuLCZPKotxcMgrt3m6migEhwtxvuSvUM29JYK
                        hex!["1a7dc684b91eb368c1fe8f7f41833bd9e6d70cc620658b0cbcba849e5f175047"].into(),
                        // Balance amount
                        40_000_000 * PHUQUOCDOG,

                    ),
                    (
                        //traffic fashion slice forum capital calm service result camera warrior increase appear
                        //5FeNn9veT49i6NqMFcsp2wFenctB6ovfPuCeyaSztrbrvQph
                        hex!["9e62d2898f3351c2dc39c9efd351efd8178a44cdb9dda0ee5ecb6c753b877552"].into(),
                        // Balance amount
                        60_000_000 * PHUQUOCDOG,

                    ),
                ],
                // Appchain config
                appchain_config(
                    // Relay Contract
                    "phuquocdog.registry.test_oct.testnet",
                    // Asset Id by Name
                    "usdc.testnet",
                    // Premined Amount
                    800_000_000 * PHUQUOCDOG,
                    // Era Payout
                    68_493 * PHUQUOCDOG,
                ),
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        Some("quark-phuquocdog"),
        None,
        // Properties
        Some(properties),
        // Extensions
        Default::default(),
    ))
}

/// Configure initial storage state for FRAME modules.
/// 
fn genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AccountId, BabeId, GrandpaId, ImOnlineId, BeefyId, OctopusId, Balance)>,
    root_key: AccountId,
    endowed_accounts: Vec<(AccountId, Balance)>,
    appchain_config: (String, String, Balance, Balance),
    _enable_println: bool,
) -> GenesisConfig {
    
    GenesisConfig {
        system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
        },
        balances: BalancesConfig {
            balances: endowed_accounts.iter().map(|x| (x.0.clone(), x.1)).collect(),
        },
        session: SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(
                            x.1.clone(),
                            x.2.clone(),
                            x.3.clone(),
                            x.4.clone(),
                            x.5.clone(),
                        ),
                    )
                })
                .collect::<Vec<_>>(),
        },
        babe: BabeConfig {
            authorities: vec![],
            epoch_config: Some(phuquocdog_runtime::BABE_GENESIS_EPOCH_CONFIG),
        },
        im_online: ImOnlineConfig { keys: vec![] },
        grandpa: GrandpaConfig { authorities: vec![] },
        transaction_payment: Default::default(),
        beefy: Default::default(),
        octopus_appchain: OctopusAppchainConfig {
            anchor_contract: appchain_config.0,
            asset_id_by_name: vec![(appchain_config.1, 0)],
            premined_amount: appchain_config.2,
            validators: initial_authorities.iter().map(|x| (x.0.clone(), x.6)).collect()
        },
        octopus_lpos: OctopusLposConfig { era_payout: appchain_config.3, ..Default::default() },
        octopus_assets: Default::default(),
        sudo: SudoConfig {
            // Assign network admin rights.
            key: Some(root_key)
        },
    }
}

pub fn mainnet_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../../resources/octopus-mainnet.json")[..])
}
pub fn testnet_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../../resources/octopus-testnet.json")[..])
}

/// Helper function to generate an properties
pub fn phuquocdog_properties() -> Properties {
    let mut properties = Properties::new();
    //properties.insert("ss58Format".into(), 90.into());
    properties.insert("ss58Format".into(), 42.into());
    properties.insert("tokenDecimals".into(), 10.into());
    properties.insert("tokenSymbol".into(), "PQD".into());

    properties
}
/// Helper function to generate appchain config
pub fn appchain_config(
    relay_contract: &str,
    asset_id_by_name: &str,
    premined_amount: Balance,
    era_payout: Balance,
) -> (String, String, Balance, Balance) {
    (relay_contract.to_string(), asset_id_by_name.to_string(), premined_amount, era_payout)
}
