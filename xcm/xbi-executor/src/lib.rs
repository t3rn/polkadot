#![cfg_attr(not(feature = "std"), no_std)]

use xcm::latest::Xcm;

pub mod xbi_format;

pub use pallet::*;

// #[cfg(test)]
// mod mock;
//
// #[cfg(test)]
// mod tests;

#[frame_support::pallet]
pub mod pallet {
	use crate::{xbi_format::XBIFormat, *};

	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn execute_xcm(origin: OriginFor<T>, _xcm: Xcm<Call<T>>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn execute_xbi(origin: OriginFor<T>, _xbi: XBIFormat) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			Ok(())
		}
	}
}
