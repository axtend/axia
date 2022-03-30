// Copyright 2017-2020 Axia Technologies (UK) Ltd.
// This file is part of Axia.

// Axia is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Axia is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Axia.  If not, see <http://www.gnu.org/licenses/>.

//! Axia chain configurations.

use beefy_primitives::crypto::AuthorityId as BeefyId;
use grandpa::AuthorityId as GrandpaId;
#[cfg(feature = "axctest-native")]
use axctest_runtime as axctest;
#[cfg(feature = "axctest-native")]
use axctest_runtime_constants::currency::UNITS as KSM;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_staking::Forcing;
use axia_primitives::v1::{AccountId, AccountPublic, AssignmentId, ValidatorId};
#[cfg(feature = "axia-native")]
use axia_runtime as axia;
#[cfg(feature = "axia-native")]
use axia_runtime_constants::currency::UNITS as AXC;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;

#[cfg(feature = "betanet-native")]
use betanet_runtime as betanet;
#[cfg(feature = "betanet-native")]
use betanet_runtime_constants::currency::UNITS as ROC;
use sc_chain_spec::{ChainSpecExtension, ChainType};
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::{traits::IdentifyAccount, Perbill};
use telemetry::TelemetryEndpoints;
#[cfg(feature = "alphanet-native")]
use alphanet_runtime as alphanet;
#[cfg(feature = "alphanet-native")]
use alphanet_runtime_constants::currency::UNITS as WND;

#[cfg(feature = "axia-native")]
const AXIA_STAGING_TELEMETRY_URL: &str = "ws://localhost:8001/submit/";
#[cfg(feature = "axctest-native")]
const AXIATEST_STAGING_TELEMETRY_URL: &str = "ws://localhost:8001/submit/";
#[cfg(feature = "alphanet-native")]
const ALPHANET_STAGING_TELEMETRY_URL: &str = "ws://localhost:8001/submit/";
#[cfg(feature = "betanet-native")]
const BETANET_STAGING_TELEMETRY_URL: &str = "ws://localhost:8001/submit/";
const DEFAULT_PROTOCOL_ID: &str = "axc";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<axia_primitives::v1::Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<axia_primitives::v1::Block>,
	/// The light sync state.
	///
	/// This value will be set by the `sync-state rpc` implementation.
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// The `ChainSpec` parameterized for the axia runtime.
#[cfg(feature = "axia-native")]
pub type AxiaChainSpec = service::GenericChainSpec<axia::GenesisConfig, Extensions>;

// Dummy chain spec, in case when we don't have the native runtime.
pub type DummyChainSpec = service::GenericChainSpec<(), Extensions>;

// Dummy chain spec, but that is fine when we don't have the native runtime.
#[cfg(not(feature = "axia-native"))]
pub type AxiaChainSpec = DummyChainSpec;

/// The `ChainSpec` parameterized for the axctest runtime.
#[cfg(feature = "axctest-native")]
pub type AxiaTestChainSpec = service::GenericChainSpec<axctest::GenesisConfig, Extensions>;

/// The `ChainSpec` parameterized for the axctest runtime.
// Dummy chain spec, but that is fine when we don't have the native runtime.
#[cfg(not(feature = "axctest-native"))]
pub type AxiaTestChainSpec = DummyChainSpec;

/// The `ChainSpec` parameterized for the alphanet runtime.
#[cfg(feature = "alphanet-native")]
pub type AlphanetChainSpec = service::GenericChainSpec<alphanet::GenesisConfig, Extensions>;

/// The `ChainSpec` parameterized for the alphanet runtime.
// Dummy chain spec, but that is fine when we don't have the native runtime.
#[cfg(not(feature = "alphanet-native"))]
pub type AlphanetChainSpec = DummyChainSpec;

/// The `ChainSpec` parameterized for the betanet runtime.
#[cfg(feature = "betanet-native")]
pub type BetanetChainSpec = service::GenericChainSpec<BetanetGenesisExt, Extensions>;

/// The `ChainSpec` parameterized for the `versi` runtime.
///
/// As of now `Versi` will just be a clone of `Betanet`, until we need it to differ.
pub type VersiChainSpec = BetanetChainSpec;

/// The `ChainSpec` parameterized for the betanet runtime.
// Dummy chain spec, but that is fine when we don't have the native runtime.
#[cfg(not(feature = "betanet-native"))]
pub type BetanetChainSpec = DummyChainSpec;

/// Extension for the Betanet genesis config to support a custom changes to the genesis state.
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg(feature = "betanet-native")]
pub struct BetanetGenesisExt {
	/// The runtime genesis config.
	runtime_genesis_config: betanet::GenesisConfig,
	/// The session length in blocks.
	///
	/// If `None` is supplied, the default value is used.
	session_length_in_blocks: Option<u32>,
}

#[cfg(feature = "betanet-native")]
impl sp_runtime::BuildStorage for BetanetGenesisExt {
	fn assimilate_storage(&self, storage: &mut sp_core::storage::Storage) -> Result<(), String> {
		sp_state_machine::BasicExternalities::execute_with_storage(storage, || {
			if let Some(length) = self.session_length_in_blocks.as_ref() {
				betanet_runtime_constants::time::EpochDurationInBlocks::set(length);
			}
		});
		self.runtime_genesis_config.assimilate_storage(storage)
	}
}

pub fn axia_config() -> Result<AxiaChainSpec, String> {
	AxiaChainSpec::from_json_bytes(&include_bytes!("../res/axia.json")[..])
}

pub fn axctest_config() -> Result<AxiaTestChainSpec, String> {
	AxiaTestChainSpec::from_json_bytes(&include_bytes!("../res/axctest.json")[..])
}

pub fn alphanet_config() -> Result<AlphanetChainSpec, String> {
	AlphanetChainSpec::from_json_bytes(&include_bytes!("../res/alphanet.json")[..])
}

pub fn betanet_config() -> Result<BetanetChainSpec, String> {
	BetanetChainSpec::from_json_bytes(&include_bytes!("../res/betanet.json")[..])
}

pub fn versi_config() -> Result<VersiChainSpec, String> {
	VersiChainSpec::from_json_bytes(&include_bytes!("../res/versi.json")[..])
}

/// This is a temporary testnet that uses the same runtime as betanet.
pub fn wococo_config() -> Result<BetanetChainSpec, String> {
	BetanetChainSpec::from_json_bytes(&include_bytes!("../res/wococo.json")[..])
}

/// The default allychains host configuration.
#[cfg(any(
	feature = "betanet-native",
	feature = "axctest-native",
	feature = "alphanet-native",
	feature = "axia-native"
))]
fn default_allychains_host_configuration(
) -> axia_runtime_allychains::configuration::HostConfiguration<
	axia_primitives::v1::BlockNumber,
> {
	use axia_primitives::v1::{MAX_CODE_SIZE, MAX_POV_SIZE};

	axia_runtime_allychains::configuration::HostConfiguration {
		validation_upgrade_cooldown: 2u32,
		validation_upgrade_delay: 2,
		code_retention_period: 1200,
		max_code_size: MAX_CODE_SIZE,
		max_pov_size: MAX_POV_SIZE,
		max_head_data_size: 32 * 1024,
		group_rotation_frequency: 20,
		chain_availability_period: 4,
		thread_availability_period: 4,
		max_upward_queue_count: 8,
		max_upward_queue_size: 1024 * 1024,
		max_downward_message_size: 1024 * 1024,
		ump_service_total_weight: 100_000_000_000,
		max_upward_message_size: 1024 * 1024,
		max_upward_message_num_per_candidate: 5,
		hrmp_sender_deposit: 0,
		hrmp_recipient_deposit: 0,
		hrmp_channel_max_capacity: 8,
		hrmp_channel_max_total_size: 8 * 1024,
		hrmp_max_allychain_inbound_channels: 4,
		hrmp_max_parathread_inbound_channels: 4,
		hrmp_channel_max_message_size: 1024 * 1024,
		hrmp_max_allychain_outbound_channels: 4,
		hrmp_max_parathread_outbound_channels: 4,
		hrmp_max_message_num_per_candidate: 5,
		dispute_period: 6,
		no_show_slots: 2,
		n_delay_tranches: 25,
		needed_approvals: 2,
		relay_vrf_modulo_samples: 2,
		zeroth_delay_tranche_width: 0,
		minimum_validation_upgrade_delay: 5,
		..Default::default()
	}
}

#[cfg(any(
	feature = "betanet-native",
	feature = "axctest-native",
	feature = "alphanet-native",
	feature = "axia-native"
))]
#[test]
fn default_allychains_host_configuration_is_consistent() {
	default_allychains_host_configuration().panic_if_not_consistent();
}

#[cfg(feature = "axia-native")]
fn axia_session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	para_validator: ValidatorId,
	para_assignment: AssignmentId,
	authority_discovery: AuthorityDiscoveryId,
) -> axia::SessionKeys {
	axia::SessionKeys {
		babe,
		grandpa,
		im_online,
		para_validator,
		para_assignment,
		authority_discovery,
	}
}

#[cfg(feature = "axctest-native")]
fn axctest_session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	para_validator: ValidatorId,
	para_assignment: AssignmentId,
	authority_discovery: AuthorityDiscoveryId,
) -> axctest::SessionKeys {
	axctest::SessionKeys {
		babe,
		grandpa,
		im_online,
		para_validator,
		para_assignment,
		authority_discovery,
	}
}

#[cfg(feature = "alphanet-native")]
fn alphanet_session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	para_validator: ValidatorId,
	para_assignment: AssignmentId,
	authority_discovery: AuthorityDiscoveryId,
) -> alphanet::SessionKeys {
	alphanet::SessionKeys {
		babe,
		grandpa,
		im_online,
		para_validator,
		para_assignment,
		authority_discovery,
	}
}

#[cfg(feature = "betanet-native")]
fn betanet_session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	para_validator: ValidatorId,
	para_assignment: AssignmentId,
	authority_discovery: AuthorityDiscoveryId,
	beefy: BeefyId,
) -> betanet_runtime::SessionKeys {
	betanet_runtime::SessionKeys {
		babe,
		grandpa,
		im_online,
		para_validator,
		para_assignment,
		authority_discovery,
		beefy,
	}
}

