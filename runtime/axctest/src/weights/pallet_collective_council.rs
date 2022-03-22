// Copyright 2017-2021 Parity Technologies (UK) Ltd.
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
//! Autogenerated weights for `pallet_collective`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE AXLIB BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-08-27, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("axctest-dev"), DB CACHE: 128

// Executed Command:
// target/release/axia
// benchmark
// --chain=axctest-dev
// --steps=50
// --repeat=20
// --pallet=pallet_collective
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/axctest/src/weights/


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	// Storage: Instance1Collective Members (r:1 w:1)
	// Storage: Instance1Collective Proposals (r:1 w:0)
	// Storage: Instance1Collective Voting (r:100 w:100)
	// Storage: Instance1Collective Prime (r:0 w:1)
	fn set_members(m: u32, n: u32, p: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 6_000
			.saturating_add((14_448_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 6_000
			.saturating_add((85_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 6_000
			.saturating_add((19_620_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(p as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	// Storage: Instance1Collective Members (r:1 w:0)
	fn execute(b: u32, m: u32, ) -> Weight {
		(22_536_000 as Weight)
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 0
			.saturating_add((84_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: Instance1Collective Members (r:1 w:0)
	// Storage: Instance1Collective ProposalOf (r:1 w:0)
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		(27_600_000 as Weight)
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 0
			.saturating_add((161_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: Instance1Collective Members (r:1 w:0)
	// Storage: Instance1Collective ProposalOf (r:1 w:1)
	// Storage: Instance1Collective Proposals (r:1 w:1)
	// Storage: Instance1Collective ProposalCount (r:1 w:1)
	// Storage: Instance1Collective Voting (r:0 w:1)
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		(42_192_000 as Weight)
			// Standard Error: 0
			.saturating_add((4_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 0
			.saturating_add((87_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 0
			.saturating_add((361_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Instance1Collective Members (r:1 w:0)
	// Storage: Instance1Collective Voting (r:1 w:1)
	fn vote(m: u32, ) -> Weight {
		(32_307_000 as Weight)
			// Standard Error: 0
			.saturating_add((199_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance1Collective Voting (r:1 w:1)
	// Storage: Instance1Collective Members (r:1 w:0)
	// Storage: Instance1Collective Proposals (r:1 w:1)
	// Storage: Instance1Collective ProposalOf (r:0 w:1)
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		(41_436_000 as Weight)
			// Standard Error: 0
			.saturating_add((170_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 0
			.saturating_add((333_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1Collective Voting (r:1 w:1)
	// Storage: Instance1Collective Members (r:1 w:0)
	// Storage: Instance1Collective ProposalOf (r:1 w:1)
	// Storage: Instance1Collective Proposals (r:1 w:1)
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		(57_836_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 0
			.saturating_add((170_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 0
			.saturating_add((339_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1Collective Voting (r:1 w:1)
	// Storage: Instance1Collective Members (r:1 w:0)
	// Storage: Instance1Collective Prime (r:1 w:0)
	// Storage: Instance1Collective Proposals (r:1 w:1)
	// Storage: Instance1Collective ProposalOf (r:0 w:1)
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		(45_551_000 as Weight)
			// Standard Error: 0
			.saturating_add((172_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 0
			.saturating_add((338_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1Collective Voting (r:1 w:1)
	// Storage: Instance1Collective Members (r:1 w:0)
	// Storage: Instance1Collective Prime (r:1 w:0)
	// Storage: Instance1Collective ProposalOf (r:1 w:1)
	// Storage: Instance1Collective Proposals (r:1 w:1)
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		(61_497_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 0
			.saturating_add((171_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 0
			.saturating_add((343_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance1Collective Proposals (r:1 w:1)
	// Storage: Instance1Collective Voting (r:0 w:1)
	// Storage: Instance1Collective ProposalOf (r:0 w:1)
	fn disapprove_proposal(p: u32, ) -> Weight {
		(25_573_000 as Weight)
			// Standard Error: 0
			.saturating_add((335_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}
