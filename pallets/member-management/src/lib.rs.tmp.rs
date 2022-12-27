
// 1. Set up the pallet project files and folder structure.
// 2. Create the pallet configuration file.
// 3. Create the pallet trait and the associated types.
// 4. Implement the storage logic for the pallet.
// 5. Implement the pallet entry points.
// 6. Create the pallet unit tests.
// 7. Compile and run the pallet.
// 8. Create the pallet documentation.
// 9. Submit the pallet to the Substrate pallet repository.




#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use serde::{Serialize, Deserialize};

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Club {
//     pub name: String,
//     pub owner: String,
//     pub members: Vec<String>,
//     pub fee: u32,
//     pub max_members_for_year: u32,
// }
// pub struct Club<T> {
//     pub name: String,
//     pub owner: String,
//     pub members: Vec<T>,
//     pub fee: u32,
//     pub max_members_for_year: u32,
// }



// impl<T> Club<T> {
//     pub fn new(name: String, owner: String, fee: u32, max_members_for_year: u32) -> Self {
//         Self {
//             name,
//             owner,
//             members: vec![],
//             fee,
//             max_members_for_year,
//         }
//     }

//     pub fn add_member(&mut self, new_member: T) -> Result<(), String> {
//         if self.members.len() < self.max_members_for_year as usize {
//             self.members.push(new_member);
//             Ok(())
//         } else {
//             Err(String::from("Max number of members exceeded"))
//         }
//     }

//     pub fn transfer_ownership(&mut self, new_owner: String) {
//         self.owner = new_owner;
//     }

//     pub fn set_annual_expense(&mut self, annual_expense: i32) {
//         if annual_expense > 0 && annual_expense <= 100 {
//             self.fee = annual_expense as u32;
//         }
//     }
// }

// impl Club {
//     pub fn new(name: String, owner: String, fee: u32, max_members_for_year: u32) -> Self {
//         Self {
//             name,
//             owner,
//             members: vec![],
//             fee,
//             max_members_for_year,
//         }
//     }

//     pub fn add_member(&mut self, new_member: String) -> Result<(), String> {
//         if self.members.len() < self.max_members_for_year as usize {
//             self.members.push(new_member);
//             Ok(())
//         } else {
//             Err(String::from("Max number of members exceeded"))
//         }
//     }

//     pub fn transfer_ownership(&mut self, new_owner: String) {
//         self.owner = new_owner;
//     }

//     pub fn set_annual_expense(&mut self, annual_expense: i32) {
//         if annual_expense > 0 && annual_expense <= 100 {
//             self.fee = annual_expense as u32;
//         }
//     }

// }



// pub struct Club {
//     pub name: String,
//     pub owner: String,
//     pub members: Vec<String>,
//     pub fee: u32,
//     pub max_members_for_year: u32,
// }

// frame_support::decl_storage! {
//     trait Store for Club as MemberManagement {
//         Clubs get(fn clubs): map hasher(blake2_128_concat) T::AccountId => Club;
//         ClubFee get(fn club_fee): u32;
//         MaxMembers get(fn max_members): u32;
//     }
// }

// impl<T: Trait> Module<T> {
//     pub fn insert_club(account_id: T::AccountId, club: Club) {
//         Clubs::insert(&account_id, club);
//     }

//     pub fn remove_club(account_id: T::AccountId) -> Option<Club> {
//         Clubs::take(&account_id)
//     }

//     pub fn get_club(account_id: T::AccountId)

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	// pub struct Club<T> {
	// 	pub name: String,
	// 	pub owner: String,
	// 	pub members: Vec<T>,
	// 	pub fee: u32,
	// 	pub max_members_for_year: u32,
	// }

	
	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]

	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;
	
		
	// - A mapping of root accounts to clubs they have created:
	
	// pub type RootAccount: map hasher(blake2_128_concat) T::AccountId => Clubs<T::AccountId, T::Balance>;
	pub struct Club {
		pub name: String,
		pub owner: String,
		pub members: Vec<String>,
		pub fee: u32,
		pub max_members_for_year: u32,
	}
	
	#[pallet::storage]
	#[pallet::getter(fn some_map)]
	type clubs: Vec<Club>;
	pub(super) type RootAccount<T: Config> = StorageMap<
		_, 
		Blake2_128Concat, T::AccountId, 
		Clubs<T::AccountId, T::Balance>, 
		ValueQuery
	>;	
	// // - A mapping of club owners to their associated clubs: 
	// #[pallet::storage] pub ClubOwners: map hasher(blake2_128_concat) T::AccountId => Vec<Clubs<T::AccountId, T::Balance>>;

	// // - A mapping of tokens to the amount of token they have associated with a particular club:
	// #[pallet::storage] pub Tokens: map hasher(blake2_128_concat) T::TokenId => T::Balance;

	// // - A mapping of accounts to their membership status within a club: 
	// #[pallet::storage] pub Accounts: map hasher(blake2_128_concat) T::AccountId => Membership<T::AccountId>;

	// // - A mapping of accounts to their membership expiration date:
	// #[pallet::storage] pub Expires: map hasher(blake2_128_concat) T::AccountId => T::Moment;

	// // - A mapping of clubs to the annual expense of club membership:
	// #[pallet::storage] pub AnnualExpenses: map hasher(blake2_128_concat) Clubs<T::AccountId, T::Balance> => T::Balance;
	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
		CreateClub { root: T::AccountId, club: Club },
		AddMember { owner: T::AccountId, member: T::AccountId },
		TransferOwner { club: T::AccountId, new_owner: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored { something, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}

// // pallet storage items
// // Root account to club mapping
// pub type RootAccountToClubMapping<T> = BTreeMap<T::AccountId, Club<T>>;

// // Club storage items
// pub type Club<T> = (T::AccountId, T::Balance, T::BlockNumber);

// // Membership storage items
// pub type Membership<T> = (T::AccountId, Club<T>, T::BlockNumber, T::Balance, T::BlockNumber);

// Dispatchable Functions

// // Create a new club
// pub fn create_club<T: Trait>(origin: T::Origin, club_name: Vec<u8>, annual_expense: T::Balance) {
//     // check if the caller is root
//     let caller = ensure_signed(origin)?;
//     ensure!(Self::is_root(&caller), "only root can create a new club");

//     // check if the club name is unique
//     ensure!(!Self::club_exists(&club_name), "club name already exists");

//     // check if the caller has enough balance
//     ensure!(T::Currency::can_pay(&caller, &annual_expense), "not enough balance to create a new club");

//     // pay the annual expense 
//     let _ = T::Currency::withdraw(&caller, annual_expense, WithdrawReasons::create_club, None)?;

//     // create the new club
//     let new_club = Self::new_club(&caller, &club_name, &annual_expense);

//     // add the new club to root account to club mapping
//     Self::add_club_to_mapping(&caller, new_club);

//     Ok(())
// }

// // Transfer the club ownership
// pub fn transfer_club_ownership<T: Trait>(origin: T::Origin, club_name: Vec<u8>, new_owner: T::AccountId) {
//     // check if the caller is root 
//     let caller = ensure_signed(origin)?;
//     ensure!(Self::is_root(&caller), "only root can transfer the club ownership");

//     // check if the club exists
//     ensure!(Self::club_exists(&club_name), "club not found");

//     // get the club owner
//     let (old_owner, _, _) = Self::get_club(&club_name).ok_or("club not found")?;

//     // transfer the club ownership
//     Self::transfer_club_ownership(&old_owner, &new_owner, &club_name);

//     Ok(())
// }

// // Add a member to the club
// pub fn add_member<T: Trait>(origin: T::Origin, club_name: Vec<u8>, member_id: T::AccountId) {
//     // check if the caller is club owner
//     let caller = ensure_signed(origin)?;
//     ensure!(Self::is_club_owner(&caller, &club_name), "only club owner can add a member to the club");

//     // check if the club exists
//     let (owner, club_expense, _) = Self::get_club(&club_name).ok_or("club not found")?;

//     // check if the account has enough balance to pay the annual expense
//     let max_duration = T::BlockNumber::sa(100);
//     let membership_expense = Self::calculate_membership_expense(&club_expense, max_duration);
//     ensure!(T::Currency::can_pay(&member_id, &membership_expense), "not enough balance to pay the annual expense");

//     // pay the annual expense 
//     let _ = T::Currency::withdraw(&member_id, membership_expense, WithdrawReasons::join_club, None)?;

//     // add the member to the club
//     Self::add_member_to_club(&owner, &member_id, &club_name, max_duration);

//     Ok(())
// }

// // Renew the membership
// pub fn renew_membership<T: Trait>(origin: T::Origin, club_name: Vec<u8>, member_id: T::AccountId, duration: T::BlockNumber) {
//     // check if the caller is club owner
//     let caller = ensure_signed(origin)?;
//     ensure!(Self::is_club_owner(&caller, &club_name), "only club owner can renew the membership");

//     // check if the club exists
//     ensure!(Self::club_exists(&club_name), "club not found");

//     // check if the account has enough balance to pay the annual expense
//     let (owner, club_expense, _) = Self::get_club(&club_name).ok_or("club not found")?;
//     let membership_expense = Self::calculate_membership_expense(&club_expense, duration);
//     ensure!(T::Currency::can_pay(&member_id, &membership_expense), "not enough balance to pay the annual expense");

//     // pay the annual expense 
//     let _ = T::Currency::withdraw(&member_id, membership_expense, WithdrawReasons::renew_membership, None)?;

//     // renew the membership
//     Self::renew_membership(&owner, &member_id, &club_name, duration);

//     Ok(())
// }

