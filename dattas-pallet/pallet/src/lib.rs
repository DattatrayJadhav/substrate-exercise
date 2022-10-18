// Copyright (C) 2019-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0

//! # Dattas Pallet
//!
//! - [`Config`]
//! - [`Call`]
//!
//! ## Overview
//!
//! Dattas is an example pallet for keeping track of account names on-chain. It makes no effort to
//! create a name hierarchy, be a DNS replacement or provide reverse lookups. 
//! 
//! ### Functions
//! * `set_name` - Set the associated name of an account; a small deposit is reserved if not already
//!   taken.
//! * `clear_name` - Remove an account's associated name; the deposit is returned.
//! * `kill_name` - Forcibly remove the associated name; the deposit is lost.
//! * `balanceOf` - get the balance of the user.
//! * `ReservedNameOf` - get the associated name with reserved deposite.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::{Currency, OnUnbalanced, ReservableCurrency};
pub use pallet::*;
use sp_runtime::traits::{StaticLookup, Zero};
use sp_std::prelude::*;

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
type NegativeImbalanceOf<T> =
	<<T as Config>::Currency as Currency<AccountIdOf<T>>>::NegativeImbalance;
type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The currency trait.
		type Currency: ReservableCurrency<Self::AccountId>;

		/// Reservation fee.
		#[pallet::constant]
		type ReservationFee: Get<BalanceOf<Self>>;

		/// What to do with slashed funds.
		type Slashed: OnUnbalanced<NegativeImbalanceOf<Self>>;

		/// The origin which may forcibly set or remove a name. Root can always do this.
		type ForceOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// The minimum length a name may be.
		#[pallet::constant]
		type MinLength: Get<u32>;

		/// The maximum length a name may be.
		#[pallet::constant]
		type MaxLength: Get<u32>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A name was set.
		NameSet { who: T::AccountId },
		/// A name was forcibly set.
		NameForced { target: T::AccountId },
		/// A name was changed.
		NameChanged { who: T::AccountId },
		/// A name was cleared, and the given balance returned.
		NameCleared { who: T::AccountId, deposit: BalanceOf<T> },
		/// A name was removed and the given balance slashed.
		NameKilled { target: T::AccountId, deposit: BalanceOf<T> },
	}

	/// Error for the nicks pallet.
	#[pallet::error]
	pub enum Error<T> {
		TooShort,
		TooLong,
		Unnamed,
	}

	/// The lookup table for names.
	#[pallet::storage]
	pub(super) type ReservedNameOf<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, (BoundedVec<u8, T::MaxLength>, BalanceOf<T>)>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Set an account's name. The name should be a UTF-8-encoded string by convention, though we don't check it.
		/// The name may not be more than `T::MaxLength` bytes, nor less than `T::MinLength` bytes.
		/// If the account doesn't already have a name, then a fee of `ReservationFee` is reserved in the account.
		/// The dispatch origin for this call must be _Signed_.
		#[pallet::weight(50_000_000)]
		pub fn set_name(origin: OriginFor<T>, name: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let bounded_name: BoundedVec<_, _> =
				name.try_into().map_err(|_| Error::<T>::TooLong)?;
			ensure!(bounded_name.len() >= T::MinLength::get() as usize, Error::<T>::TooShort);

			let deposit = if let Some((_, deposit)) = <ReservedNameOf<T>>::get(&sender) {
				Self::deposit_event(Event::<T>::NameChanged { who: sender.clone() });
				deposit
			} else {
				let deposit = T::ReservationFee::get();
				T::Currency::reserve(&sender, deposit)?;
				Self::deposit_event(Event::<T>::NameSet { who: sender.clone() });
				deposit
			};

			<ReservedNameOf<T>>::insert(&sender, (bounded_name, deposit));
			Ok(())
		}

		/// Clear an account's name and return the deposit. Fails if the account was not named.
		/// The dispatch origin for this call must be _Signed_.
		#[pallet::weight(70_000_000)]
		pub fn clear_name(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let deposit = <ReservedNameOf<T>>::take(&sender).ok_or(Error::<T>::Unnamed)?.1;

			let err_amount = T::Currency::unreserve(&sender, deposit);
			debug_assert!(err_amount.is_zero());

			Self::deposit_event(Event::<T>::NameCleared { who: sender, deposit });
			Ok(())
		}

		/// Remove an account's name and take charge of the deposit.
		/// Fails if `target` has not been named. The deposit is dealt with through `T::Slashed` imbalance handler.
		/// The dispatch origin for this call must match `T::ForceOrigin`.
		#[pallet::weight(70_000_000)]
		pub fn kill_name(origin: OriginFor<T>, target: AccountIdLookupOf<T>) -> DispatchResult {
			T::ForceOrigin::ensure_origin(origin)?;

			// Figure out who we're meant to be clearing.
			let target = T::Lookup::lookup(target)?;
			// Grab their deposit (and check that they have one).
			let deposit = <ReservedNameOf<T>>::take(&target).ok_or(Error::<T>::Unnamed)?.1;
			// Slash their deposit from them.
			T::Slashed::on_unbalanced(T::Currency::slash_reserved(&target, deposit).0);

			Self::deposit_event(Event::<T>::NameKilled { target, deposit });
			Ok(())
		}

		/// Set a third-party account's name with no deposit.
		/// No length checking is done on the name.
		/// The dispatch origin for this call must match `T::ForceOrigin`.
		#[pallet::weight(70_000_000)]
		pub fn force_name(
			origin: OriginFor<T>,
			target: AccountIdLookupOf<T>,
			name: Vec<u8>,
		) -> DispatchResult {
			T::ForceOrigin::ensure_origin(origin)?;

			let bounded_name: BoundedVec<_, _> =
				name.try_into().map_err(|_| Error::<T>::TooLong)?;
			let target = T::Lookup::lookup(target)?;
			let deposit = <ReservedNameOf<T>>::get(&target).map(|x| x.1).unwrap_or_else(Zero::zero);
			<ReservedNameOf<T>>::insert(&target, (bounded_name, deposit));

			Self::deposit_event(Event::<T>::NameForced { target });
			Ok(())
		}

		pub fn balanceOf(
			who: &<T as SystemConfig>::AccountId
		) -> Self::BalanceOf {
			Pallet::<T, I>::balanceOf(who)
		}
	}
}
