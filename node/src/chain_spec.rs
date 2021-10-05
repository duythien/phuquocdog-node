use frame_support::PalletId;
use grandpa_primitives::AuthorityId as GrandpaId;
use hex_literal::hex;
use itertools::Itertools;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
pub use phuquocdog_primitives::{AccountId, Balance, Signature};
use phuquocdog_primitives::Block;
use sc_chain_spec::ChainSpecExtension;
use sc_service::{ChainType,Properties};
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, Pair, Public, sr25519};
use sp_runtime::{Perbill, traits::{IdentifyAccount, Verify}};
use sp_runtime::traits::AccountIdConversion;

use phuquocdog_runtime::{
    AuthorityDiscoveryConfig, BabeConfig, BalancesConfig, CouncilConfig,
    IndicesConfig, SessionConfig, StakerStatus, StakingConfig,
    SudoConfig, SystemConfig, TechnicalCommitteeConfig, wasm_binary_unwrap,
};
use phuquocdog_runtime::constants::currency::PQD;
pub use phuquocdog_runtime::GenesisConfig;
use phuquocdog_runtime::SessionKeys;

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const MAINNET_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

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
    pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,

}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;


pub(crate) fn session_keys(
    grandpa: GrandpaId,
    babe: BabeId,
    im_online: ImOnlineId,
    authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
    SessionKeys {
        grandpa,
        babe,
        im_online,
        authority_discovery,
    }
}

fn quark_testnet_config_genesis() -> GenesisConfig {
    let initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )> = vec![
        (
        // 5FNCTJVDxfFnmUYKHqbJHjUi7UFbZ6pzC39sL6E5RVpB4vc9
        hex!["920c238572e2b31c2efd19dad1a5674c8188388d9a30d0d01847759a5dc64069"].into(),
        // 5GgaLpTUcgbCTGnwVkCjSSzZ5jTaEPuxtWGRDhi8M1BP1hTs
        hex!["cc4c78c7f22298f17e0e2dcefb7cff85b30e19dc1699cb9d1de00e5ea65a433d"].into(),
        // 5Fm7Lc3XDxxbH4LBKxn1tf44P1R5M5cm2vmuLZbUnPFLfu5p
        hex!["a3859016b0b17b7ed6a5b2efcb4ce0e2b6b56ec8594d416c0ea3685929f0a15c"].unchecked_into(),
        // 5CyLUbfTe941tZDvQQ5AYPXZ6zzqwS987DTwFGnZ3yPFX5wB
        hex!["2824087e4d670acc6f2ac4251736b7fb581b5bff414437b6abc88dc118ea8d5c"].unchecked_into(),
        // 5CahSqUXepwzCkbC7KNUSghUcuJxPDPKiQ4ow144Gb9qBPsX
        hex!["16dffa9a82c7bb62f0f9929407223bf156458a4e7970ec4007ab2da7fb389f7d"].unchecked_into(),
        // 5Eeard4qtNM8DBvqDEKn5GBAspbT7QEvhAjxSsYePB26XAiJ
        hex!["724f3e6ec8a61ea3dc5b76c00a049f84fd7f212443b01241e0a2bb4ce503b345"].unchecked_into(),
        ),
        (
        // 5DP3mCevjzqrYhJgPpQFkpoERKg55K422u5KiRGPQaoJEgRH
        hex!["3a39a8d0654e0f52b2ee8202ed3488e7a82650dde0daadaddbc8ea825e408d13"].into(),
        // 5HeTTicL5u17JCkDhAwcAHUXMGEzXbDLjPYmNC5ahKhwaLgt
        hex!["f6eb0cff5244d7437ed659ac34e6ea66daa857f3d1c580f452b8512ae7fdba0f"].into(),
        // 5FKFid7kAaVFkfbpShH8dzw3wJipiuGPruTzc6WB2WKMviUX
        hex!["8fcd640390db86812092a0b2b244aac9d8375be2c0a3434eb9062b58643c60fb"].unchecked_into(),
        // 5G4AdD8rQ6MHp2K1L7vF1E43eX69JMZDQ1vknonsALwGQMwW
        hex!["b087cc20818f98e543c55989afccd3ec28c57e425dae970d9dd63cad806c1f6d"].unchecked_into(),
        // 5DknzWSQVCpo7bNf2NnBsjb529K2WVpvGv6Q3kn9RgcFgoeQ
        hex!["4acf560d0aa80158ee06971c0ebbf4e6a1a407e6de2df16a003a765b73e63d7b"].unchecked_into(),
        // 5DhZENrJzzaJL2MwLsQsvxARhhAPCVXdHxs2oSJuJLxhUsbg
        hex!["485746d4cc0f20b5581f24b30f91b34d49a7b96b85bb8ba202f354aea8e14b1f"].unchecked_into(),
        ),
        (
        // 5DJQ1NXeThmu2N5yQHZUsY64Lmgm95nnchpRWi1nSBU2rgod
        hex!["36ad94b252606800bc80869baf453663ac2e9276e83f0401107384c053552f3e"].into(),
        // 5EWQq4ns7miu8B8ArsspZ9KBHX6gwjJXptJq5dbLgQucZvdc
        hex!["6c1386fd76e4eec0365a439db0decae0d5d715e33db934bc44be28f73df50674"].into(),
        // 5EUsrdaXAAJ87Y7yCRdrYKeyHdTYbSr9tJFCYEy12CNap2v2
        hex!["6ae80477725a1e4f3194fac59286662ea491c9461cb54909432228351be3474a"].unchecked_into(),
        // 5FHCHVMPD9VfpzMcGVyL7gqkq2Rd9NomkHFHP8BzP8isUBnh
        hex!["8e3b579b007999dce44a28bb266f73b54e6f7ec219c495ae23fe0dc3c101e158"].unchecked_into(),
        // 5GRarw8oivnRh5ViPC9kH6ztbPNiyrfb61BitYz2YzhoqS4L
        hex!["c0dd89e234665e119ac8396af69c37d1956ffbf4a0173c21ee5872fea2366026"].unchecked_into(),
        // 5CLfsFaNYPGQvpYkroN1qrWLt54Xpmn6shAxdE45bCy1cvgv
        hex!["0c2d3a4c604c4ad68e285cc1c401dd2665c1cd7193b16d4d9c854c27a9238a1a"].unchecked_into(),
        ),
    ];

    let root_key: AccountId = hex![
        // 5Fk6QsYKvDXxdXumGdHnNQ7V7FziREy6qn8WjDLEWF8WsbU3
        "a2bf32e50edd79c181888da41c80c67c191e9e6b29d3f2efb102ca0e2b53c558"
    ].into();

    testnet_genesis(
        initial_authorities,
        vec![],
        root_key,
    )
}

