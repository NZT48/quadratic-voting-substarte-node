use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn proposal_creation() {
	new_test_ext().execute_with(|| {
		let proposer_with_identity = Origin::signed(1);
		let proposer_without_identity = Origin::signed(4);
		let proposal = BoundedVec::default();

		// Fail if user has no registered identity.
		assert_noop!(QuadraticVoting::propose(proposer_without_identity, proposal.clone()),  Error::<Test>::MissingIdentity);
		// Success if user has registered indetity.
		assert_ok!(QuadraticVoting::propose(proposer_with_identity, proposal));
		// Check that valid event is emitted after succesful proposal.
		System::assert_last_event(
			crate::Event::Proposed(0)
			.into(),
		)
	});
}

#[test]
fn vote_aye_for_proposal() {
	new_test_ext().execute_with(|| {
		let proposer_with_identity = Origin::signed(1);
		let voter_with_identity = Origin::signed(2);
		let voter_late = Origin::signed(3);
		let voter_without_identity = Origin::signed(4);
		let proposal = BoundedVec::default();
		let proposal_index = 0;
		let proposal_index_missing = 1;
		let number_of_votes = 3;

		// Create proposal for voting.
		assert_ok!(QuadraticVoting::propose(proposer_with_identity, proposal));
		// Fail because user has no identity.
		assert_noop!(
			QuadraticVoting::vote_aye(voter_without_identity, proposal_index, number_of_votes), 
			Error::<Test>::MissingIdentity
		);
		// Fail because user voted for unexisting proposal.
		assert_noop!(
			QuadraticVoting::vote_aye(voter_with_identity.clone(), proposal_index_missing, number_of_votes), 
			Error::<Test>::UnexistingProposal
		);
		// Successful voting.
		assert_ok!(QuadraticVoting::vote_aye(voter_with_identity.clone(), proposal_index, number_of_votes));
		// Check that last event is voted aye for proposal.
		System::assert_last_event(
			crate::Event::VotedAye(0)
			.into(),
		);
		// Fail because user has already voted for the proposal.
		assert_noop!(
			QuadraticVoting::vote_aye(voter_with_identity.clone(), proposal_index, number_of_votes), 
			Error::<Test>::AlreadyVoted
		);
		// Set block number to be after the voting period deadline.
		System::set_block_number(20);
		// Fail because voting period is over and user cannot vote.
		assert_noop!(
			QuadraticVoting::vote_aye(voter_late.clone(), proposal_index, number_of_votes), 
			Error::<Test>::VotingEnded
		);
	});
}

#[test]
fn vote_aye_on_multiple_proposals() {
	new_test_ext().execute_with(|| {
			let first_proposer_with_identity = Origin::signed(1);
			let second_proposer_with_identity = Origin::signed(2);
			let voter = Origin::signed(3);
			let proposal = BoundedVec::default();
			let first_proposal_index = 0;
			let second_proposal_index = 1;
			let number_of_votes = 3;

			// Create first and second proposal for voting.
			assert_ok!(QuadraticVoting::propose(first_proposer_with_identity, proposal.clone()));
			assert_ok!(QuadraticVoting::propose(second_proposer_with_identity, proposal));
			// Vote for first proposal and check that expected event is emmited.
			assert_ok!(QuadraticVoting::vote_aye(voter.clone(), first_proposal_index, number_of_votes));
			System::assert_last_event(
				crate::Event::VotedAye(0)
				.into(),
			);
			// Vote for second proposal and check that expected event is emmited.
			assert_ok!(QuadraticVoting::vote_aye(voter.clone(), second_proposal_index, number_of_votes));
			System::assert_last_event(
				crate::Event::VotedAye(1)
				.into(),
			);
	});
}

#[test]
fn vote_nay_for_proposal() {
	new_test_ext().execute_with(|| {
		let proposer_with_identity = Origin::signed(1);
		let voter_with_identity = Origin::signed(2);
		let voter_late = Origin::signed(3);
		let voter_without_identity = Origin::signed(4);
		let proposal = BoundedVec::default();
		let proposal_index = 0;
		let proposal_index_missing = 1;
		let number_of_votes = 3;

		// Create proposal for voting.
		assert_ok!(QuadraticVoting::propose(proposer_with_identity, proposal));
		// Fail because user has no identity.
		assert_noop!(
			QuadraticVoting::vote_nay(voter_without_identity, proposal_index, number_of_votes), 
			Error::<Test>::MissingIdentity
		);
		// Fail because user voted for unexisting proposal.
		assert_noop!(
			QuadraticVoting::vote_nay(voter_with_identity.clone(), proposal_index_missing, number_of_votes), 
			Error::<Test>::UnexistingProposal
		);
		// Successful voting.
		assert_ok!(QuadraticVoting::vote_nay(voter_with_identity.clone(), proposal_index, number_of_votes));
		// Check that last event is voted aye for proposal.
		System::assert_last_event(
			crate::Event::VotedNay(0)
			.into(),
		);
		// Fail because user has already voted for the proposal.
		assert_noop!(
			QuadraticVoting::vote_nay(voter_with_identity.clone(), proposal_index, number_of_votes), 
			Error::<Test>::AlreadyVoted
		);
		// Set block number to be after the voting period deadline.
		System::set_block_number(20);
		// Fail because voting period is over and user cannot vote.
		assert_noop!(
			QuadraticVoting::vote_nay(voter_late.clone(), proposal_index, number_of_votes),
			Error::<Test>::VotingEnded
		);
	});
}

