// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Pallet for checking GRANDPA Finality Proofs.
//!
//! Adapted copy of `axlib/client/finality-grandpa/src/justification.rs`. If origin
//! will ever be moved to the sp_finality_grandpa, we should reuse that implementation.

use codec::{Decode, Encode};
use finality_grandpa::voter_set::VoterSet;
use frame_support::RuntimeDebug;
use scale_info::TypeInfo;
use sp_finality_grandpa::{AuthorityId, AuthoritySignature, SetId};
use sp_runtime::traits::Header as HeaderT;
use sp_std::{
	collections::{btree_map::BTreeMap, btree_set::BTreeSet},
	prelude::*,
};

/// A GRANDPA Justification is a proof that a given header was finalized
/// at a certain height and with a certain set of authorities.
///
/// This particular proof is used to prove that headers on a bridged chain
/// (so not our chain) have been finalized correctly.
#[derive(Encode, Decode, RuntimeDebug, Clone, PartialEq, Eq, TypeInfo)]
pub struct GrandpaJustification<Header: HeaderT> {
	/// The round (voting period) this justification is valid for.
	pub round: u64,
	/// The set of votes for the chain which is to be finalized.
	pub commit:
		finality_grandpa::Commit<Header::Hash, Header::Number, AuthoritySignature, AuthorityId>,
	/// A proof that the chain of blocks in the commit are related to each other.
	pub votes_ancestries: Vec<Header>,
}

impl<H: HeaderT> crate::FinalityProof<H::Number> for GrandpaJustification<H> {
	fn target_header_number(&self) -> H::Number {
		self.commit.target_number
	}
}

/// Justification verification error.
#[derive(RuntimeDebug, PartialEq)]
pub enum Error {
	/// Failed to decode justification.
	JustificationDecode,
	/// Justification is finalizing unexpected header.
	InvalidJustificationTarget,
	/// The authority has provided an invalid signature.
	InvalidAuthoritySignature,
	/// The justification contains precommit for header that is not a descendant of the commit
	/// header.
	PrecommitIsNotCommitDescendant,
	/// The cumulative weight of all votes in the justification is not enough to justify commit
	/// header finalization.
	TooLowCumulativeWeight,
	/// The justification contains extra (unused) headers in its `votes_ancestries` field.
	ExtraHeadersInVotesAncestries,
}

/// Decode justification target.
pub fn decode_justification_target<Header: HeaderT>(
	raw_justification: &[u8],
) -> Result<(Header::Hash, Header::Number), Error> {
	GrandpaJustification::<Header>::decode(&mut &*raw_justification)
		.map(|justification| (justification.commit.target_hash, justification.commit.target_number))
		.map_err(|_| Error::JustificationDecode)
}