/// Staging testnet config.
pub fn quark_testnet_config() -> ChainSpec {
    let boot_nodes = vec![];
    ChainSpec::from_genesis(
        "Phuquocdog Test Net",
        "phuquocdog_quark_testnet",
        ChainType::Live,
        quark_testnet_config_genesis,
        boot_nodes,
        Some(
            TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
                .expect("Staging telemetry url is valid; qed"),
        ),
        None,
        None,
        Default::default(),
    )
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
    where
        AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(
    seed: &str,
) -> (
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
) {
    (
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
        get_account_id_from_seed::<sr25519::Public>(seed),
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<ImOnlineId>(seed),
        get_from_seed::<AuthorityDiscoveryId>(seed),
    )
}

fn development_config_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![authority_keys_from_seed("Alice")],
        vec![],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
    )
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Development",
        "dev",
        ChainType::Development,
        development_config_genesis,
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        Some(phuquocdog_properties()),

        Default::default(),
    )
}

fn soba_testnet_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![
            authority_keys_from_seed("Alice"),
            authority_keys_from_seed("Bob"),
        ],
        vec![],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
    )
}

/// Local testnet config ()
pub fn soba_testnet_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Local Testnet",
        "soba_testnet",
        ChainType::Local,
        soba_testnet_genesis,
        vec![],
        None,
        None,
        None,
        Default::default(),
    )
}


