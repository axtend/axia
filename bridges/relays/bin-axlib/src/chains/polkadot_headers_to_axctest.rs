// Copyright 2019-2021 Axia Technologies (UK) Ltd.
// This file is part of Axia Bridges Common.

// Axia Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Axia Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Axia Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Axia-to-AxiaTest headers sync entrypoint.

use codec::Encode;
use sp_core::{Bytes, Pair};

use bp_header_chain::justification::GrandpaJustification;
use relay_axctest_client::{AxiaTest, SigningParams as AxiaTestSigningParams};
use relay_polkadot_client::{Axia, SyncHeader as AxiaSyncHeader};
use relay_axlib_client::{Client, TransactionSignScheme, UnsignedTransaction};
use relay_utils::metrics::MetricsParams;
use axlib_relay_helper::finality_pipeline::{
	AxlibFinalitySyncPipeline, AxlibFinalityToAxlib,
};

/// Maximal saturating difference between `balance(now)` and `balance(now-24h)` to treat
/// relay as gone wild.
///
/// Actual value, returned by `maximal_balance_decrease_per_day_is_sane` test is approximately 0.001
/// KSM, but let's round up to 0.1 KSM here.
pub(crate) const MAXIMAL_BALANCE_DECREASE_PER_DAY: bp_polkadot::Balance = 100_000_000_000;

/// Axia-to-AxiaTest finality sync pipeline.
pub(crate) type FinalityPipelineAxiaFinalityToAxiaTest =
	AxlibFinalityToAxlib<Axia, AxiaTest, AxiaTestSigningParams>;

#[derive(Clone, Debug)]
pub(crate) struct AxiaFinalityToAxiaTest {
	finality_pipeline: FinalityPipelineAxiaFinalityToAxiaTest,
}

impl AxiaFinalityToAxiaTest {
	pub fn new(target_client: Client<AxiaTest>, target_sign: AxiaTestSigningParams) -> Self {
		Self {
			finality_pipeline: FinalityPipelineAxiaFinalityToAxiaTest::new(
				target_client,
				target_sign,
			),
		}
	}
}

impl AxlibFinalitySyncPipeline for AxiaFinalityToAxiaTest {
	type FinalitySyncPipeline = FinalityPipelineAxiaFinalityToAxiaTest;

	const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str =
		bp_polkadot::BEST_FINALIZED_AXIA_HEADER_METHOD;

	type TargetChain = AxiaTest;

	fn customize_metrics(params: MetricsParams) -> anyhow::Result<MetricsParams> {
		crate::chains::add_polkadot_axctest_price_metrics::<Self::FinalitySyncPipeline>(params)
	}

	fn start_relay_guards(&self) {
		relay_axlib_client::guard::abort_on_spec_version_change(
			self.finality_pipeline.target_client.clone(),
			bp_axctest::VERSION.spec_version,
		);
		relay_axlib_client::guard::abort_when_account_balance_decreased(
			self.finality_pipeline.target_client.clone(),
			self.transactions_author(),
			MAXIMAL_BALANCE_DECREASE_PER_DAY,
		);
	}

	fn transactions_author(&self) -> bp_axctest::AccountId {
		(*self.finality_pipeline.target_sign.public().as_array_ref()).into()
	}

	fn make_submit_finality_proof_transaction(
		&self,
		era: bp_runtime::TransactionEraOf<AxiaTest>,
		transaction_nonce: bp_runtime::IndexOf<AxiaTest>,
		header: AxiaSyncHeader,
		proof: GrandpaJustification<bp_polkadot::Header>,
	) -> Bytes {
		let call = relay_axctest_client::runtime::Call::BridgeAxiaGrandpa(
			relay_axctest_client::runtime::BridgeAxiaGrandpaCall::submit_finality_proof(
				Box::new(header.into_inner()),
				proof,
			),
		);
		let genesis_hash = *self.finality_pipeline.target_client.genesis_hash();
		let transaction = AxiaTest::sign_transaction(
			genesis_hash,
			&self.finality_pipeline.target_sign,
			era,
			UnsignedTransaction::new(call, transaction_nonce),
		);

		Bytes(transaction.encode())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::chains::axctest_headers_to_polkadot::tests::compute_maximal_balance_decrease_per_day;

	#[test]
	fn maximal_balance_decrease_per_day_is_sane() {
		// we expect Axia -> AxiaTest relay to be running in mandatory-headers-only mode
		// => we expect single header for every Axia session
		let maximal_balance_decrease = compute_maximal_balance_decrease_per_day::<
			bp_axctest::Balance,
			bp_axctest::WeightToFee,
		>(bp_polkadot::DAYS / bp_polkadot::SESSION_LENGTH + 1);
		assert!(
			MAXIMAL_BALANCE_DECREASE_PER_DAY >= maximal_balance_decrease,
			"Maximal expected loss per day {} is larger than hardcoded {}",
			maximal_balance_decrease,
			MAXIMAL_BALANCE_DECREASE_PER_DAY,
		);
	}
}
