// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Quadratic Voting Pallet
//!
//! The Quadratic voting pallet is simple implementation of quadratic voting system. 
//! Users are able to propose something by providing hash or short string, 
//! then other users can vote with different amount of votes to express their opinion about proposal. 
//!
//! ## Overview
//!
//! Quadratic voting is a collective decision-making procedure which involves individuals 
//! allocating votes to express the degree of their preferences, rather than just the direction of their preferences. 
//! By doing so, quadratic voting seeks to address issues of voting paradox and majority rule.
//!
//! The quadratic cost function has the unique property that people purchase votes directly proportionally
//! to the strength of their preferences
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! General spending/proposal protocol:
//! - `propose` - Create a proposal for voting using quadratic voting system.
//! - `vote_aye` - Vote for proposal at proposal index with one or more votes.
//! - `vote_nay` - Vote against proposal at proposal index with one or more votes.
//! - `unreserve` - Unreserve tokens after voting period is ended.


#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use frame_support::{ RuntimeDebug, BoundedVec, traits::{Currency,ConstU32 }};
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// An index of a proposal. Just a `u32`.
pub type ProposalIndex = u32;

/// Type alias for `frame_system`'s account id.
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

/// A type alias for the balance type from this pallet's point of view.
pub type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, MaxEncodedLen, TypeInfo)]
pub struct Proposal<AccountId, BlockNumber> {
	/// Number of votes that support proposal.
	pub aye: u128,
	/// Number of votes that are against proposal.
	pub nay: u128,
	/// Hash of the proposal.
	pub hash: BoundedVec<u8, ConstU32<32>>,
	/// Account that created the proposal.
	pub proposer: AccountId,
	/// Block number after which voting period is over.
	pub end: BlockNumber,
}

/// A trait to allow the Quadratic pallet to verify that account setup identity.
pub trait IdentityVerifier<AccountId> {
	fn has_identity(who: &AccountId, fields: u64) -> bool;
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::traits::ReservableCurrency;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// The currency trait.
		type Currency: ReservableCurrency<Self::AccountId>;
		/// Identity checking trait.
		type IdentityVerifier: IdentityVerifier<Self::AccountId>;
		/// Number of blocks that voting is open since the creation of proposal.
		type VotingPeriod: Get<Self::BlockNumber>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Number of proposals that have been made.
	#[pallet::storage]
	#[pallet::getter(fn proposal_count)]
	pub(super) type ProposalCount<T: Config> = StorageValue<_, ProposalIndex, OptionQuery>;

	/// Map of all proposals that have been made.
	#[pallet::storage]
	#[pallet::getter(fn proposals)]
	pub(super) type Proposals<T: Config> = 
		StorageMap<_, Blake2_128Concat, u32, Proposal<T::AccountId, T::BlockNumber>, OptionQuery>;

	/// Map of all tokens reservations that are used for voting.
	#[pallet::storage]
	#[pallet::getter(fn resreved_tokens)]
	pub(super) type ReservedTokens<T: Config> = 
		StorageMap<_, Blake2_128Concat, (u32, T::AccountId), BalanceOf<T>, OptionQuery>;
	

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New proposal has been created with determined index proposal.
		Proposed(u32),
		/// User supported some proposal with his votes.
		VotedAye(u32),
		/// User voted against some proposal with his votes.
		VotedNay(u32),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Proposal at provided index does not exist.
		UnexistingProposal,
		/// Voting period is over.
		VotingEnded,
		/// Voting period is ongoing.
		VotingNotEnded,
		/// User didn't vote so it cannot unreserve any tokens.
		NotVoted,
		/// User does not have identity which is required to be eligable to vote.
		MissingIdentity,
		/// User has tried to vote multiple time on one proposal, but possible only once.
		AlreadyVoted,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// Create a proposal for voting using quadratic voting system.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn propose(
			origin: OriginFor<T>, 
			hash: BoundedVec<u8, ConstU32<32>>
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(T::IdentityVerifier::has_identity(&sender, 0), Error::<T>::MissingIdentity);
			let proposal_index = ProposalCount::<T>::get().unwrap_or_default();
			let block_number = <frame_system::Pallet<T>>::block_number() + T::VotingPeriod::get();
			let proposal = Proposal {
				aye: 0,
				nay: 0,
				hash: hash,
				proposer: sender.clone(),
				end: block_number
			};