fn mainnet_genesis_constuctor() -> GenesisConfig {
    let initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )> = vec![
        (
        // 5Hov7urPicecsqvd7Cr297rduvQcNDLq5eLfYLz3ZhUzG4Mi
        hex!["fe21fffdfb381a80f2adde9ef09c11d08dadd91b6914ea89e54034f956e95d1f"].into(),
        // 5DkYAJ6crS4vAts7WrgfdAkXNDtPsgzxR931P3AKkLiQ8Zkp
        hex!["4a9d68e0f137af646d5a44f7e1f72ebe7464de1db37c7dd284de5056f3411469"].into(),
        // 5HZJ3jDQPHGSw4N9PifRnBBtc6zBV6s8L83vZ6ZF5sRTdBa6
        hex!["f2fb227bd118410c4047360403f20445b74d6fcb12780b265f231804dc34196f"].unchecked_into(),
        // 5ChYidPgEDXkZXyE8b9B9zAmBk9Sxix9T3jSAyj95MUdXpV8
        hex!["1c19510c363a448aa2d8d61b2815713adc2eb8ef5cfa5a8ca47535a693e94e6a"].unchecked_into(),
        // 5F6we7w7xJh7X4MQRTvK4211J8PKQMsCY6dLgBaeMARGuY17
        hex!["8669974ff7b4995bba623f916a16c8a7859ab6a703b804a5c62ee4e21068cd19"].unchecked_into(),
        // 5GnUccNMgiX1z6c8tXJYtiGg78UEsR282q8VbvKt256y3nTC
        hex!["d0cca9b80ca8e78a578c3e5247ee0a0c82983735dfdbac69fc56b5e14d39aa56"].unchecked_into(),
        ),
        (
        // 5FsSbEA2r4EBLph7mQ2o1CKupTXdpLmRYKxTHf1BoRDwKdV5
        hex!["a859d7fa7e0a4da51542736e983992cd36c8c44052e70d864126b25424215756"].into(),
        // 5GF4bRwTWxXgYNR5ZNacrfTNuWczN2JRSLzYh8fv7Es1iqSX
        hex!["b8d7321cc575466bc845bf878e1d317911684294b19e0bbc3546effa5851b908"].into(),
        // 5EFxhL67ZttLMMPYF1u9EJWYkG6G2CWDHDnQTygpydcQ2Jro
        hex!["610e1e6bd0ff4b913241640e4674e760bc56860f5305894347e1ee14f32bbc67"].unchecked_into(),
        // 5GHhczittjMrJCKgcfVzvTtvTEwhJnSfWJfsnJykek4dHsYe
        hex!["bada549c347707051f9714ae83d81e0165aeb9694eba2263d68bcba0e7aed65a"].unchecked_into(),
        // 5Dvz5deCgEjFGBsu9ZrX4PDQUPLib8ErbgPZFQ6SAQZL5CJ5
        hex!["52951dec5775c9007fdacf7adcd52159f6ce5d6200af1d49906f0061fdb55164"].unchecked_into(),
        // 5Ca7PVYsxQrWGpGHz8nuWQjc7349ZSnmDiqwts14TBV1khra
        hex!["166d54a6223dcbffaab9d56eddd81a4c6501285b2c6aed37d4915aaa8237b001"].unchecked_into(),
        ),
        (
        // 5FpiFT3pLNxb5jzMjyk6Nq17Ays647U1kSyX7aCKBHezcuiu
        hex!["a644d2034879b5d703198b1872e6f59ade4456009cbc5365a9231282f53a4765"].into(),
        // 5F1osXV65wVyGaWdqqThHdhy6sjh3tDWZXpCzVshzCr1QG2b
        hex!["827f363391387a30188b23d5269a2f49832fa0937f111d96b71b44874fcf3661"].into(),
        // 5EUGjPPsGbPCP5fpML9p9hiNj5YDYXRrX4iXJ3QQSx7edFdB
        hex!["6a71c6c7002df0cca262de274f9dad9c8c479fc709d6a6490c51e3a913e635bc"].unchecked_into(),
        // 5D4jHCtafUqxZboumYTrXMGit2tgzcVbNuyC5c32TkzHNrvk
        hex!["2c41061caf431e28b6511ac39c76bf2cb626fad5580593b8ba737eae90b3b950"].unchecked_into(),
        // 5HNXW5YVBuma3i7u11R3R7qBDB7TWb4aTfLqAH47UcfsStgY
        hex!["eac4b9fdca2fb47f7a56811060ff7c6d33f5d317816f30e1b7deead1efb79731"].unchecked_into(),
        // 5GCRwQ4NXTyg7NpqA6ib19ySFbKxzfhQ6YGrauui9FMha91H
        hex!["b6d54f8d61a36f86c03183150fbaea53bea789bb86042e226b9ced008be82537"].unchecked_into(),
        ),
    ];
    //5C8biCLaxCwtygHnwA5cZk31M977mUsERPniC9iGX1kj2oFJ
    let root_key = hex!["02f84ac5a75193ecc07256cf9715cb312a70b2355e28ea53a0f0902da5b2ef77"].into();
    testnet_genesis(initial_authorities, vec![], root_key)
}

pub fn mainnet_testnet_config() -> ChainSpec {
    let bootnodes = vec![];
    const PHUQUOCDOG_PROTOCOL_ID: &str = "phuquocdog";
    ChainSpec::from_genesis(
        "Phuquocdog Main Network",
        "phuquocdog_main_network",
        ChainType::Live,
        mainnet_genesis_constuctor,
        bootnodes,
        Some(
            TelemetryEndpoints::new(vec![(MAINNET_TELEMETRY_URL.to_string(), 0)])
                .expect("Staging telemetry url is valid; qed"),
        ),
        Some(PHUQUOCDOG_PROTOCOL_ID),
        Some(phuquocdog_properties()),
        Default::default(),
    )
}

