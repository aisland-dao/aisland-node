use aisland_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, Signature, SudoConfig,
	SystemConfig, WASM_BINARY,
};
use sc_service::{ChainType, Properties};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_core::crypto::UncheckedInto;
use hex_literal::hex;
// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;


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

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
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
			blockchain_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					hex!("dc31445d24993e946ebf9f444dd17a9698fe859eeb574b78910100baab083b75").into(),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Fork id
		None,
		// Properties
		Some(aisland_properties()),
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			blockchain_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
				authority_keys_from_seed("Alice"), 
				authority_keys_from_seed("Bob"),
				],
				// Sudo account
				hex!("001a667f2603ce2cb86703796aff2372c5a78ddeef0ff9b540d9ad745c254447").into(),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// ??
		None,
		// Properties
		Some(aisland_properties()),
		// Extensions
		None,
	))
}
pub fn public_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Aisland Testnet",
		// ID
		"aisland_testnet",
		ChainType::Live,
		move || {
			blockchain_genesis(
				wasm_binary,
				// Initial PoA authorities (Aura,Grandpa)
				vec![
					(
					 hex!("06c0cf3980869373aef29b4164425eb7371d76cde759dc8f5c68a11eeaf8f57e").unchecked_into(),
					 hex!("a7ec58dda00f7aeee12c776a29a15f6ea41328ab55e169cbaca7d971e3b2a87b").unchecked_into()
					),
										(
					 hex!("ea966d82672695cbd3a78a36e08cdc940f0b1085719ed5c5fe58a290fd4ba604").unchecked_into(),
					 hex!("99b3c75736a01b3657a6802771adf4d64c732c492b9035328a08dee0a87e55a7").unchecked_into()
					),
										(
					 hex!("00a62c03cd3d554dbf48a0cce9bef680f0ac1bc06d4dc8b02868f8d199d0cd63").unchecked_into(),
					 hex!("85fed304963bedb9232947f30ad39f5357e4842076ebb3275e2bfb72fec56767").unchecked_into()
					)
				],
				// Sudo account
				hex!("001a667f2603ce2cb86703796aff2372c5a78ddeef0ff9b540d9ad745c254447").into(),

				// Pre-funded accounts
				vec![
					hex!("9ad611a1a67fcf50f16be2650316d3ef976452bc32eaee8caca2da485bf40202").into(),
					hex!("1ec3154ebd4d0fc993c91c04b22f402d863f360fef026c80debf7fee4cdc7e68").into(),
				],
				true,
			)
		},
		// Bootnodes
		vec![
            "/ip4/65.108.62.72/tcp/30333/p2p/12D3KooWB8vQKy3di1vk9FmcXcCHdCrFbgrFNoUoy3hs94dBi6TU"
                .parse()
                .unwrap(),
            "/ip4/94.130.184.125/tcp/30333/p2p/12D3KooWGxSVjGui9huLCSkp8jh9Dbsr7sMFa9tJDjSCQDRTC51K"
                .parse()
                .unwrap(),
            "/ip4/94.130.183.49/tcp/30333/p2p/12D3KooWQQYoPEvpodMDNhxhSzDh4V7pDWr9fdYYdPb8N1BdBEXg"
                .parse()
                .unwrap(),
	        ],
		// Telemetry
		None,
		// Protocol ID
		None,
		// ??
		None,
		// Properties
		Some(aisland_properties()),
		// Extensions
		None,
	))
}
pub fn public_mainnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Aisland Mainnet",
		// ID
		"aisland_mainnet",
		ChainType::Live,
		move || {
			blockchain_genesis(
				wasm_binary,
				// Initial PoA authorities (Aura,Grandpa)
				vec![
					(
					 hex!("5ea8a8624c3a9a92ee670255e0581f165ad7296488cf7888469fe1df48e35a7c").unchecked_into(),
					 hex!("b2c7585164154e28eeb5de74bf4664487292cf0b3b55b2e3c467169ad0a585ff").unchecked_into()
					),
					(
					 hex!("749c4aa8de06d015cafc38b1abafef5348e54e3f1fe4cc4a37b229a4c0a2ae2e").unchecked_into(),
					 hex!("430838faae609280cf16274f6c17fd02c57643e7577213272add28c139d35ff4").unchecked_into()
					),
					(
					 hex!("524af842443c3780f203e6128e64a0b5b855596b510cc5c62cee15c47c02b06e").unchecked_into(),
					 hex!("cef0468a34ae89127711aef517569ca2273e6693e45492fb73692039b18c91b3").unchecked_into()
					),
					(
					 hex!("f6edf6f0b04e564fcebcd767ce57371100cfb17c488d17ca716c0b89592c6449").unchecked_into(),
					 hex!("cad1c490410e53560d30383d551ab4703b952490282ec3f05bd1446fb7b9cd68").unchecked_into()
					)
				],
				// Sudo account
				hex!("787cfae73abc1c72d7763528b484f36e644ae3d1b83059c98a24825f77735126").into(),

				// Pre-funded accounts
				vec![
					hex!("787cfae73abc1c72d7763528b484f36e644ae3d1b83059c98a24825f77735126").into(),
					hex!("96a4e3c699df5daf3941a232b81208e1c110fdc44d18fc47b492523520fe3826").into(),
					hex!("f2166f9e848c86e87d255e79a0d4f2ba2290c77855eb24203b5e232f1f16d53b").into(),
				],
				true,
			)
		},
		// Bootnodes
		vec![
            "/ip4/5.9.31.145/tcp/30333/p2p/12D3KooWFu3RN7pVgn7tG5FYR1nPQVw2qpLaL1BkzCF2oLP3PV8G"
                .parse()
                .unwrap(),
            "/ip4/135.181.213.73/tcp/30333/p2p/12D3KooWPbX3PwRcAcgmvA79dG9i2FeuKLDfwuLJ3cx8brbjgQa2"
                .parse()
                .unwrap(),
            "/ip4/79.143.186.172/tcp/30333/p2p/12D3KooWT2AASJbzJNKxoRYb7PnW6TAK7dsr42WkWDvN1WuDYZu8"
                .parse()
                .unwrap(),
            "/ip4/65.108.9.21/tcp/30333/p2p/12D3KooWDrJVWV3FLyXEFCUQYZyLwefKRbv8G46ZCAzkAx9Jm39D"
                .parse()
                .unwrap(),                
	        ],
		// Telemetry
		None,
		// Protocol ID
		None,
		// ??
		None,
		// Properties
		Some(aisland_properties()),
		// Extensions
		None,
	))
}


/// Configure initial storage state for FRAME modules.
fn blockchain_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
		
	}
}

/// Token
pub fn aisland_properties() -> Properties {
    let mut p = Properties::new();
    p.insert("ss58format".into(), 42.into());
    p.insert("tokenDecimals".into(), 18.into());
    p.insert("tokenSymbol".into(), "AISC".into());
    p
}