/// Verify that justification, that is generated by given authority set, finalizes given header.
pub fn verify_justification<Header: HeaderT>(
	finalized_target: (Header::Hash, Header::Number),
	authorities_set_id: SetId,
	authorities_set: &VoterSet<AuthorityId>,
	justification: &GrandpaJustification<Header>,
) -> Result<(), Error>
where
	Header::Number: finality_grandpa::BlockNumberOps,
{
	// ensure that it is justification for the expected header
	if (justification.commit.target_hash, justification.commit.target_number) != finalized_target {
		return Err(Error::InvalidJustificationTarget)
	}

	let mut chain = AncestryChain::new(&justification.votes_ancestries);
	let mut signature_buffer = Vec::new();
	let mut votes = BTreeSet::new();
	let mut cumulative_weight = 0u64;
	for signed in &justification.commit.precommits {
		// authority must be in the set
		let authority_info = match authorities_set.get(&signed.id) {
			Some(authority_info) => authority_info,
			None => {
				// just ignore precommit from unknown authority as
				// `finality_grandpa::import_precommit` does
				continue
			},
		};

		// check if authority has already voted in the same round.
		//
		// there's a lot of code in `validate_commit` and `import_precommit` functions inside
		// `finality-grandpa` crate (mostly related to reporing equivocations). But the only thing
		// that we care about is that only first vote from the authority is accepted
		if !votes.insert(signed.id.clone()) {
			continue
		}

		// everything below this line can't just `continue`, because state is already altered

		// all precommits must be for block higher than the target
		if signed.precommit.target_number < justification.commit.target_number {
			return Err(Error::PrecommitIsNotCommitDescendant)
		}
		// all precommits must be for target block descendents
		chain = chain
			.ensure_descendant(&justification.commit.target_hash, &signed.precommit.target_hash)?;
		// since we know now that the precommit target is the descendant of the justification
		// target, we may increase 'weight' of the justification target
		//
		// there's a lot of code in the `VoteGraph::insert` method inside `finality-grandpa` crate,
		// but in the end it is only used to find GHOST, which we don't care about. The only thing
		// that we care about is that the justification target has enough weight
		cumulative_weight = cumulative_weight.checked_add(authority_info.weight().0.into()).expect(
			"sum of weights of ALL authorities is expected not to overflow - this is guaranteed by\
				existence of VoterSet;\
				the order of loop conditions guarantees that we can account vote from same authority\
				multiple times;\
				thus we'll never overflow the u64::MAX;\
				qed",
		);
		// verify authority signature
		if !sp_finality_grandpa::check_message_signature_with_buffer(
			&finality_grandpa::Message::Precommit(signed.precommit.clone()),
			&signed.id,
			&signed.signature,
			justification.round,
			authorities_set_id,
			&mut signature_buffer,
		) {
			return Err(Error::InvalidAuthoritySignature)
		}
	}

	// check that there are no extra headers in the justification
	if !chain.unvisited.is_empty() {
		return Err(Error::ExtraHeadersInVotesAncestries)
	}

	// check that the cumulative weight of validators voted for the justification target (or one
	// of its descendants) is larger than required threshold.
	let threshold = authorities_set.threshold().0.into();
	if cumulative_weight >= threshold {
		Ok(())
	} else {
		Err(Error::TooLowCumulativeWeight)
	}
}

/// Votes ancestries with useful methods.
#[derive(RuntimeDebug)]
pub struct AncestryChain<Header: HeaderT> {
	/// Header hash => parent header hash mapping.
	pub parents: BTreeMap<Header::Hash, Header::Hash>,
	/// Hashes of headers that were not visited by `is_ancestor` method.
	pub unvisited: BTreeSet<Header::Hash>,
}

impl<Header: HeaderT> AncestryChain<Header> {
	/// Create new ancestry chain.
	pub fn new(ancestry: &[Header]) -> AncestryChain<Header> {
		let mut parents = BTreeMap::new();
		let mut unvisited = BTreeSet::new();
		for ancestor in ancestry {
			let hash = ancestor.hash();
			let parent_hash = *ancestor.parent_hash();
			parents.insert(hash, parent_hash);
			unvisited.insert(hash);
		}
		AncestryChain { parents, unvisited }
	}

	/// Returns `Err(_)` if `precommit_target` is a descendant of the `commit_target` block and
	/// `Ok(_)` otherwise.
	pub fn ensure_descendant(
		mut self,
		commit_target: &Header::Hash,
		precommit_target: &Header::Hash,
	) -> Result<Self, Error> {
		let mut current_hash = *precommit_target;
		loop {
			if current_hash == *commit_target {
				break
			}

			let is_visited_before = !self.unvisited.remove(&current_hash);
			current_hash = match self.parents.get(&current_hash) {
				Some(parent_hash) => {
					if is_visited_before {
						// `Some(parent_hash)` means that the `current_hash` is in the `parents`
						// container `is_visited_before` means that it has been visited before in
						// some of previous calls => since we assume that previous call has finished
						// with `true`, this also will    be finished with `true`
						return Ok(self)
					}

					*parent_hash
				},
				None => return Err(Error::PrecommitIsNotCommitDescendant),
			};
		}
		Ok(self)
	}
}
