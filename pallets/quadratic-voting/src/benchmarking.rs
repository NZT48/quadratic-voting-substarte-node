//! Benchmarking setup for pallet-quadratic voting

use super::*;

#[allow(unused)]
use crate::Pallet as QuadraticVoting;
use frame_benchmarking::{benchmarks, account, whitelisted_caller, impl_benchmark_test_suite};
use frame_system::{ RawOrigin, Origin };


benchmarks! {
	/// TODO: Fix benchmarking and implement for other dispetchables.
	propose {
		const SEED: u32 = 0;
		let account: T::AccountId = account("user", 0u32, SEED);
		T::Currency::make_free_balance_be(&account, 100u32.into());
		
		let info = IdentityInfo {
			additional: vec![(data.clone(), data.clone()); num_fields as usize].try_into().unwrap(),
			display: data.clone(),
			legal: data.clone(),
			web: data.clone(),
			riot: data.clone(),
			email: data.clone(),
			pgp_fingerprint: Some([0; 20]),
			image: data.clone(),
			twitter: data,
		}

		crate::mock::Identity::set_identity(RawOrigin::Signed(account.clone()).into(), Box::new(info));
		let proposal = BoundedVec::default();
	}: _(RawOrigin::Signed(account.clone()), proposal)
	verify {
		
	}

	vote_aye {
		const SEED: u32 = 0;
		let account: T::AccountId = account("user", 0u32, SEED);
		T::Currency::make_free_balance_be(&account, 100u32.into());
		let proposal_index = 0;
		let votes = 3;
		
		let info = IdentityInfo {
			additional: vec![(data.clone(), data.clone()); num_fields as usize].try_into().unwrap(),
			display: data.clone(),
			legal: data.clone(),
			web: data.clone(),
			riot: data.clone(),
			email: data.clone(),
			pgp_fingerprint: Some([0; 20]),
			image: data.clone(),
			twitter: data,
		}

		crate::mock::Identity::set_identity(RawOrigin::Signed(account.clone()).into(), Box::new(info));
		let proposal = BoundedVec::default();
	}: _(RawOrigin::Signed(account.clone()), proposal_index, votes)
	verify {
		
	}

	vote_nay {
		const SEED: u32 = 0;
		let account: T::AccountId = account("user", 0u32, SEED);
		T::Currency::make_free_balance_be(&account, 100u32.into());
		let proposal_index = 0;
		let votes = 3;
		
		let info = IdentityInfo {
			additional: vec![(data.clone(), data.clone()); num_fields as usize].try_into().unwrap(),
			display: data.clone(),
			legal: data.clone(),
			web: data.clone(),
			riot: data.clone(),
			email: data.clone(),
			pgp_fingerprint: Some([0; 20]),
			image: data.clone(),
			twitter: data,
		}

		crate::mock::Identity::set_identity(RawOrigin::Signed(account.clone()).into(), Box::new(info));
		let proposal = BoundedVec::default();
	}: _(RawOrigin::Signed(account.clone()), proposal_index, votes)
	verify {
		
	}

}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);

