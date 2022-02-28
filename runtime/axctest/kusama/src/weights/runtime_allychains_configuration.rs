// Copyright 2017-2021 Axia Technologies (UK) Ltd.
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
//! Autogenerated weights for `runtime_allychains::configuration`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE AXLIB BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-12-16, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("axctest-dev"), DB CACHE: 128

// Executed Command:
// target/release/polkadot
// benchmark
// --chain=axctest-dev
// --steps=50
// --repeat=20
// --pallet=runtime_allychains::configuration
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/axctest/src/weights/runtime_allychains_configuration.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_allychains::configuration`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_allychains::configuration::WeightInfo for WeightInfo<T> {
	// Storage: Configuration PendingConfigs (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	fn set_config_with_block_number() -> Weight {
		(8_240_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Configuration PendingConfigs (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	fn set_config_with_u32() -> Weight {
		(8_123_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Configuration PendingConfigs (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	fn set_config_with_option_u32() -> Weight {
		(8_547_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Configuration PendingConfigs (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	fn set_config_with_weight() -> Weight {
		(8_379_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Benchmark Override (r:0 w:0)
	fn set_hrmp_open_request_ttl() -> Weight {
		(2_000_000_000_000 as Weight)
	}
	// Storage: Configuration PendingConfigs (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	fn set_config_with_balance() -> Weight {
		(8_400_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