#[cfg(feature = "axia-native")]
fn axia_staging_testnet_config_genesis(wasm_binary: &[u8]) -> axia::GenesisConfig {
	use hex_literal::hex;
	use sp_core::crypto::UncheckedInto;

	// subkey inspect "$SECRET"
	let endowed_accounts = vec![
		hex!["a2bf32e50edd79c181888da41c80c67c191e9e6b29d3f2efb102ca0e2b53c558"].into()
	];

	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)> = vec![
		(
			// Sankar//stash
			hex!["3233f745d0860ed64ae9c7f4ea5c0773316fc9265199f312d3f6e8ce08255c10"].into(),
			// Sankar
			hex!["3edc55e451a46f7d2ec513fb40b3687b9a03fc32e16274d37f332205d6413945"].into(),
			// Sankar//babe
			hex!["b609973a7b7e1468afda62babf32844e387d31dbff7e046412ae8a18f3452971"].unchecked_into(),
			// Sankar//grandpa
			hex!["161a16204ef752deeb0002578456e77df6caa89ec463a1bf471533e93369e2b8"].unchecked_into(),
			// Sankar//im_online
			hex!["9a5d57e1433b8f667851beff9be7ed25f1d32e6f3d4c03fa8da21506e20a3c6c"].unchecked_into(),
			// Sankar//para_validator
			hex!["368f272ba94824fb2589d01074bcaf483ced022734b299f49ac66a61c2bb961b"].unchecked_into(),
			// Sankar//para_assignment
			hex!["6221352ed419451d86fda2481de68986bececd16fe59c274ec4de50ee5bf3810"].unchecked_into(),
			// Sankar//authority_discovery
			hex!["76e6cdd1f98d574da88af6920dc870ab6a356151d6a60d4e0b4bcbe0b4578c16"].unchecked_into(),
		),
		(
			// Arun//stash
			hex!["445f574d57f768ea7e1a2f551bef4298ace99d8895d316352cfc02aececcf26c"].into(),
			// Arun
			hex!["d05d1412507f428a1f426e9358eaadb73f4604e9abadf3a98c26af0422be2e17"].into(),
			// Arun//babe
			hex!["54097885aae3ab47c7eec090b7dcb672778f3362f41d318ecc10c889ec0b2652"].unchecked_into(),
			// Arun//grandpa
			hex!["14aa4f9c765360d5d408f1fd6563612bd30a9f50d1b3f37ac096d0a4671bcad7"].unchecked_into(),
			// Arun//im_online
			hex!["2a8acf53b52d4fd54aad5b05afd5e87cbbfffd42ed29459639ff91de03a4c167"].unchecked_into(),
			// Arun//para_validator
			hex!["7c4ecd0a349aa899c8fa35a7bba8225ddc0ce6922a4a480c00b2a90ef9abef07"].unchecked_into(),
			// Arun//para_assignment
			hex!["7293fda5944b0045d6e50743835baf0329275580ee2837a94a7824e20bb7d77a"].unchecked_into(),
			// Arun//authority_discovery
			hex!["f68863365282f034e86650515dd9ee1962e7f8c534287121aeaa203ee213cf18"].unchecked_into(),
		),
		(
			// Rakhi//stash
			hex!["c2312f7f9a8190bf76db9dc40e5ef351c4c23e3ae6540932bf2c2d485289c37b"].into(),
			// Rakhi
			hex!["82c14ac892565d92d7a7f4e38dddf7edbe07295d08a6d3e65fa19585a43bdf0c"].into(),
			// Rakhi//babe
			hex!["ba8331b4dd6073bd0de45fcbdf6142b402ae7b1e02335b7318e5856a0e3dcd77"].unchecked_into(),
			// Rakhi//grandpa
			hex!["b34908f44b24b3052d2324a06d12a564350083b03441bc204c8acd8642b7c844"].unchecked_into(),
			// Rakhi//im_online
			hex!["14c233dfa06d9153ab014074391c659bfca62c730655409b51209f66587f2467"].unchecked_into(),
			// Rakhi//para_validator
			hex!["4ac7bdfc1b8b8c3d2e7f4c4bd8372077f9e9edcc95368a45d5607b7d75411c0a"].unchecked_into(),
			// Rakhi//para_assignment
			hex!["02167bdafcbe516376ef3c24086ccb2fc06dc80fd05e553dbd53cc7b75b70335"].unchecked_into(),
			// Rakhi//authority_discovery
			hex!["183c3a5c383f78805c5f6b60301939b536be4c3254c8ea3012ecb81d5e290279"].unchecked_into(),
		),
		(
			// Priya//stash
			hex!["688d6fa54d9ace0fa07492f3d8dfef78594130719e61c213d700c62421177c38"].into(),
			// Priya
			hex!["b04155fab96288008150d78409961051d6e4e7d2b5d4bc7c51f6cf55699aa161"].into(),
			// Priya//babe
			hex!["74477e67460cab9167678aebbf746cb9a2a9d33148ae1f6bf12f7f83c179c75c"].unchecked_into(),
			// Priya//grandpa
			hex!["687e40323706755ab58d6a1b31aabedde5fbde9f682277df587af1b4e8616847"].unchecked_into(),
			// Priya//im_online
			hex!["4a37d8530ba98c21b90580e16a2d56b7e931692067ef096d990700c59ecd515f"].unchecked_into(),
			// Priya//para_validator
			hex!["2c7aa0bf1337b32c2edf66388c928cc07731efcca87160b2c5c213765b44255e"].unchecked_into(),
			// Priya//para_assignment
			hex!["ea610dcfc75cb6a33937dbbc50c0722a6e7992e5cec70096c086bc1e6e4ef83b"].unchecked_into(),
			// Priya//authority_discovery
			hex!["0c1ecee6c2199e514f838a5fe93ddff8722423d20aa190a3563b7bff9100fd17"].unchecked_into(),
		)
	];

	const ENDOWMENT: u128 = 1_000_000_000 * AXC;
	const STASH: u128 = 1_000_000_000 * AXC;

	axia::GenesisConfig {
		system: axia::SystemConfig { code: wasm_binary.to_vec() },
		balances: axia::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k: &AccountId| (k.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
		},
		indices: axia::IndicesConfig { indices: vec![] },
		session: axia::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						axia_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: axia::StakingConfig {
			validator_count: 50,
			minimum_validator_count: 4,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, axia::StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::ForceNone,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		phragmen_election: Default::default(),
		democracy: Default::default(),
		council: axia::CouncilConfig { members: vec![], phantom: Default::default() },
		technical_committee: axia::TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		},
		technical_membership: Default::default(),
		babe: axia::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(axia::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		authority_discovery: axia::AuthorityDiscoveryConfig { keys: vec![] },
		claims: axia::ClaimsConfig { claims: vec![], vesting: vec![] },
		vesting: axia::VestingConfig { vesting: vec![] },
		treasury: Default::default(),
		hrmp: Default::default(),
		configuration: axia::ConfigurationConfig {
			config: default_allychains_host_configuration(),
		},
		paras: Default::default(),
		xcm_pallet: Default::default(),
		sudo: axia::SudoConfig { key: Some(endowed_accounts[0].clone()) },
	}
}