fn adjust_treasury_balance_for_initial_validators(initial_validators: usize, endowment: u128) -> u128 {
    // The extra one is for root_key
    (initial_validators + 1) as u128 * endowment
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
    initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )>,
    _initial_nominators: Vec<AccountId>,
    root_key: AccountId,
) -> GenesisConfig {
    const ENDOWMENT: u128 = 1_000_000_000 * PQD;
    const STASH: u128 = 200 * PQD;
    // Total Supply in 90_000_000_000
    // Total funds in treasury for parachain auctions 25% and init root account
    let mut treasury_funds: u128 = 23_500_000_000 * PQD;
    treasury_funds = treasury_funds - adjust_treasury_balance_for_initial_validators(initial_authorities.len(), ENDOWMENT);

    // Treasury Account Id
    pub const TREASURY_PALLET_ID: PalletId = PalletId(*b"py/trsry");
    let treasury_account: AccountId = TREASURY_PALLET_ID.into_account();

    let mut inital_validators_endowment = initial_authorities
        .iter()
        .map(|k| (k.0.clone(), ENDOWMENT)).collect_vec();
    let mut endowed_accounts = vec![
        // Root key
        (root_key.clone(), ENDOWMENT),
        // Treasury Funds
        (treasury_account, treasury_funds),
    ];
    
    // Endow to validators
    endowed_accounts.append(&mut inital_validators_endowment);
  

    //assert_eq!(total_supply + ERC20_PQD_SUPPLY, 90_000_000_000 * PQD, "Total Supply Not equal to 20 million");

    GenesisConfig {
        system: SystemConfig {
            code: wasm_binary_unwrap().to_vec(),
            changes_trie_config: Default::default(),
        },
        balances: BalancesConfig {
            balances: endowed_accounts,
        },

        indices: IndicesConfig { indices: vec![] },
        session: SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(x.2.clone(),
                                     x.3.clone(),
                                     x.4.clone(),
                                     x.5.clone(),
                        ),
                    )
                })
                .collect::<Vec<_>>(),
        },
        staking: StakingConfig {
            minimum_validator_count: 1,
            validator_count: initial_authorities.len() as u32,
            invulnerables: initial_authorities
                .iter()
                .map(|x| x.0.clone()).collect(),
            stakers: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
                .collect(),
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        },
        elections: Default::default(),
        council: CouncilConfig { members: vec![], phantom: Default::default() },
        technical_committee: TechnicalCommitteeConfig {
            members: vec![],
            phantom: Default::default(),
        },
        sudo: SudoConfig {
            key: root_key.clone(),
        },
        babe: BabeConfig {
            authorities: Default::default(),
            epoch_config: Some(phuquocdog_runtime::BABE_GENESIS_EPOCH_CONFIG),
        },
        im_online: Default::default(),
        authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
        grandpa: Default::default(),
        technical_membership: Default::default(),
        treasury: Default::default()
       
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use sp_runtime::BuildStorage;

    use super::*;

    fn local_testnet_genesis_instant_single() -> GenesisConfig {
        testnet_genesis(
            vec![authority_keys_from_seed("Alice")],
            vec![],
            get_account_id_from_seed::<sr25519::Public>("Alice"),
        )
    }

    /// Local testnet config (single validator - Alice)
    pub fn integration_test_config_with_single_authority() -> ChainSpec {
        ChainSpec::from_genesis(
            "Integration Test",
            "test",
            ChainType::Development,
            local_testnet_genesis_instant_single,
            vec![],
            None,
            None,
            None,
            Default::default(),
        )
    }

    /// Local testnet config (multivalidator Alice + Bob)
    pub fn integration_test_config_with_two_authorities() -> ChainSpec {
        ChainSpec::from_genesis(
            "Integration Test",
            "test",
            ChainType::Development,
            soba_testnet_genesis,
            vec![],
            None,
            None,
            None,
            Default::default(),
        )
    }

    #[test]
    fn test_create_development_chain_spec() {
        assert!(!development_config().build_storage().is_err());
    }

    #[test]
    fn test_create_soba_testnet_chain_spec() {
        assert!(!soba_testnet_config().build_storage().is_err());
    }

    #[test]
    fn test_staging_test_net_chain_spec() {
        assert!(!quark_testnet_config().build_storage().is_err());
    }
}


pub fn phuquocdog_properties() -> Properties {
    let mut properties = Properties::new();
    //properties.insert("ss58Format".into(), 90.into());
    properties.insert("ss58Format".into(), 42.into());
    properties.insert("tokenDecimals".into(), 10.into());
    properties.insert("tokenSymbol".into(), "PQD".into());

    properties
}
