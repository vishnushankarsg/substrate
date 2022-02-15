#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::{BoundedVec, inherent::Vec, traits::ConstU32};
use sp_core::H256;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, ensure, inherent::Vec, BoundedVec};
	use frame_system::pallet_prelude::*;
	use super::H256;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn get_something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, H256>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
		SomethingStoredUnsigned(H256),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something_unsigned(origin: OriginFor<T>, something: H256) -> DispatchResult {
			ensure_none(origin)?;

			// Update storage.
			<Something<T>>::put(something.clone());

			// Emit an event.
			Self::deposit_event(Event::SomethingStoredUnsigned(something));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}

	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		/// Validate unsigned call to this module.
		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			// Firstly let's check that we call the right function.
			if let Call::do_something_unsigned { something } = call
			{
				ValidTransaction::with_tag_prefix("TemplateModule")
					// We set base priority to 2**20 and hope it's included before any other
					// transactions in the pool. Next we tweak the priority depending on how much
					// it differs from the current average. (the more it differs the more priority it
					// has).
					.priority(TransactionPriority::max_value())
					// The transaction is only valid for next 5 blocks. After that it's
					// going to be revalidated by the pool.
					.longevity(5)
					// It's fine to propagate that transaction to other peers, which means it can be
					// created even by nodes that don't produce blocks.
					// Note that sometimes it's better to keep it for yourself (if you are the block
					// producer), since for instance in some schemes others may copy your solution and
					// claim a reward.
					.propagate(true)
					.and_provides(&something)
					.build()
			} else {
				InvalidTransaction::Call.into()
			}
		}
	}
}

impl<T> Pallet<T>
where
	// We use `offchain::SendTransactionTypes` for unsigned extrinsic creation and submission.
	T: Config + frame_system::offchain::SendTransactionTypes<Call<T>>,
{

	pub fn submit_processing_result_hash(hash: H256) -> Result<(), ()> {
		let call = Call::<T>::64 { something: hash };
		Ok(())
	}
}