#[cfg(feature = "alphanet-native")]
fn alphanet_staging_testnet_config_genesis(wasm_binary: &[u8]) -> alphanet::GenesisConfig {
	use hex_literal::hex;
	use sp_core::crypto::UncheckedInto;

	// subkey inspect "$SECRET"
	let endowed_accounts = vec![
		// 5DaVh5WRfazkGaKhx1jUu6hjz7EmRe4dtW6PKeVLim84KLe8
		hex!["42f4a4b3e0a89c835ee696205caa90dd85c8ea1d7364b646328ee919a6b2fc1e"].into(),
	];
	// SECRET='...' ./scripts/prepare-test-net.sh 4
	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)> = vec![
		(
			//5ERCqy118nnXDai8g4t3MjdX7ZC5PrQzQpe9vwex5cELWqbt
			hex!["681af4f93073484e1acd6b27395d0d258f1a6b158c808846c8fd05ee2435056e"].into(),
			//5GTS114cfQNBgpQULhMaNCPXGds6NokegCnikxDe1vqANhtn
			hex!["c2463372598ebabd21ee5bc33e1d7e77f391d2df29ce2fbe6bed0d13be629a45"].into(),
			//5FhGbceKeH7fuGogcBwd28ZCkAwDGYBADCTeHiYrvx2ztyRd
			hex!["a097bfc6a33499ed843b711f52f523f8a7174f798a9f98620e52f4170dbe2948"]
				.unchecked_into(),
			//5Es7nDkJt2by5qVCCD7PZJdp76KJw1LdRCiNst5S5f4eecnz
			hex!["7bde49dda82c2c9f082b807ef3ceebff96437d67b3e630c584db7a220ecafacf"]
				.unchecked_into(),
			//5D4e8zRjaYzFamqChGPPtu26PcKbKgUrhb7WqcNbKa2RDFUR
			hex!["2c2fb730a7d9138e6d62fcf516f9ecc2d712af3f2f03ca330c9564b8c0c1bb33"]
				.unchecked_into(),
			//5DD3JY5ENkjcgVFbVSgUbZv7WmrnyJ8bxxu56ee6hZFiRdnh
			hex!["3297a8622988cc23dd9c131e3fb8746d49e007f6e58a81d43420cd539e250e4c"]
				.unchecked_into(),
			//5Gpodowhud8FG9xENXR5YwTFbUAWyoEtw7sYFytFsG4z7SU6
			hex!["d2932edf775088bd088dc5a112ad867c24cc95858f77f8a1ab014de8d4f96a3f"]
				.unchecked_into(),
			//5GUMj8tnjL3PJZgXoiWtgLCaMVNHBNeSeTqDsvcxmaVAjKn9
			hex!["c2fb0f74591a00555a292bc4882d3158bafc4c632124cb60681f164ef81bcf72"]
				.unchecked_into(),
		),
		(
			//5HgDCznTkHKUjzPkQoTZGWbvbyqB7sqHDBPDKdF1FyVYM7Er
			hex!["f8418f189f84814fd40cc1b2e90873e72ea789487f3b98ed42811ba76d10fc37"].into(),
			//5GQTryeFwuvgmZ2tH5ZeAKZHRM9ch5WGVGo6ND9P8f9uMsNY
			hex!["c002bb4af4a1bd2f33d104aef8a41878fe1ac94ba007029c4dfdefa8b698d043"].into(),
			//5C7YkWSVH1zrpsE5KwW1ua1qatyphzYxiZrL24mjkxz7mUbn
			hex!["022b14fbcf65a93b81f453105b9892c3fc4aa74c22c53b4abab019e1d58fbd41"]
				.unchecked_into(),
			//5GwFC6Tmg4fhj4PxSqHycgJxi3PDfnC9RGDsNHoRwAvXvpnZ
			hex!["d77cafd3b32c8b52b0e2780a586a6e527c94f1bdec117c4e4acb0a491461ffa3"]
				.unchecked_into(),
			//5DSVrGURuDuh8Luzo8FYq7o2NWiUSLSN6QAVNrj9BtswWH6R
			hex!["3cdb36a5a14715999faffd06c5b9e5dcdc24d4b46bc3e4df1aaad266112a7b49"]
				.unchecked_into(),
			//5DLEG2AupawCXGwhJtrzBRc3zAhuP8V662dDrUTzAsCiB9Ec
			hex!["38134245c9919ecb20bf2eedbe943b69ba92ceb9eb5477b92b0afd3cb6ce2858"]
				.unchecked_into(),
			//5D83o9fDgnHxaKPkSx59hk8zYzqcgzN2mrf7cp8fiVEi7V4E
			hex!["2ec917690dc1d676002e3504c530b2595490aa5a4603d9cc579b9485b8d0d854"]
				.unchecked_into(),
			//5DwBJquZgncRWXFxj2ydbF8LBUPPUbiq86sXWXgm8Z38m8L2
			hex!["52bae9b8dedb8058dda93ec6f57d7e5a517c4c9f002a4636fada70fed0acf376"]
				.unchecked_into(),
		),
		(
			//5DMHpkRpQV7NWJFfn2zQxCLiAKv7R12PWFRPHKKk5X3JkYfP
			hex!["38e280b35d08db46019a210a944e4b7177665232ab679df12d6a8bbb317a2276"].into(),
			//5FbJpSHmFDe5FN3DVGe1R345ZePL9nhcC9V2Cczxo7q8q6rN
			hex!["9c0bc0e2469924d718ae683737f818a47c46b0612376ecca06a2ac059fe1f870"].into(),
			//5E5Pm3Udzxy26KGkLE5pc8JPfQrvkYHiaXWtuEfmQsBSgep9
			hex!["58fecadc2df8182a27e999e7e1fd7c99f8ec18f2a81f9a0db38b3653613f3f4d"]
				.unchecked_into(),
			//5FxcystSLHtaWoy2HEgRNerj9PrUs452B6AvHVnQZm5ZQmqE
			hex!["ac4d0c5e8f8486de05135c10a707f58aa29126d5eb28fdaaba00f9a505f5249d"]
				.unchecked_into(),
			//5E7KqVXaVGuAqiqMigpuH8oXHLVh4tmijmpJABLYANpjMkem
			hex!["5a781385a0235fe8594dd101ec55ef9ba01883f8563a0cdd37b89e0303f6a578"]
				.unchecked_into(),
			//5H9AybjkpyZ79yN5nHuBqs6RKuZPgM7aAVVvTQsDFovgXb2A
			hex!["e09570f62a062450d4406b4eb43e7f775ff954e37606646cd590d1818189501f"]
				.unchecked_into(),
			//5Ccgs7VwJKBawMbwMENDmj2eFAxhFdGksVHdk8aTAf4w7xox
			hex!["1864832dae34df30846d5cc65973f58a2d01b337d094b1284ec3466ecc90251d"]
				.unchecked_into(),
			//5EsSaZZ7niJs7hmAtp4QeK19AcAuTp7WXB7N7gRipVooerq4
			hex!["7c1d92535e6d94e21cffea6633a855a7e3c9684cd2f209e5ddbdeaf5111e395b"]
				.unchecked_into(),
		),
		(
			//5Ea11qhmGRntQ7pyEkEydbwxvfrYwGMKW6rPERU4UiSBB6rd
			hex!["6ed057d2c833c45629de2f14b9f6ce6df1edbf9421b7a638e1fb4828c2bd2651"].into(),
			//5CZomCZwPB78BZMZsCiy7WSpkpHhdrN8QTSyjcK3FFEZHBor
			hex!["1631ff446b3534d031adfc37b7f7aed26d2a6b3938d10496aab3345c54707429"].into(),
			//5CSM6vppouFHzAVPkVFWN76DPRUG7B9qwJe892ccfSfJ8M5f
			hex!["108188c43a7521e1abe737b343341c2179a3a89626c7b017c09a5b10df6f1c42"]
				.unchecked_into(),
			//5GwkG4std9KcjYi3ThSC7QWfhqokmYVvWEqTU9h7iswjhLnr
			hex!["d7de8a43f7ee49fa3b3aaf32fb12617ec9ff7b246a46ab14e9c9d259261117fa"]
				.unchecked_into(),
			//5CoUk3wrCGJAWbiJEcsVjYhnd2JAHvR59jBRbSw77YrBtRL1
			hex!["209f680bc501f9b59358efe3636c51fd61238a8659bac146db909aea2595284b"]
				.unchecked_into(),
			//5EcSu96wprFM7G2HfJTjYu8kMParnYGznSUNTsoEKXywEsgG
			hex!["70adf80395b3f59e4cab5d9da66d5a286a0b6e138652a06f72542e46912df922"]
				.unchecked_into(),
			//5Ge3sjpD43Cuy7rNoJQmE9WctgCn6Faw89Pe7xPs3i55eHwJ
			hex!["ca5f6b970b373b303f64801a0c2cadc4fc05272c6047a2560a27d0c65589ca1d"]
				.unchecked_into(),
			//5EFcjHLvB2z5vd5g63n4gABmhzP5iPsKvTwd8sjfvTehNNrk
			hex!["60cae7fa5a079d9fc8061d715fbcc35ef57c3b00005694c2badce22dcc5a9f1b"]
				.unchecked_into(),
		),
	];

	const ENDOWMENT: u128 = 1_000_000 * WND;
	const STASH: u128 = 100 * WND;

	alphanet::GenesisConfig {
		system: alphanet::SystemConfig { code: wasm_binary.to_vec() },
		balances: alphanet::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k: &AccountId| (k.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
		},
		indices: alphanet::IndicesConfig { indices: vec![] },
		session: alphanet::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						alphanet_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: alphanet::StakingConfig {
			validator_count: 50,
			minimum_validator_count: 4,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, alphanet::StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::ForceNone,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		babe: alphanet::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(alphanet::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		authority_discovery: alphanet::AuthorityDiscoveryConfig { keys: vec![] },
		vesting: alphanet::VestingConfig { vesting: vec![] },
		sudo: alphanet::SudoConfig { key: Some(endowed_accounts[0].clone()) },
		hrmp: Default::default(),
		configuration: alphanet::ConfigurationConfig {
			config: default_allychains_host_configuration(),
		},
		paras: Default::default(),
		registrar: alphanet_runtime::RegistrarConfig {
			next_free_para_id: axia_primitives::v1::LOWEST_PUBLIC_ID,
		},
		xcm_pallet: Default::default(),
	}
}

#[cfg(feature = "axctest-native")]
fn axctest_staging_testnet_config_genesis(wasm_binary: &[u8]) -> axctest::GenesisConfig {
	use hex_literal::hex;
	use sp_core::crypto::UncheckedInto;

	// subkey inspect "$SECRET"
	let endowed_accounts = vec![
		// 5CVFESwfkk7NmhQ6FwHCM9roBvr9BGa4vJHFYU8DnGQxrXvz
		hex!["12b782529c22032ed4694e0f6e7d486be7daa6d12088f6bc74d593b3900b8438"].into(),
	];

	// for i in 1 2 3 4; do for j in stash controller; do subkey inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in babe; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in grandpa; do subkey --ed25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in im_online; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in para_validator para_assignment; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)> = vec![
		(
			// 5DD7Q4VEfPTLEdn11CnThoHT5f9xKCrnofWJL5SsvpTghaAT
			hex!["32a5718e87d16071756d4b1370c411bbbb947eb62f0e6e0b937d5cbfc0ea633b"].into(),
			// 5GNzaEqhrZAtUQhbMe2gn9jBuNWfamWFZHULryFwBUXyd1cG
			hex!["bee39fe862c85c91aaf343e130d30b643c6ea0b4406a980206f1df8331f7093b"].into(),
			// 5FpewyS2VY8Cj3tKgSckq8ECkjd1HKHvBRnWhiHqRQsWfFC1
			hex!["a639b507ee1585e0b6498ff141d6153960794523226866d1b44eba3f25f36356"]
				.unchecked_into(),
			// 5EjvdwATjyFFikdZibVvx1q5uBHhphS2Mnsq5c7yfaYK25vm
			hex!["76620f7c98bce8619979c2b58cf2b0aff71824126d2b039358729dad993223db"]
				.unchecked_into(),
			// 5FpewyS2VY8Cj3tKgSckq8ECkjd1HKHvBRnWhiHqRQsWfFC1
			hex!["a639b507ee1585e0b6498ff141d6153960794523226866d1b44eba3f25f36356"]
				.unchecked_into(),
			// 5FpewyS2VY8Cj3tKgSckq8ECkjd1HKHvBRnWhiHqRQsWfFC1
			hex!["a639b507ee1585e0b6498ff141d6153960794523226866d1b44eba3f25f36356"]
				.unchecked_into(),
			// 5FpewyS2VY8Cj3tKgSckq8ECkjd1HKHvBRnWhiHqRQsWfFC1
			hex!["a639b507ee1585e0b6498ff141d6153960794523226866d1b44eba3f25f36356"]
				.unchecked_into(),
			// 5FpewyS2VY8Cj3tKgSckq8ECkjd1HKHvBRnWhiHqRQsWfFC1
			hex!["a639b507ee1585e0b6498ff141d6153960794523226866d1b44eba3f25f36356"]
				.unchecked_into(),
		),
		(
			// 5G9VGb8ESBeS8Ca4or43RfhShzk9y7T5iTmxHk5RJsjZwsRx
			hex!["b496c98a405ceab59b9e970e59ef61acd7765a19b704e02ab06c1cdfe171e40f"].into(),
			// 5F7V9Y5FcxKXe1aroqvPeRiUmmeQwTFcL3u9rrPXcMuMiCNx
			hex!["86d3a7571dd60139d297e55d8238d0c977b2e208c5af088f7f0136b565b0c103"].into(),
			// 5GvuM53k1Z4nAB5zXJFgkRSHv4Bqo4BsvgbQWNWkiWZTMwWY
			hex!["765e46067adac4d1fe6c783aa2070dfa64a19f84376659e12705d1734b3eae01"]
				.unchecked_into(),
			// 5HBDAaybNqjmY7ww8ZcZZY1L5LHxvpnyfqJwoB7HhR6raTmG
			hex!["e2234d661bee4a04c38392c75d1566200aa9e6ae44dd98ee8765e4cc9af63cb7"]
				.unchecked_into(),
			// 5GvuM53k1Z4nAB5zXJFgkRSHv4Bqo4BsvgbQWNWkiWZTMwWY
			hex!["765e46067adac4d1fe6c783aa2070dfa64a19f84376659e12705d1734b3eae01"]
				.unchecked_into(),
			// 5GvuM53k1Z4nAB5zXJFgkRSHv4Bqo4BsvgbQWNWkiWZTMwWY
			hex!["765e46067adac4d1fe6c783aa2070dfa64a19f84376659e12705d1734b3eae01"]
				.unchecked_into(),
			// 5GvuM53k1Z4nAB5zXJFgkRSHv4Bqo4BsvgbQWNWkiWZTMwWY
			hex!["765e46067adac4d1fe6c783aa2070dfa64a19f84376659e12705d1734b3eae01"]
				.unchecked_into(),
			// 5GvuM53k1Z4nAB5zXJFgkRSHv4Bqo4BsvgbQWNWkiWZTMwWY
			hex!["765e46067adac4d1fe6c783aa2070dfa64a19f84376659e12705d1734b3eae01"]
				.unchecked_into(),
		),
		(
			// 5FzwpgGvk2kk9agow6KsywLYcPzjYc8suKej2bne5G5b9YU3
			hex!["ae12f70078a22882bf5135d134468f77301927aa67c376e8c55b7ff127ace115"].into(),
			// 5EqoZhVC2BcsM4WjvZNidu2muKAbu5THQTBKe3EjvxXkdP7A
			hex!["7addb914ec8486bbc60643d2647685dcc06373401fa80e09813b630c5831d54b"].into(),
			// 5CXNq1mSKJT4Sc2CbyBBdANeSkbUvdWvE4czJjKXfBHi9sX5
			hex!["664eae1ca4713dd6abf8c15e6c041820cda3c60df97dc476c2cbf7cb82cb2d2e"]
				.unchecked_into(),
			// 5E8ULLQrDAtWhfnVfZmX41Yux86zNAwVJYguWJZVWrJvdhBe
			hex!["5b57ed1443c8967f461db1f6eb2ada24794d163a668f1cf9d9ce3235dfad8799"]
				.unchecked_into(),
			// 5CXNq1mSKJT4Sc2CbyBBdANeSkbUvdWvE4czJjKXfBHi9sX5
			hex!["664eae1ca4713dd6abf8c15e6c041820cda3c60df97dc476c2cbf7cb82cb2d2e"]
				.unchecked_into(),
			// 5CXNq1mSKJT4Sc2CbyBBdANeSkbUvdWvE4czJjKXfBHi9sX5
			hex!["664eae1ca4713dd6abf8c15e6c041820cda3c60df97dc476c2cbf7cb82cb2d2e"]
				.unchecked_into(),
			// 5CXNq1mSKJT4Sc2CbyBBdANeSkbUvdWvE4czJjKXfBHi9sX5
			hex!["664eae1ca4713dd6abf8c15e6c041820cda3c60df97dc476c2cbf7cb82cb2d2e"]
				.unchecked_into(),
			// 5CXNq1mSKJT4Sc2CbyBBdANeSkbUvdWvE4czJjKXfBHi9sX5
			hex!["664eae1ca4713dd6abf8c15e6c041820cda3c60df97dc476c2cbf7cb82cb2d2e"]
				.unchecked_into(),
		),
		(
			// 5CFj6Kg9rmVn1vrqpyjau2ztyBzKeVdRKwNPiA3tqhB5HPqq
			hex!["0867dbb49721126df589db100dda728dc3b475cbf414dad8f72a1d5e84897252"].into(),
			// 5CwQXP6nvWzigFqNhh2jvCaW9zWVzkdveCJY3tz2MhXMjTon
			hex!["26ab2b4b2eba2263b1e55ceb48f687bb0018130a88df0712fbdaf6a347d50e2a"].into(),
			// 5FCd9Y7RLNyxz5wnCAErfsLbXGG34L2BaZRHzhiJcMUMd5zd
			hex!["2adb17a5cafbddc7c3e00ec45b6951a8b12ce2264235b4def342513a767e5d3d"]
				.unchecked_into(),
			// 5HGLmrZsiTFTPp3QoS1W8w9NxByt8PVq79reqvdxNcQkByqK
			hex!["e60d23f49e93c1c1f2d7c115957df5bbd7faf5ebf138d1e9d02e8b39a1f63df0"]
				.unchecked_into(),
			// 5FCd9Y7RLNyxz5wnCAErfsLbXGG34L2BaZRHzhiJcMUMd5zd
			hex!["2adb17a5cafbddc7c3e00ec45b6951a8b12ce2264235b4def342513a767e5d3d"]
				.unchecked_into(),
			// 5FCd9Y7RLNyxz5wnCAErfsLbXGG34L2BaZRHzhiJcMUMd5zd
			hex!["2adb17a5cafbddc7c3e00ec45b6951a8b12ce2264235b4def342513a767e5d3d"]
				.unchecked_into(),
			// 5FCd9Y7RLNyxz5wnCAErfsLbXGG34L2BaZRHzhiJcMUMd5zd
			hex!["2adb17a5cafbddc7c3e00ec45b6951a8b12ce2264235b4def342513a767e5d3d"]
				.unchecked_into(),
			// 5FCd9Y7RLNyxz5wnCAErfsLbXGG34L2BaZRHzhiJcMUMd5zd
			hex!["2adb17a5cafbddc7c3e00ec45b6951a8b12ce2264235b4def342513a767e5d3d"]
				.unchecked_into(),
		),
	];

	const ENDOWMENT: u128 = 1_000_000 * KSM;
	const STASH: u128 = 100 * KSM;

	axctest::GenesisConfig {
		system: axctest::SystemConfig { code: wasm_binary.to_vec() },
		balances: axctest::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k: &AccountId| (k.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
		},
		indices: axctest::IndicesConfig { indices: vec![] },
		session: axctest::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						axctest_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: axctest::StakingConfig {
			validator_count: 50,
			minimum_validator_count: 4,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, axctest::StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::ForceNone,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		phragmen_election: Default::default(),
		democracy: Default::default(),
		council: axctest::CouncilConfig { members: vec![], phantom: Default::default() },
		technical_committee: axctest::TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		},
		technical_membership: Default::default(),
		babe: axctest::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(axctest::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		authority_discovery: axctest::AuthorityDiscoveryConfig { keys: vec![] },
		claims: axctest::ClaimsConfig { claims: vec![], vesting: vec![] },
		vesting: axctest::VestingConfig { vesting: vec![] },
		treasury: Default::default(),
		hrmp: Default::default(),
		configuration: axctest::ConfigurationConfig {
			config: default_allychains_host_configuration(),
		},
		gilt: Default::default(),
		paras: Default::default(),
		xcm_pallet: Default::default(),
	}
}

#[cfg(feature = "betanet-native")]
fn betanet_staging_testnet_config_genesis(wasm_binary: &[u8]) -> betanet_runtime::GenesisConfig {
	use hex_literal::hex;
	use sp_core::crypto::UncheckedInto;

	// subkey inspect "$SECRET"
	let endowed_accounts = vec![
		// 5FeyRQmjtdHoPH56ASFW76AJEP1yaQC1K9aEMvJTF9nzt9S9
		hex!["9ed7705e3c7da027ba0583a22a3212042f7e715d3c168ba14f1424e2bc111d00"].into(),
	];

	// ./scripts/prepare-test-net.sh 8
	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
		BeefyId,
	)> = vec![
		(
			//5EHZkbp22djdbuMFH9qt1DVzSCvqi3zWpj6DAYfANa828oei
			hex!["62475fe5406a7cb6a64c51d0af9d3ab5c2151bcae982fb812f7a76b706914d6a"].into(),
			//5FeSEpi9UYYaWwXXb3tV88qtZkmSdB3mvgj3pXkxKyYLGhcd
			hex!["9e6e781a76810fe93187af44c79272c290c2b9e2b8b92ee11466cd79d8023f50"].into(),
			//5Fh6rDpMDhM363o1Z3Y9twtaCPfizGQWCi55BSykTQjGbP7H
			hex!["a076ef1280d768051f21d060623da3ab5b56944d681d303ed2d4bf658c5bed35"]
				.unchecked_into(),
			//5CPd3zoV9Aaah4xWucuDivMHJ2nEEmpdi864nPTiyRZp4t87
			hex!["0e6d7d1afbcc6547b92995a394ba0daed07a2420be08220a5a1336c6731f0bfa"]
				.unchecked_into(),
			//5F7BEa1LGFksUihyatf3dCDYneB8pWzVyavnByCsm5nBgezi
			hex!["86975a37211f8704e947a365b720f7a3e2757988eaa7d0f197e83dba355ef743"]
				.unchecked_into(),
			//5CP6oGfwqbEfML8efqm1tCZsUgRsJztp9L8ZkEUxA16W8PPz
			hex!["0e07a51d3213842f8e9363ce8e444255990a225f87e80a3d651db7841e1a0205"]
				.unchecked_into(),
			//5HQdwiDh8Qtd5dSNWajNYpwDvoyNWWA16Y43aEkCNactFc2b
			hex!["ec60e71fe4a567ef9fef99d4bbf37ffae70564b41aa6f94ef0317c13e0a5477b"]
				.unchecked_into(),
			//5HbSgM72xVuscsopsdeG3sCSCYdAeM1Tay9p79N6ky6vwDGq
			hex!["f49eae66a0ac9f610316906ec8f1a0928e20d7059d76a5ca53cbcb5a9b50dd3c"]
				.unchecked_into(),
			//5DPSWdgw38Spu315r6LSvYCggeeieBAJtP5A1qzuzKhqmjVu
			hex!["034f68c5661a41930c82f26a662276bf89f33467e1c850f2fb8ef687fe43d62276"]
				.unchecked_into(),
		),
		(
			//5DvH8oEjQPYhzCoQVo7WDU91qmQfLZvxe9wJcrojmJKebCmG
			hex!["520b48452969f6ddf263b664de0adb0c729d0e0ad3b0e5f3cb636c541bc9022a"].into(),
			//5ENZvCRzyXJJYup8bM6yEzb2kQHEb1NDpY2ZEyVGBkCfRdj3
			hex!["6618289af7ae8621981ffab34591e7a6486e12745dfa3fd3b0f7e6a3994c7b5b"].into(),
			//5DLjSUfqZVNAADbwYLgRvHvdzXypiV1DAEaDMjcESKTcqMoM
			hex!["38757d0de00a0c739e7d7984ef4bc01161bd61e198b7c01b618425c16bb5bd5f"]
				.unchecked_into(),
			//5HnDVBN9mD6mXyx8oryhDbJtezwNSj1VRXgLoYCBA6uEkiao
			hex!["fcd5f87a6fd5707a25122a01b4dac0a8482259df7d42a9a096606df1320df08d"]
				.unchecked_into(),
			//5DhyXZiuB1LvqYKFgT5tRpgGsN3is2cM9QxgW7FikvakbAZP
			hex!["48a910c0af90898f11bd57d37ceaea53c78994f8e1833a7ade483c9a84bde055"]
				.unchecked_into(),
			//5EPEWRecy2ApL5n18n3aHyU1956zXTRqaJpzDa9DoqiggNwF
			hex!["669a10892119453e9feb4e3f1ee8e028916cc3240022920ad643846fbdbee816"]
				.unchecked_into(),
			//5ES3fw5X4bndSgLNmtPfSbM2J1kLqApVB2CCLS4CBpM1UxUZ
			hex!["68bf52c482630a8d1511f2edd14f34127a7d7082219cccf7fd4c6ecdb535f80d"]
				.unchecked_into(),
			//5HeXbwb5PxtcRoopPZTp5CQun38atn2UudQ8p2AxR5BzoaXw
			hex!["f6f8fe475130d21165446a02fb1dbce3a7bf36412e5d98f4f0473aed9252f349"]
				.unchecked_into(),
			//5F7nTtN8MyJV4UsXpjg7tHSnfANXZ5KRPJmkASc1ZSH2Xoa5
			hex!["03a90c2bb6d3b7000020f6152fe2e5002fa970fd1f42aafb6c8edda8dacc2ea77e"]
				.unchecked_into(),
		),
		(
			//5FPMzsezo1PRxYbVpJMWK7HNbR2kUxidsAAxH4BosHa4wd6S
			hex!["92ef83665b39d7a565e11bf8d18d41d45a8011601c339e57a8ea88c8ff7bba6f"].into(),
			//5G6NQidFG7YiXsvV7hQTLGArir9tsYqD4JDxByhgxKvSKwRx
			hex!["b235f57244230589523271c27b8a490922ffd7dccc83b044feaf22273c1dc735"].into(),
			//5GpZhzAVg7SAtzLvaAC777pjquPEcNy1FbNUAG2nZvhmd6eY
			hex!["d2644c1ab2c63a3ad8d40ad70d4b260969e3abfe6d7e6665f50dc9f6365c9d2a"]
				.unchecked_into(),
			//5HAes2RQYPbYKbLBfKb88f4zoXv6pPA6Ke8CjN7dob3GpmSP
			hex!["e1b68fbd84333e31486c08e6153d9a1415b2e7e71b413702b7d64e9b631184a1"]
				.unchecked_into(),
			//5HTXBf36LXmkFWJLokNUK6fPxVpkr2ToUnB1pvaagdGu4c1T
			hex!["ee93e26259decb89afcf17ef2aa0fa2db2e1042fb8f56ecfb24d19eae8629878"]
				.unchecked_into(),
			//5FtAGDZYJKXkhVhAxCQrXmaP7EE2mGbBMfmKDHjfYDgq2BiU
			hex!["a8e61ffacafaf546283dc92d14d7cc70ea0151a5dd81fdf73ff5a2951f2b6037"]
				.unchecked_into(),
			//5CtK7JHv3h6UQZ44y54skxdwSVBRtuxwPE1FYm7UZVhg8rJV
			hex!["244f3421b310c68646e99cdbf4963e02067601f57756b072a4b19431448c186e"]
				.unchecked_into(),
			//5D4r6YaB6F7A7nvMRHNFNF6zrR9g39bqDJFenrcaFmTCRwfa
			hex!["2c57f81fd311c1ab53813c6817fe67f8947f8d39258252663b3384ab4195494d"]
				.unchecked_into(),
			//5EPoHj8uV4fFKQHYThc6Z9fDkU7B6ih2ncVzQuDdNFb8UyhF
			hex!["039d065fe4f9234f0a4f13cc3ae585f2691e9c25afa469618abb6645111f607a53"]
				.unchecked_into(),
		),
		(
			//5DMNx7RoX6d7JQ38NEM7DWRcW2THu92LBYZEWvBRhJeqcWgR
			hex!["38f3c2f38f6d47f161e98c697bbe3ca0e47c033460afda0dda314ab4222a0404"].into(),
			//5GGdKNDr9P47dpVnmtq3m8Tvowwf1ot1abw6tPsTYYFoKm2v
			hex!["ba0898c1964196474c0be08d364cdf4e9e1d47088287f5235f70b0590dfe1704"].into(),
			//5EjkyPCzR2SjhDZq8f7ufsw6TfkvgNRepjCRQFc4TcdXdaB1
			hex!["764186bc30fd5a02477f19948dc723d6d57ab174debd4f80ed6038ec960bfe21"]
				.unchecked_into(),
			//5DJV3zCBTJBLGNDCcdWrYxWDacSz84goGTa4pFeKVvehEBte
			hex!["36be9069cdb4a8a07ecd51f257875150f0a8a1be44a10d9d98dabf10a030aef4"]
				.unchecked_into(),
			//5FHf8kpK4fPjEJeYcYon2gAPwEBubRvtwpzkUbhMWSweKPUY
			hex!["8e95b9b5b4dc69790b67b566567ca8bf8cdef3a3a8bb65393c0d1d1c87cd2d2c"]
				.unchecked_into(),
			//5F9FsRjpecP9GonktmtFL3kjqNAMKjHVFjyjRdTPa4hbQRZA
			hex!["882d72965e642677583b333b2d173ac94b5fd6c405c76184bb14293be748a13b"]
				.unchecked_into(),
			//5F1FZWZSj3JyTLs8sRBxU6QWyGLSL9BMRtmSKDmVEoiKFxSP
			hex!["821271c99c958b9220f1771d9f5e29af969edfa865631dba31e1ab7bc0582b75"]
				.unchecked_into(),
			//5CtgRR74VypK4h154s369abs78hDUxZSJqcbWsfXvsjcHJNA
			hex!["2496f28d887d84705c6dae98aee8bf90fc5ad10bb5545eca1de6b68425b70f7c"]
				.unchecked_into(),
			//5CPx6dsr11SCJHKFkcAQ9jpparS7FwXQBrrMznRo4Hqv1PXz
			hex!["0307d29bbf6a5c4061c2157b44fda33b7bb4ec52a5a0305668c74688cedf288d58"]
				.unchecked_into(),
		),
		(
			//5C8AL1Zb4bVazgT3EgDxFgcow1L4SJjVu44XcLC9CrYqFN4N
			hex!["02a2d8cfcf75dda85fafc04ace3bcb73160034ed1964c43098fb1fe831de1b16"].into(),
			//5FLYy3YKsAnooqE4hCudttAsoGKbVG3hYYBtVzwMjJQrevPa
			hex!["90cab33f0bb501727faa8319f0845faef7d31008f178b65054b6629fe531b772"].into(),
			//5Et3tfbVf1ByFThNAuUq5pBssdaPPskip5yob5GNyUFojXC7
			hex!["7c94715e5dd8ab54221b1b6b2bfa5666f593f28a92a18e28052531de1bd80813"]
				.unchecked_into(),
			//5EX1JBghGbQqWohTPU6msR9qZ2nYPhK9r3RTQ2oD1K8TCxaG
			hex!["6c878e33b83c20324238d22240f735457b6fba544b383e70bb62a27b57380c81"]
				.unchecked_into(),
			//5GqL8RbVAuNXpDhjQi1KrS1MyNuKhvus2AbmQwRGjpuGZmFu
			hex!["d2f9d537ffa59919a4028afdb627c14c14c97a1547e13e8e82203d2049b15b1a"]
				.unchecked_into(),
			//5EUNaBpX9mJgcmLQHyG5Pkms6tbDiKuLbeTEJS924Js9cA1N
			hex!["6a8570b9c6408e54bacf123cc2bb1b0f087f9c149147d0005badba63a5a4ac01"]
				.unchecked_into(),
			//5CaZuueRVpMATZG4hkcrgDoF4WGixuz7zu83jeBdY3bgWGaG
			hex!["16c69ea8d595e80b6736f44be1eaeeef2ac9c04a803cc4fd944364cb0d617a33"]
				.unchecked_into(),
			//5DABsdQCDUGuhzVGWe5xXzYQ9rtrVxRygW7RXf9Tsjsw1aGJ
			hex!["306ac5c772fe858942f92b6e28bd82fb7dd8cdd25f9a4626c1b0eee075fcb531"]
				.unchecked_into(),
			//5H91T5mHhoCw9JJG4NjghDdQyhC6L7XcSuBWKD3q3TAhEVvQ
			hex!["02fb0330356e63a35dd930bc74525edf28b3bf5eb44aab9e9e4962c8309aaba6a6"]
				.unchecked_into(),
		),
		(
			//5C8XbDXdMNKJrZSrQURwVCxdNdk8AzG6xgLggbzuA399bBBF
			hex!["02ea6bfa8b23b92fe4b5db1063a1f9475e3acd0ab61e6b4f454ed6ba00b5f864"].into(),
			//5GsyzFP8qtF8tXPSsjhjxAeU1v7D1PZofuQKN9TdCc7Dp1JM
			hex!["d4ffc4c05b47d1115ad200f7f86e307b20b46c50e1b72a912ec4f6f7db46b616"].into(),
			//5GHWB8ZDzegLcMW7Gdd1BS6WHVwDdStfkkE4G7KjPjZNJBtD
			hex!["bab3cccdcc34401e9b3971b96a662686cf755aa869a5c4b762199ce531b12c5b"]
				.unchecked_into(),
			//5GzDPGbUM9uH52ZEwydasTj8edokGUJ7vEpoFWp9FE1YNuFB
			hex!["d9c056c98ca0e6b4eb7f5c58c007c1db7be0fe1f3776108f797dd4990d1ccc33"]
				.unchecked_into(),
			//5GWZbVkJEfWZ7fRca39YAQeqri2Z7pkeHyd7rUctUHyQifLp
			hex!["c4a980da30939d5bb9e4a734d12bf81259ae286aa21fa4b65405347fa40eff35"]
				.unchecked_into(),
			//5CmLCFeSurRXXtwMmLcVo7sdJ9EqDguvJbuCYDcHkr3cpqyE
			hex!["1efc23c0b51ad609ab670ecf45807e31acbd8e7e5cb7c07cf49ee42992d2867c"]
				.unchecked_into(),
			//5DnsSy8a8pfE2aFjKBDtKw7WM1V4nfE5sLzP15MNTka53GqS
			hex!["4c64d3f06d28adeb36a892fdaccecace150bec891f04694448a60b74fa469c22"]
				.unchecked_into(),
			//5CZdFnyzZvKetZTeUwj5APAYskVJe4QFiTezo5dQNsrnehGd
			hex!["160ea09c5717270e958a3da42673fa011613a9539b2e4ebcad8626bc117ca04a"]
				.unchecked_into(),
			//5HgoR9JJkdBusxKrrs3zgd3ToppgNoGj1rDyAJp4e7eZiYyT
			hex!["020019a8bb188f8145d02fa855e9c36e9914457d37c500e03634b5223aa5702474"]
				.unchecked_into(),
		),
		(
			//5HinEonzr8MywkqedcpsmwpxKje2jqr9miEwuzyFXEBCvVXM
			hex!["fa373e25a1c4fe19c7148acde13bc3db1811cf656dc086820f3dda736b9c4a00"].into(),
			//5EHJbj6Td6ks5HDnyfN4ttTSi57osxcQsQexm7XpazdeqtV7
			hex!["62145d721967bd88622d08625f0f5681463c0f1b8bcd97eb3c2c53f7660fd513"].into(),
			//5EeCsC58XgJ1DFaoYA1WktEpP27jvwGpKdxPMFjicpLeYu96
			hex!["720537e2c1c554654d73b3889c3ef4c3c2f95a65dd3f7c185ebe4afebed78372"]
				.unchecked_into(),
			//5DnEySxbnppWEyN8cCLqvGjAorGdLRg2VmkY96dbJ1LHFK8N
			hex!["4bea0b37e0cce9bddd80835fa2bfd5606f5dcfb8388bbb10b10c483f0856cf14"]
				.unchecked_into(),
			//5E1Y1FJ7dVP7qtE3wm241pTm72rTMcDT5Jd8Czv7Pwp7N3AH
			hex!["560d90ca51e9c9481b8a9810060e04d0708d246714960439f804e5c6f40ca651"]
				.unchecked_into(),
			//5CAC278tFCHAeHYqE51FTWYxHmeLcENSS1RG77EFRTvPZMJT
			hex!["042f07fc5268f13c026bbe199d63e6ac77a0c2a780f71cda05cee5a6f1b3f11f"]
				.unchecked_into(),
			//5HjRTLWcQjZzN3JDvaj1UzjNSayg5ZD9ZGWMstaL7Ab2jjAa
			hex!["fab485e87ed1537d089df521edf983a777c57065a702d7ed2b6a2926f31da74f"]
				.unchecked_into(),
			//5ELv74v7QcsS6FdzvG4vL2NnYDGWmRnJUSMKYwdyJD7Xcdi7
			hex!["64d59feddb3d00316a55906953fb3db8985797472bd2e6c7ea1ab730cc339d7f"]
				.unchecked_into(),
			//5FaUcPt4fPz93vBhcrCJqmDkjYZ7jCbzAF56QJoCmvPaKrmx
			hex!["033f1a6d47fe86f88934e4b83b9fae903b92b5dcf4fec97d5e3e8bf4f39df03685"]
				.unchecked_into(),
		),
		(
			//5Ey3NQ3dfabaDc16NUv7wRLsFCMDFJSqZFzKVycAsWuUC6Di
			hex!["8062e9c21f1d92926103119f7e8153cebdb1e5ab3e52d6f395be80bb193eab47"].into(),
			//5HiWsuSBqt8nS9pnggexXuHageUifVPKPHDE2arTKqhTp1dV
			hex!["fa0388fa88f3f0cb43d583e2571fbc0edad57dff3a6fd89775451dd2c2b8ea00"].into(),
			//5H168nKX2Yrfo3bxj7rkcg25326Uv3CCCnKUGK6uHdKMdPt8
			hex!["da6b2df18f0f9001a6dcf1d301b92534fe9b1f3ccfa10c49449fee93adaa8349"]
				.unchecked_into(),
			//5DrA2fZdzmNqT5j6DXNwVxPBjDV9jhkAqvjt6Us3bQHKy3cF
			hex!["4ee66173993dd0db5d628c4c9cb61a27b76611ad3c3925947f0d0011ee2c5dcc"]
				.unchecked_into(),
			//5FNFDUGNLUtqg5LgrwYLNmBiGoP8KRxsvQpBkc7GQP6qaBUG
			hex!["92156f54a114ee191415898f2da013d9db6a5362d6b36330d5fc23e27360ab66"]
				.unchecked_into(),
			//5Gx6YeNhynqn8qkda9QKpc9S7oDr4sBrfAu516d3sPpEt26F
			hex!["d822d4088b20dca29a580a577a97d6f024bb24c9550bebdfd7d2d18e946a1c7d"]
				.unchecked_into(),
			//5DhDcHqwxoes5s89AyudGMjtZXx1nEgrk5P45X88oSTR3iyx
			hex!["481538f8c2c011a76d7d57db11c2789a5e83b0f9680dc6d26211d2f9c021ae4c"]
				.unchecked_into(),
			//5DqAvikdpfRdk5rR35ZobZhqaC5bJXZcEuvzGtexAZP1hU3T
			hex!["4e262811acdfe94528bfc3c65036080426a0e1301b9ada8d687a70ffcae99c26"]
				.unchecked_into(),
			//5E41Znrr2YtZu8bZp3nvRuLVHg3jFksfQ3tXuviLku4wsao7
			hex!["025e84e95ed043e387ddb8668176b42f8e2773ddd84f7f58a6d9bf436a4b527986"]
				.unchecked_into(),
		),
	];

	const ENDOWMENT: u128 = 1_000_000 * ROC;
	const STASH: u128 = 100 * ROC;

	betanet_runtime::GenesisConfig {
		system: betanet_runtime::SystemConfig { code: wasm_binary.to_vec() },
		balances: betanet_runtime::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k: &AccountId| (k.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
		},
		beefy: Default::default(),
		indices: betanet_runtime::IndicesConfig { indices: vec![] },
		session: betanet_runtime::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						betanet_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
							x.8.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		babe: betanet_runtime::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(betanet_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		collective: Default::default(),
		membership: Default::default(),
		authority_discovery: betanet_runtime::AuthorityDiscoveryConfig { keys: vec![] },
		sudo: betanet_runtime::SudoConfig { key: Some(endowed_accounts[0].clone()) },
		paras: betanet_runtime::ParasConfig { paras: vec![] },
		hrmp: Default::default(),
		configuration: betanet_runtime::ConfigurationConfig {
			config: default_allychains_host_configuration(),
		},
		registrar: betanet_runtime::RegistrarConfig {
			next_free_para_id: axia_primitives::v1::LOWEST_PUBLIC_ID,
		},
		xcm_pallet: Default::default(),
		transaction_payment: Default::default(),
		bridge_betanet_grandpa: betanet_runtime::BridgeBetanetGrandpaConfig {
			owner: Some(endowed_accounts[0].clone()),
			..Default::default()
		},
		bridge_wococo_grandpa: betanet_runtime::BridgeWococoGrandpaConfig {
			owner: Some(endowed_accounts[0].clone()),
			..Default::default()
		},
		bridge_betanet_messages: betanet_runtime::BridgeBetanetMessagesConfig {
			owner: Some(endowed_accounts[0].clone()),
			..Default::default()
		},
		bridge_wococo_messages: betanet_runtime::BridgeWococoMessagesConfig {
			owner: Some(endowed_accounts[0].clone()),
			..Default::default()
		},
	}
}

/// Axia staging testnet config.
#[cfg(feature = "axia-native")]
pub fn axia_staging_testnet_config() -> Result<AxiaChainSpec, String> {
	let wasm_binary = axia::WASM_BINARY.ok_or("Axia development wasm not available")?;
	let boot_nodes = vec![];

	Ok(AxiaChainSpec::from_genesis(
		"Axia Staging Testnet",
		"axia_staging_testnet",
		ChainType::Live,
		move || axia_staging_testnet_config_genesis(wasm_binary),
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(AXIA_STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Axia Staging telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
}

/// Staging testnet config.
#[cfg(feature = "axctest-native")]
pub fn axctest_staging_testnet_config() -> Result<AxiaTestChainSpec, String> {
	let wasm_binary = axctest::WASM_BINARY.ok_or("AxiaTest development wasm not available")?;
	let boot_nodes = vec![];

	Ok(AxiaTestChainSpec::from_genesis(
		"AxiaTest Staging Testnet",
		"axctest_staging_testnet",
		ChainType::Live,
		move || axctest_staging_testnet_config_genesis(wasm_binary),
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(AXIATEST_STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("AxiaTest Staging telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
}

/// Alphanet staging testnet config.
#[cfg(feature = "alphanet-native")]
pub fn alphanet_staging_testnet_config() -> Result<AlphanetChainSpec, String> {
	let wasm_binary = alphanet::WASM_BINARY.ok_or("Alphanet development wasm not available")?;
	let boot_nodes = vec![];

	Ok(AlphanetChainSpec::from_genesis(
		"Alphanet Staging Testnet",
		"alphanet_staging_testnet",
		ChainType::Live,
		move || alphanet_staging_testnet_config_genesis(wasm_binary),
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(ALPHANET_STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Alphanet Staging telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
}

/// Betanet staging testnet config.
#[cfg(feature = "betanet-native")]
pub fn betanet_staging_testnet_config() -> Result<BetanetChainSpec, String> {
	let wasm_binary = betanet::WASM_BINARY.ok_or("Betanet development wasm not available")?;
	let boot_nodes = vec![];

	Ok(BetanetChainSpec::from_genesis(
		"Betanet Staging Testnet",
		"betanet_staging_testnet",
		ChainType::Live,
		move || BetanetGenesisExt {
			runtime_genesis_config: betanet_staging_testnet_config_genesis(wasm_binary),
			session_length_in_blocks: None,
		},
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(BETANET_STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Betanet Staging telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
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
pub fn get_authority_keys_from_seed(
	seed: &str,
) -> (
	AccountId,
	AccountId,
	BabeId,
	GrandpaId,
	ImOnlineId,
	ValidatorId,
	AssignmentId,
	AuthorityDiscoveryId,
	BeefyId,
) {
	let keys = get_authority_keys_from_seed_no_beefy(seed);
	(keys.0, keys.1, keys.2, keys.3, keys.4, keys.5, keys.6, keys.7, get_from_seed::<BeefyId>(seed))
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed_no_beefy(
	seed: &str,
) -> (
	AccountId,
	AccountId,
	BabeId,
	GrandpaId,
	ImOnlineId,
	ValidatorId,
	AssignmentId,
	AuthorityDiscoveryId,
) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<ValidatorId>(seed),
		get_from_seed::<AssignmentId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

fn testnet_accounts() -> Vec<AccountId> {
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
	]
}

/// Helper function to create axia `GenesisConfig` for testing
#[cfg(feature = "axia-native")]
pub fn axia_testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)>,
	_root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> axia::GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);

	const ENDOWMENT: u128 = 1_000_000 * AXC;
	const STASH: u128 = 100 * AXC;

	axia::GenesisConfig {
		system: axia::SystemConfig { code: wasm_binary.to_vec() },
		indices: axia::IndicesConfig { indices: vec![] },
		balances: axia::BalancesConfig {
			balances: endowed_accounts.iter().map(|k| (k.clone(), ENDOWMENT)).collect(),
		},
		session: axia::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						axia_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: axia::StakingConfig {
			minimum_validator_count: 1,
			validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, axia::StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::NotForcing,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		phragmen_election: Default::default(),
		democracy: axia::DemocracyConfig::default(),
		council: axia::CouncilConfig { members: vec![], phantom: Default::default() },
		technical_committee: axia::TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		},
		technical_membership: Default::default(),
		babe: axia::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(axia::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		authority_discovery: axia::AuthorityDiscoveryConfig { keys: vec![] },
		claims: axia::ClaimsConfig { claims: vec![], vesting: vec![] },
		vesting: axia::VestingConfig { vesting: vec![] },
		treasury: Default::default(),
		hrmp: Default::default(),
		configuration: axia::ConfigurationConfig {
			config: default_allychains_host_configuration(),
		},
		paras: Default::default(),
		xcm_pallet: Default::default(),
		sudo: axia::SudoConfig { key: Some(endowed_accounts[0].clone()) },
	}
}

/// Helper function to create axctest `GenesisConfig` for testing
#[cfg(feature = "axctest-native")]
pub fn axctest_testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)>,
	_root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> axctest::GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);

	const ENDOWMENT: u128 = 1_000_000 * KSM;
	const STASH: u128 = 100 * KSM;

	axctest::GenesisConfig {
		system: axctest::SystemConfig { code: wasm_binary.to_vec() },
		indices: axctest::IndicesConfig { indices: vec![] },
		balances: axctest::BalancesConfig {
			balances: endowed_accounts.iter().map(|k| (k.clone(), ENDOWMENT)).collect(),
		},
		session: axctest::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						axctest_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: axctest::StakingConfig {
			minimum_validator_count: 1,
			validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, axctest::StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::NotForcing,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		phragmen_election: Default::default(),
		democracy: axctest::DemocracyConfig::default(),
		council: axctest::CouncilConfig { members: vec![], phantom: Default::default() },
		technical_committee: axctest::TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		},
		technical_membership: Default::default(),
		babe: axctest::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(axctest::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		authority_discovery: axctest::AuthorityDiscoveryConfig { keys: vec![] },
		claims: axctest::ClaimsConfig { claims: vec![], vesting: vec![] },
		vesting: axctest::VestingConfig { vesting: vec![] },
		treasury: Default::default(),
		hrmp: Default::default(),
		configuration: axctest::ConfigurationConfig {
			config: default_allychains_host_configuration(),
		},
		gilt: Default::default(),
		paras: Default::default(),
		xcm_pallet: Default::default(),
	}
}

/// Helper function to create alphanet `GenesisConfig` for testing
#[cfg(feature = "alphanet-native")]
pub fn alphanet_testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
	)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> alphanet::GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);

	const ENDOWMENT: u128 = 1_000_000 * AXC;
	const STASH: u128 = 100 * AXC;

	alphanet::GenesisConfig {
		system: alphanet::SystemConfig { code: wasm_binary.to_vec() },
		indices: alphanet::IndicesConfig { indices: vec![] },
		balances: alphanet::BalancesConfig {
			balances: endowed_accounts.iter().map(|k| (k.clone(), ENDOWMENT)).collect(),
		},
		session: alphanet::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						alphanet_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: alphanet::StakingConfig {
			minimum_validator_count: 1,
			validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, alphanet::StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::NotForcing,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		babe: alphanet::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(alphanet::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		authority_discovery: alphanet::AuthorityDiscoveryConfig { keys: vec![] },
		vesting: alphanet::VestingConfig { vesting: vec![] },
		sudo: alphanet::SudoConfig { key: Some(root_key) },
		hrmp: Default::default(),
		configuration: alphanet::ConfigurationConfig {
			config: default_allychains_host_configuration(),
		},
		paras: Default::default(),
		registrar: alphanet_runtime::RegistrarConfig {
			next_free_para_id: axia_primitives::v1::LOWEST_PUBLIC_ID,
		},
		xcm_pallet: Default::default(),
	}
}

/// Helper function to create betanet `GenesisConfig` for testing
#[cfg(feature = "betanet-native")]
pub fn betanet_testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
		BeefyId,
	)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> betanet_runtime::GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);

	const ENDOWMENT: u128 = 1_000_000 * AXC;

	betanet_runtime::GenesisConfig {
		system: betanet_runtime::SystemConfig { code: wasm_binary.to_vec() },
		beefy: Default::default(),
		indices: betanet_runtime::IndicesConfig { indices: vec![] },
		balances: betanet_runtime::BalancesConfig {
			balances: endowed_accounts.iter().map(|k| (k.clone(), ENDOWMENT)).collect(),
		},
		session: betanet_runtime::SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						betanet_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
							x.8.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		babe: betanet_runtime::BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(betanet_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: Default::default(),
		im_online: Default::default(),
		collective: Default::default(),
		membership: Default::default(),
		authority_discovery: betanet_runtime::AuthorityDiscoveryConfig { keys: vec![] },
		sudo: betanet_runtime::SudoConfig { key: Some(root_key.clone()) },
		hrmp: Default::default(),
		configuration: betanet_runtime::ConfigurationConfig {
			config: axia_runtime_allychains::configuration::HostConfiguration {
				max_validators_per_core: Some(1),
				..default_allychains_host_configuration()
			},
		},
		paras: betanet_runtime::ParasConfig { paras: vec![] },
		registrar: betanet_runtime::RegistrarConfig {
			next_free_para_id: axia_primitives::v1::LOWEST_PUBLIC_ID,
		},
		xcm_pallet: Default::default(),
		transaction_payment: Default::default(),
		bridge_betanet_grandpa: betanet_runtime::BridgeBetanetGrandpaConfig {
			owner: Some(root_key.clone()),
			..Default::default()
		},
		bridge_wococo_grandpa: betanet_runtime::BridgeWococoGrandpaConfig {
			owner: Some(root_key.clone()),
			..Default::default()
		},
		bridge_betanet_messages: betanet_runtime::BridgeBetanetMessagesConfig {
			owner: Some(root_key.clone()),
			..Default::default()
		},
		bridge_wococo_messages: betanet_runtime::BridgeWococoMessagesConfig {
			owner: Some(root_key.clone()),
			..Default::default()
		},
	}
}

#[cfg(feature = "axia-native")]
fn axia_development_config_genesis(wasm_binary: &[u8]) -> axia::GenesisConfig {
	axia_testnet_genesis(
		wasm_binary,
		vec![get_authority_keys_from_seed_no_beefy("Alice")],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

#[cfg(feature = "axctest-native")]
fn axctest_development_config_genesis(wasm_binary: &[u8]) -> axctest::GenesisConfig {
	axctest_testnet_genesis(
		wasm_binary,
		vec![get_authority_keys_from_seed_no_beefy("Alice")],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

#[cfg(feature = "alphanet-native")]
fn alphanet_development_config_genesis(wasm_binary: &[u8]) -> alphanet::GenesisConfig {
	alphanet_testnet_genesis(
		wasm_binary,
		vec![get_authority_keys_from_seed_no_beefy("Alice")],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

#[cfg(feature = "betanet-native")]
fn betanet_development_config_genesis(wasm_binary: &[u8]) -> betanet_runtime::GenesisConfig {
	betanet_testnet_genesis(
		wasm_binary,
		vec![get_authority_keys_from_seed("Alice")],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Axia development config (single validator Alice)
#[cfg(feature = "axia-native")]
pub fn axia_development_config() -> Result<AxiaChainSpec, String> {
	let wasm_binary = axia::WASM_BINARY.ok_or("Axia development wasm not available")?;

	Ok(AxiaChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		move || axia_development_config_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
}

/// AxiaTest development config (single validator Alice)
#[cfg(feature = "axctest-native")]
pub fn axctest_development_config() -> Result<AxiaTestChainSpec, String> {
	let wasm_binary = axctest::WASM_BINARY.ok_or("AxiaTest development wasm not available")?;

	Ok(AxiaTestChainSpec::from_genesis(
		"Development",
		"axctest_dev",
		ChainType::Development,
		move || axctest_development_config_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
}

/// Alphanet development config (single validator Alice)
#[cfg(feature = "alphanet-native")]
pub fn alphanet_development_config() -> Result<AlphanetChainSpec, String> {
	let wasm_binary = alphanet::WASM_BINARY.ok_or("Alphanet development wasm not available")?;

	Ok(AlphanetChainSpec::from_genesis(
		"Development",
		"alphanet_dev",
		ChainType::Development,
		move || alphanet_development_config_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
}

/// Betanet development config (single validator Alice)
#[cfg(feature = "betanet-native")]
pub fn betanet_development_config() -> Result<BetanetChainSpec, String> {
	let wasm_binary = betanet::WASM_BINARY.ok_or("Betanet development wasm not available")?;

	Ok(BetanetChainSpec::from_genesis(
		"Development",
		"betanet_dev",
		ChainType::Development,
		move || BetanetGenesisExt {
			runtime_genesis_config: betanet_development_config_genesis(wasm_binary),
			// Use 1 minute session length.
			session_length_in_blocks: Some(10),
		},
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
}

/// `Versi` development config (single validator Alice)
#[cfg(feature = "betanet-native")]
pub fn versi_development_config() -> Result<BetanetChainSpec, String> {
	let wasm_binary = betanet::WASM_BINARY.ok_or("Versi development wasm not available")?;

	Ok(BetanetChainSpec::from_genesis(
		"Development",
		"versi_dev",
		ChainType::Development,
		move || BetanetGenesisExt {
			runtime_genesis_config: betanet_development_config_genesis(wasm_binary),
			// Use 1 minute session length.
			session_length_in_blocks: Some(10),
		},
		vec![],
		None,
		Some("versi"),
		None,
		None,
		Default::default(),
	))
}

/// Wococo development config (single validator Alice)
#[cfg(feature = "betanet-native")]
pub fn wococo_development_config() -> Result<BetanetChainSpec, String> {
	const WOCOCO_DEV_PROTOCOL_ID: &str = "woco";
	let wasm_binary = betanet::WASM_BINARY.ok_or("Wococo development wasm not available")?;

	Ok(BetanetChainSpec::from_genesis(
		"Development",
		"wococo_dev",
		ChainType::Development,
		move || BetanetGenesisExt {
			runtime_genesis_config: betanet_development_config_genesis(wasm_binary),
			// Use 1 minute session length.
			session_length_in_blocks: Some(10),
		},
		vec![],
		None,
		Some(WOCOCO_DEV_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
}

#[cfg(feature = "axia-native")]
fn axia_local_testnet_genesis(wasm_binary: &[u8]) -> axia::GenesisConfig {
	axia_testnet_genesis(
		wasm_binary,
		vec![
			get_authority_keys_from_seed_no_beefy("Alice"),
			get_authority_keys_from_seed_no_beefy("Bob"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Axia local testnet config (multivalidator Alice + Bob)
#[cfg(feature = "axia-native")]
pub fn axia_local_testnet_config() -> Result<AxiaChainSpec, String> {
	let wasm_binary = axia::WASM_BINARY.ok_or("Axia development wasm not available")?;

	Ok(AxiaChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		move || axia_local_testnet_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
}

#[cfg(feature = "axctest-native")]
fn axctest_local_testnet_genesis(wasm_binary: &[u8]) -> axctest::GenesisConfig {
	axctest_testnet_genesis(
		wasm_binary,
		vec![
			get_authority_keys_from_seed_no_beefy("Alice"),
			get_authority_keys_from_seed_no_beefy("Bob"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// AxiaTest local testnet config (multivalidator Alice + Bob)
#[cfg(feature = "axctest-native")]
pub fn axctest_local_testnet_config() -> Result<AxiaTestChainSpec, String> {
	let wasm_binary = axctest::WASM_BINARY.ok_or("AxiaTest development wasm not available")?;

	Ok(AxiaTestChainSpec::from_genesis(
		"AxiaTest Local Testnet",
		"axctest_local_testnet",
		ChainType::Local,
		move || axctest_local_testnet_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
}

#[cfg(feature = "alphanet-native")]
fn alphanet_local_testnet_genesis(wasm_binary: &[u8]) -> alphanet::GenesisConfig {
	alphanet_testnet_genesis(
		wasm_binary,
		vec![
			get_authority_keys_from_seed_no_beefy("Alice"),
			get_authority_keys_from_seed_no_beefy("Bob"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Alphanet local testnet config (multivalidator Alice + Bob)
#[cfg(feature = "alphanet-native")]
pub fn alphanet_local_testnet_config() -> Result<AlphanetChainSpec, String> {
	let wasm_binary = alphanet::WASM_BINARY.ok_or("Alphanet development wasm not available")?;

	Ok(AlphanetChainSpec::from_genesis(
		"Alphanet Local Testnet",
		"alphanet_local_testnet",
		ChainType::Local,
		move || alphanet_local_testnet_genesis(wasm_binary),
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
}

#[cfg(feature = "betanet-native")]
fn betanet_local_testnet_genesis(wasm_binary: &[u8]) -> betanet_runtime::GenesisConfig {
	betanet_testnet_genesis(
		wasm_binary,
		vec![get_authority_keys_from_seed("Alice"), get_authority_keys_from_seed("Bob")],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Betanet local testnet config (multivalidator Alice + Bob)
#[cfg(feature = "betanet-native")]
pub fn betanet_local_testnet_config() -> Result<BetanetChainSpec, String> {
	let wasm_binary = betanet::WASM_BINARY.ok_or("Betanet development wasm not available")?;

	Ok(BetanetChainSpec::from_genesis(
		"Betanet Local Testnet",
		"betanet_local_testnet",
		ChainType::Local,
		move || BetanetGenesisExt {
			runtime_genesis_config: betanet_local_testnet_genesis(wasm_binary),
			// Use 1 minute session length.
			session_length_in_blocks: Some(10),
		},
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
}

/// Wococo is a temporary testnet that uses almost the same runtime as betanet.
#[cfg(feature = "betanet-native")]
fn wococo_local_testnet_genesis(wasm_binary: &[u8]) -> betanet_runtime::GenesisConfig {
	betanet_testnet_genesis(
		wasm_binary,
		vec![
			get_authority_keys_from_seed("Alice"),
			get_authority_keys_from_seed("Bob"),
			get_authority_keys_from_seed("Charlie"),
			get_authority_keys_from_seed("Dave"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// `Versi` is a temporary testnet that uses the same runtime as betanet.
#[cfg(feature = "betanet-native")]
fn versi_local_testnet_genesis(wasm_binary: &[u8]) -> betanet_runtime::GenesisConfig {
	betanet_testnet_genesis(
		wasm_binary,
		vec![
			get_authority_keys_from_seed("Alice"),
			get_authority_keys_from_seed("Bob"),
			get_authority_keys_from_seed("Charlie"),
			get_authority_keys_from_seed("Dave"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Wococo local testnet config (multivalidator Alice + Bob + Charlie + Dave)
#[cfg(feature = "betanet-native")]
pub fn wococo_local_testnet_config() -> Result<BetanetChainSpec, String> {
	let wasm_binary = betanet::WASM_BINARY.ok_or("Wococo development wasm not available")?;

	Ok(BetanetChainSpec::from_genesis(
		"Wococo Local Testnet",
		"wococo_local_testnet",
		ChainType::Local,
		move || BetanetGenesisExt {
			runtime_genesis_config: wococo_local_testnet_genesis(wasm_binary),
			// Use 1 minute session length.
			session_length_in_blocks: Some(10),
		},
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
}

/// `Versi` local testnet config (multivalidator Alice + Bob + Charlie + Dave)
#[cfg(feature = "betanet-native")]
pub fn versi_local_testnet_config() -> Result<BetanetChainSpec, String> {
	let wasm_binary = betanet::WASM_BINARY.ok_or("Versi development wasm not available")?;

	Ok(BetanetChainSpec::from_genesis(
		"Versi Local Testnet",
		"versi_local_testnet",
		ChainType::Local,
		move || BetanetGenesisExt {
			runtime_genesis_config: versi_local_testnet_genesis(wasm_binary),
			// Use 1 minute session length.
			session_length_in_blocks: Some(10),
		},
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None,
		Default::default(),
	))
}
