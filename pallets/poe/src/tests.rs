use crate::{Error, mock::*};
use frame_support::{assert_ok,assert_err, assert_noop};

#[test]
fn given_proof_not_yet_claimed_should_be_ok_when_create_claim() {
	new_test_ext().execute_with(|| {
		let proof = vec![42,2];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), proof));
	});
}

#[test]
fn given_proof_already_claimed_should_be_error_when_create_claim() {
	new_test_ext().execute_with(|| {
		assert_ok!(PoeModule::create_claim(Origin::signed(1), vec![42]));
		assert_err!(PoeModule::create_claim(Origin::signed(1), vec![42]),Error::<Test>::ProofAlreadyClaimed);
	});
}

#[test]
fn given_proof_not_existed_should_be_error_when_revoke_claim() {
	new_test_ext().execute_with(|| {
		assert_err!(PoeModule::revoke_claim(Origin::signed(1), vec![42]),Error::<Test>::NoSuchProof);
	});
}

#[test]
fn given_not_proof_owner_should_be_error_when_revoke_claim() {
	new_test_ext().execute_with(|| {
		assert_ok!(PoeModule::create_claim(Origin::signed(1), vec![42]));
		assert_err!(PoeModule::revoke_claim(Origin::signed(2), vec![42]),Error::<Test>::NotProofOwner);
	});
}

#[test]
fn should_be_ok_when_revoke_claim() {
	new_test_ext().execute_with(|| {
		assert_ok!(PoeModule::create_claim(Origin::signed(1), vec![42]));
		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), vec![42]));
	});
}

#[test]
fn given_proof_not_existed_should_be_error_when_transfer_proof() {
	new_test_ext().execute_with(|| {
		assert_err!(PoeModule::transfer_proof(Origin::signed(1),2, vec![42]),Error::<Test>::NoSuchProof);
	});
}

#[test]
fn given_not_proof_owner_should_be_error_when_transfer_proof() {
	new_test_ext().execute_with(|| {
		assert_ok!(PoeModule::create_claim(Origin::signed(1), vec![42]));
		assert_err!(PoeModule::transfer_proof(Origin::signed(2),3, vec![42]),Error::<Test>::NotProofOwner);
	});
}

#[test]
fn should_be_ok_when_transfer_proof() {
	new_test_ext().execute_with(|| {
		assert_ok!(PoeModule::create_claim(Origin::signed(1), vec![42]));
		assert_ok!(PoeModule::transfer_proof(Origin::signed(1),2, vec![42]));
	});
}

