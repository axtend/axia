// Copyright 2017-2020 Parity Technologies (UK) Ltd.
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
//! Autogenerated weights for `pallet_multisig`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-07-02, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("westend-dev"), DB CACHE: 128

// Executed Command:
// target/release/axia
// benchmark
// --chain=westend-dev
// --steps=50
// --repeat=20
// --pallet=pallet_multisig
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/westend/src/weights/

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_multisig`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for WeightInfo<T> {
	fn as_multi_threshold_1(_z: u32) -> Weight {
		(12_189_000 as Weight)
	}
	fn as_multi_create(s: u32, z: u32) -> Weight {
		(50_768_000 as Weight)
			// Standard Error: 0
			.saturating_add((106_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(z as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn as_multi_create_store(s: u32, z: u32) -> Weight {
		(56_293_000 as Weight)
			// Standard Error: 0
			.saturating_add((110_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(z as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn as_multi_approve(s: u32, z: u32) -> Weight {
		(29_281_000 as Weight)
			// Standard Error: 0
			.saturating_add((105_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(z as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn as_multi_approve_store(s: u32, z: u32) -> Weight {
		(53_770_000 as Weight)
			// Standard Error: 0
			.saturating_add((122_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(z as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn as_multi_complete(s: u32, z: u32) -> Weight {
		(70_625_000 as Weight)
			// Standard Error: 0
			.saturating_add((212_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((4_000 as Weight).saturating_mul(z as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn approve_as_multi_create(s: u32) -> Weight {
		(49_662_000 as Weight)
			// Standard Error: 0
			.saturating_add((107_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn approve_as_multi_approve(s: u32) -> Weight {
		(28_469_000 as Weight)
			// Standard Error: 0
			.saturating_add((107_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn approve_as_multi_complete(s: u32) -> Weight {
		(121_389_000 as Weight)
			// Standard Error: 0
			.saturating_add((212_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn cancel_as_multi(s: u32) -> Weight {
		(86_993_000 as Weight)
			// Standard Error: 0
			.saturating_add((102_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