#[test]
fn vote_nay_on_multiple_proposals() {
	new_test_ext().execute_with(|| {
		let first_proposer_with_identity = Origin::signed(1);
		let second_proposer_with_identity = Origin::signed(2);
		let voter = Origin::signed(3);
		let proposal = BoundedVec::default();
		let first_proposal_index = 0;
		let second_proposal_index = 1;
		let number_of_votes = 3;

		// Create first and second proposal for voting.
		assert_ok!(QuadraticVoting::propose(first_proposer_with_identity, proposal.clone()));
		assert_ok!(QuadraticVoting::propose(second_proposer_with_identity, proposal));
		// Vote for first proposal and check that expected event is emmited.
		assert_ok!(QuadraticVoting::vote_nay(voter.clone(), first_proposal_index, number_of_votes));
		System::assert_last_event(
			crate::Event::VotedNay(0)
			.into(),
		);
		// Vote for second proposal and check that expected event is emmited.
		assert_ok!(QuadraticVoting::vote_nay(voter.clone(), second_proposal_index, number_of_votes));
		System::assert_last_event(
			crate::Event::VotedNay(1)
			.into(),
		);
	});
}

#[test]
fn vote_aye_then_nay_on_same_proposal() {
	new_test_ext().execute_with(|| {
			let proposer = Origin::signed(1);
			let voter = Origin::signed(2);
			let proposal = BoundedVec::default();
			let proposal_index = 0;
			let number_of_votes = 3;

			// Create proposal for voting.
			assert_ok!(QuadraticVoting::propose(proposer, proposal));
			// Vote for the proposal and check that correct event is emmited.
			assert_ok!(QuadraticVoting::vote_aye(voter.clone(), proposal_index, number_of_votes));
			System::assert_last_event(
				crate::Event::VotedAye(0)
				.into(),
			);
			// Fail because user already voted aye on same proposal.
			assert_noop!(
				QuadraticVoting::vote_nay(voter.clone(), proposal_index, number_of_votes), 
				Error::<Test>::AlreadyVoted
			);
			// Check that last event didn't changed.
			System::assert_last_event(
				crate::Event::VotedAye(0)
				.into(),
			);
	});
}

#[test]
fn unreserve_tokens_after_voting() {
	new_test_ext().execute_with(|| {
			let proposer = Origin::signed(1);
			let voter = Origin::signed(2);
			let not_voter = Origin::signed(3);
			let proposal = BoundedVec::default();
			let proposal_index = 0;
			let proposal_index_unexisting = 1;
			let number_of_votes = 3;

			// Create proposal for voting.
			assert_ok!(QuadraticVoting::propose(proposer, proposal));
			// Vote to support some proposal and check emitted event.
			assert_ok!(QuadraticVoting::vote_aye(voter.clone(), proposal_index, number_of_votes));
			System::assert_last_event(
				crate::Event::VotedAye(0)
				.into(),
			);
			// Fail because user tried to unreserve tokens from unexisting proposal.
			assert_noop!(QuadraticVoting::unreserve(voter.clone(), proposal_index_unexisting), Error::<Test>::UnexistingProposal);
			// Fail because user tried to unreserve tokens while the voting is ongoing
			assert_noop!(QuadraticVoting::unreserve(voter.clone(), proposal_index), Error::<Test>::VotingNotEnded);
			// Set block number to be after the voting ended.
			System::set_block_number(20);
			// Fail because user tried to unreserve tokens but the user didn't voted at all.
			assert_noop!(QuadraticVoting::unreserve(not_voter.clone(), proposal_index), Error::<Test>::NotVoted);
			// Successful unlocking of tokens.
			assert_ok!(QuadraticVoting::unreserve(voter.clone(), proposal_index));
			
	});
}