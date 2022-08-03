use crate as pallet_quadratic_voting;
use frame_support::{
	parameter_types, BoundedVec, assert_ok,
	traits::{ConstU16, ConstU64}
};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
use frame_system::{EnsureRoot};
use pallet_identity::{Data, IdentityInfo};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;

type Block = frame_system::mocking::MockBlock<Test>;

pub type AccountId = u64;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		QuadraticVoting: pallet_quadratic_voting,
		Identity: pallet_identity,
		Balances: pallet_balances,
	}
);

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = u64;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU64<1>;
	type AccountStore = System;
	type WeightInfo = ();
}

parameter_types! {
	pub const BasicDeposit: u64 = 10;
	pub const FieldDeposit: u64 = 10;
	pub const SubAccountDeposit: u64 = 10;
	pub const MaxSubAccounts: u32 = 2;
	pub const MaxAdditionalFields: u32 = 2;
	pub const MaxRegistrars: u32 = 20;
}


impl pallet_identity::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type BasicDeposit = BasicDeposit;
	type FieldDeposit = FieldDeposit;
	type SubAccountDeposit = SubAccountDeposit;
	type MaxSubAccounts = MaxSubAccounts;
	type MaxAdditionalFields = MaxAdditionalFields;
	type MaxRegistrars = MaxRegistrars;
	type Slashed = ();
	type RegistrarOrigin = EnsureRoot<AccountId>;
	type ForceOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
}

pub struct VotingIdentityVerifier;
impl crate::IdentityVerifier<AccountId> for VotingIdentityVerifier {
	fn has_identity(who: &AccountId, fields: u64) -> bool {
		Identity::has_identity(who, fields)
	}
}

parameter_types! {
	pub const VotingPeriod: u64 = 10; // Number of blocks that voting period lasts
}

impl pallet_quadratic_voting::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type IdentityVerifier = VotingIdentityVerifier;
	type VotingPeriod = VotingPeriod;
}


// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	
	// Prefund 3 accounts.
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(1, 50), (2, 50), (3, 50), (4, 50)],
	}
	.assimilate_storage(&mut t)
	.unwrap();


	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| { 
		assert_ok!(Identity::add_registrar(Origin::root(), 1));

		// Create default identity info and set it up for 3 predefined accounts.
		let info = IdentityInfo {
			additional: BoundedVec::default(),
			display: Data::Raw(b"name".to_vec().try_into().unwrap()),
			legal: Data::default(),
			web: Data::Raw(b"website".to_vec().try_into().unwrap()),
			riot: Data::default(),
			email: Data::default(),
			pgp_fingerprint: None,
			image: Data::default(),
			twitter: Data::default(),
		};
		assert_ok!(Identity::set_identity(Origin::signed(1), Box::new(info.clone())));
		assert_ok!(Identity::set_identity(Origin::signed(2), Box::new(info.clone())));
		assert_ok!(Identity::set_identity(Origin::signed(3), Box::new(info.clone())));
		
		System::set_block_number(1)
	});
	ext
}
