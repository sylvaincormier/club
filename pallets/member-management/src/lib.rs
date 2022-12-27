#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
use codec::{Decode, Encode, FullCodec};
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
use scale_info::TypeInfo;

use std::error;
use std::fmt;
#[derive(Debug)]
pub struct CustomError {
	pub message: String,
}

impl error::Error for CustomError {}

impl fmt::Display for CustomError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.message)
	}
}

#[frame_support::pallet]
pub mod pallet {

	use frame_support::traits::Time;
use frame_support::traits::tokens::ExistenceRequirement;
	use frame_support::traits::tokens::WithdrawReasons;
	use frame_support::{pallet, pallet_prelude::*, traits::Currency, IterableStorageMap};
	use frame_system::pallet_prelude::*;
	
	pub(crate) use sp_runtime::offchain::Timestamp;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[derive(Debug, Eq, Clone, Default, Encode, Decode, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())] 
	pub struct Club <T: Config>{
		pub id: u64,
		pub name: String,
		pub owner: T::AccountId,
		pub members: Vec<String>,
		pub fee: u32,
		// pub date: Timestamp,
	}

	impl<T: Config> Club<T> {
		pub fn new(
			id: u64,
			name: String,
			owner: T::AccountId,
			members: Vec<String>,
			fee: u32,
			// date: Timestamp,
		) -> Self {
			// Self { id, name, owner, fee, date }
			Self { id, name, owner, members, fee }
		}

		
	}
	impl<T: Config> PartialEq for Club<T> {
		fn eq(&self, other: &Self) -> bool {
			self.id == other.id
				&& self.name == other.name
				&& self.owner == other.owner
		}
	}
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency: frame_support::traits::Currency<Self::AccountId>;
	}
	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	#[pallet::storage]
	#[pallet::getter(fn something)]

	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::unbounded]
	#[pallet::getter(fn clubs)]
	pub type Clubs<T: Config> =
		StorageDoubleMap<_, Twox64Concat, T::AccountId, Twox64Concat, Vec<u8>, Club<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
		/// parameters. [root, club]
		CreateClub {
			club_id: u64,
			club_name: String,
			club_owner: T::AccountId,
			members: Vec<String>,
			club_fee: u32,
			// date: Timestamp,
		},
		/// parameters. [owner, member]
		AddMember { owner: String, member: String },
		/// parameters. [club, new_owner]
		TransferOwner { club: T::AccountId, new_owner: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
		RootError,
		InsufficientFunds,
		OwnerError,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(1000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<Something<T>>::put(something);
			Self::deposit_event(Event::SomethingStored { something, who });
			Ok(())
		}
		#[pallet::call_index(1)]
		#[pallet::weight(1000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			match <Something<T>>::get() {
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
		#[pallet::call_index(2)]
		#[pallet::weight(1000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn create_club(
			origin: OriginFor<T>,
			root: T::AccountId,
			club_id: u64,
			club_name: String,
			club_owner: T::AccountId,
			members: Vec<String>,
			club_fee: u32,
		) -> DispatchResult {
			let sender = ensure_signed(origin.clone())?;

			ensure!(sender != root, Error::<T>::RootError);
			let iter = &mut <Clubs<T>>::iter();
			// let result = iter.any(|(_key1, _key2, value)| value.name == club_name);
			// check if the club name is unique
			ensure!(
				!iter.any(|(_key1, _key2, value)| value.name == club_name),
				"club name already exists"
			);

			let required_balance: BalanceOf<T> = 100u32.into();
			let sender_: &mut <T as frame_system::Config>::AccountId;

			ensure!(
				T::Currency::free_balance(&sender) >= required_balance,
				<Error<T>>::InsufficientFunds
			);

			let annual_expense: BalanceOf<T> = club_fee.into();
			let reasons = WithdrawReasons::FEE;
			let existence_requirement = ExistenceRequirement::AllowDeath;
			// pay the annual expense
			let _ = T::Currency::withdraw(&sender, annual_expense, reasons, existence_requirement)?;
			// let now = traits::Timestamp::now();
			let members: Vec<String> = Vec::new();
			let club = Club::new(club_id, club_name, club_owner, members, club_fee);
			
			let stub: Vec<u8> = Vec::new();

			 Clubs::<T>::insert(sender, stub, club);
			

			Ok(())
		}
		#[pallet::call_index(3)]
		#[pallet::weight(1000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn add_member(
			origin: OriginFor<T>,
			club_id: u64,
			club_owner: T::AccountId,
			new_member: String,
		) -> DispatchResult {
			let sender = ensure_signed(origin.clone())?;
			let stub: Vec<u8> = Vec::new();
			
			
			
			match Clubs::<T>::get(club_owner, stub) {
				Some(mut club) => {
								if club.id == club_id {
									ensure!(sender == club.owner, Error::<T>::OwnerError);
									club.members.push(new_member);
								}
							}
   				 _ => return Err(Error::<T>::NoneValue.into()),
			}
			

			Ok(())
		}
		
	}
}
