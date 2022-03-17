// Copyright 2017-2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `runtime_parachains::paras_inherent`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-12-02, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("westend-dev"), DB CACHE: 128

// Executed Command:
// target/release/polkadot
// benchmark
// --chain=westend-dev
// --steps=50
// --repeat=20
// --pallet=runtime_parachains::paras_inherent
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/westend/src/weights/runtime_parachains_paras_inherent.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_parachains::paras_inherent`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::paras_inherent::WeightInfo for WeightInfo<T> {
	// Storage: ParaInherent Included (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: ParaScheduler AvailabilityCores (r:1 w:1)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: ParaInclusion PendingAvailability (r:2 w:1)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: Paras Parachains (r:1 w:0)
	// Storage: ParaInclusion PendingAvailabilityCommitments (r:1 w:1)
	// Storage: Session Validators (r:1 w:0)
	// Storage: ParasShared ActiveValidatorIndices (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	// Storage: ParaScheduler SessionStartBlock (r:1 w:0)
	// Storage: ParaScheduler ParathreadQueue (r:1 w:1)
	// Storage: ParaScheduler Scheduled (r:1 w:1)
	// Storage: ParaScheduler ValidatorGroups (r:1 w:0)
	// Storage: Ump NeedsDispatch (r:1 w:1)
	// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	// Storage: ParaInherent OnChainVotes (r:0 w:1)
	// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	// Storage: Paras Heads (r:0 w:1)
	fn enter_variable_disputes(v: u32, ) -> Weight {
		(199_341_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((312_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(22 as Weight))
			.saturating_add(T::DbWeight::get().writes(14 as Weight))
	}
	// Storage: ParaInherent Included (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: ParaScheduler AvailabilityCores (r:1 w:1)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: Paras Parachains (r:1 w:0)
	// Storage: ParaInclusion PendingAvailability (r:2 w:1)
	// Storage: ParaInclusion PendingAvailabilityCommitments (r:1 w:1)
	// Storage: Session Validators (r:1 w:0)
	// Storage: ParasShared ActiveValidatorIndices (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	// Storage: ParaScheduler SessionStartBlock (r:1 w:0)
	// Storage: ParaScheduler ParathreadQueue (r:1 w:1)
	// Storage: ParaScheduler Scheduled (r:1 w:1)
	// Storage: ParaScheduler ValidatorGroups (r:1 w:0)
	// Storage: Ump NeedsDispatch (r:1 w:1)
	// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	// Storage: ParaInclusion AvailabilityBitfields (r:0 w:1)
	// Storage: ParaInherent OnChainVotes (r:0 w:1)
	// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	// Storage: Paras Heads (r:0 w:1)
	fn enter_bitfields() -> Weight {
		(241_955_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(22 as Weight))
			.saturating_add(T::DbWeight::get().writes(15 as Weight))
	}
	// Storage: ParaInherent Included (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: ParaScheduler AvailabilityCores (r:1 w:1)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: Paras Parachains (r:1 w:0)
	// Storage: ParaInclusion PendingAvailability (r:2 w:1)
	// Storage: ParaInclusion PendingAvailabilityCommitments (r:1 w:1)
	// Storage: Session Validators (r:1 w:0)
	// Storage: ParasShared ActiveValidatorIndices (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	// Storage: ParaScheduler SessionStartBlock (r:1 w:0)
	// Storage: ParaScheduler ParathreadQueue (r:1 w:1)
	// Storage: ParaScheduler Scheduled (r:1 w:1)
	// Storage: ParaScheduler ValidatorGroups (r:1 w:0)
	// Storage: Paras PastCodeMeta (r:1 w:0)
	// Storage: Paras CurrentCodeHash (r:1 w:0)
	// Storage: Ump RelayDispatchQueueSize (r:1 w:0)
	// Storage: Ump NeedsDispatch (r:1 w:1)
	// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	// Storage: ParaInherent OnChainVotes (r:0 w:1)
	// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	// Storage: Paras Heads (r:0 w:1)
	fn enter_backed_candidates_variable(v: u32, ) -> Weight {
		(312_340_000 as Weight)
			// Standard Error: 25_000
			.saturating_add((49_112_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(25 as Weight))
			.saturating_add(T::DbWeight::get().writes(14 as Weight))
	}
	// Storage: ParaInherent Included (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: ParaScheduler AvailabilityCores (r:1 w:1)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: Paras Parachains (r:1 w:0)
	// Storage: ParaInclusion PendingAvailability (r:2 w:1)
	// Storage: ParaInclusion PendingAvailabilityCommitments (r:1 w:1)
	// Storage: Session Validators (r:1 w:0)
	// Storage: ParasShared ActiveValidatorIndices (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	// Storage: ParaScheduler SessionStartBlock (r:1 w:0)
	// Storage: ParaScheduler ParathreadQueue (r:1 w:1)
	// Storage: ParaScheduler Scheduled (r:1 w:1)
	// Storage: ParaScheduler ValidatorGroups (r:1 w:0)
	// Storage: Paras PastCodeMeta (r:1 w:0)
	// Storage: Paras CurrentCodeHash (r:1 w:0)
	// Storage: Ump RelayDispatchQueueSize (r:1 w:0)
	// Storage: Ump NeedsDispatch (r:1 w:1)
	// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	// Storage: ParaInherent OnChainVotes (r:0 w:1)
	// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	// Storage: Paras Heads (r:0 w:1)
	fn enter_backed_candidate_code_upgrade() -> Weight {
		(42_008_483_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(25 as Weight))
			.saturating_add(T::DbWeight::get().writes(14 as Weight))
	}
}