			Proposals::<T>::insert(proposal_index, proposal);
			ProposalCount::<T>::put(proposal_index + 1u32);
			Self::deposit_event(Event::Proposed(proposal_index));
			Ok(())
		}

		/// Vote for proposal at proposal index with one or more votes.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn vote_aye(
			origin: OriginFor<T>, 
			proposal_index: u32,
			votes: u128,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// Check that user has identity.
			ensure!(T::IdentityVerifier::has_identity(&sender, 0), Error::<T>::MissingIdentity);
			// Check that proposal that user is voting exists.
			ensure!(Proposals::<T>::contains_key(&proposal_index), Error::<T>::UnexistingProposal);
			// Fetch our proposal from storage.
			let mut p = Self::proposals(proposal_index)
				.expect("Already checked that value exsits; so it is safe to unwrap. QED!");
			// Fetch current block number and check that voting period is still ongoing.
			let now = <frame_system::Pallet<T>>::block_number();
			ensure!(
				now <= p.end,
				Error::<T>::VotingEnded
			);
			// Check if user has already voted.
			ensure!(!ReservedTokens::<T>::contains_key((proposal_index.clone(), sender.clone())), Error::<T>::AlreadyVoted);
			// Calculate amount of tokens that needs to be reserved from users to get desired number of votes.
			let reserved_amount = Self::u128_to_balance(Self::calculate_price(votes));
			// Try to reserve funds, and fail fast if the user can't afford it.
			T::Currency::reserve(&sender, reserved_amount.clone())?;
			// Increment number of supporting votes.
			p.aye = p.aye + votes;
			// Store the updated proposal in storage.
			Proposals::<T>::insert(proposal_index, p);
			// Create new entry for reserved tokens for this voting.
			ReservedTokens::<T>::insert((proposal_index, sender), reserved_amount);
			// Deposit event that voting for proposal with proposal_index happened.
			Self::deposit_event(Event::VotedAye(proposal_index));

			Ok(())
		}

		/// Vote against proposal at proposal index with one or more votes.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn vote_nay(
			origin: OriginFor<T>, 
			proposal_index: u32,
			votes: u128,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// Check that user has identity.
			ensure!(T::IdentityVerifier::has_identity(&sender, 0), Error::<T>::MissingIdentity);
			// Check that proposal that user is voting exists.
			ensure!(Proposals::<T>::contains_key(&proposal_index), Error::<T>::UnexistingProposal);
			// Fetch our proposal from storage.
			let mut p = Self::proposals(proposal_index)
				.expect("Already checked that value exsits; so it is safe to unwrap. QED!");
			// Fetch current block number and check that voting period is still ongoing.
			let now = <frame_system::Pallet<T>>::block_number();
			ensure!(
				now <= p.end,
				Error::<T>::VotingEnded
			);
			// Check if user has already voted.
			ensure!(!ReservedTokens::<T>::contains_key((proposal_index.clone(), sender.clone())), Error::<T>::AlreadyVoted);
			// Calculate amount of tokens that needs to be reserved from users to get desired number of votes.
			let reserved_amount = Self::u128_to_balance(Self::calculate_price(votes));
			// Try to reserve funds, and fail fast if the user can't afford it.
			T::Currency::reserve(&sender, reserved_amount.clone())?;
			// Increment number of votes against proposal.
			p.nay = p.nay + votes;
			// Store the updated proposal in storage.
			Proposals::<T>::insert(proposal_index, p);
			// Create new entry for reserved tokens for this voting.
			ReservedTokens::<T>::insert((proposal_index, sender), reserved_amount);
			// Deposit event that voting against proposal with proposal_index happened.
			Self::deposit_event(Event::VotedNay(proposal_index));
			
			Ok(())
		}

		/// Unreserve tokens after voting period is ended.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn unreserve(
			origin: OriginFor<T>, 
			proposal_index: u32,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// Check that proposal that user is voting exists.
			ensure!(Proposals::<T>::contains_key(&proposal_index), Error::<T>::UnexistingProposal);
			// Fetch our proposal from storage.
			let p = Self::proposals(proposal_index)
				.expect("Already checked that value exsits; so it is safe to unwrap. QED!");
			// Fetch current block number and check that voting period is over.
			let now = <frame_system::Pallet<T>>::block_number();
			ensure!(
				now >= p.end,
				Error::<T>::VotingNotEnded
			);
			// Check that there is mapped tokens entry for this user in storage.
			ensure!(ReservedTokens::<T>::contains_key((proposal_index.clone(), sender.clone())), Error::<T>::NotVoted);
			// Fetch token reservation details from storage.
			let reservation = ReservedTokens::<T>::take((proposal_index, sender.clone()))
				.expect("Already checked that value exsits; so it is safe to unwrap. QED!");
			// Unreserve tokens for the user.
			T::Currency::unreserve(&sender, reservation);

			Ok(())
		}
		
	}

	impl<T: Config> Pallet<T> {
		// Helper function to calculate price in tokens for given amount of votes.
		pub fn calculate_price(amount: u128) -> u128 {
			amount.checked_mul(amount).unwrap()
		}

		// Helper function to convert number of votes (u128) to balance.
		pub fn u128_to_balance(cost: u128) -> BalanceOf<T> {
			TryInto::<BalanceOf::<T>>::try_into(cost).ok().unwrap()
		}	
	}
	
}

