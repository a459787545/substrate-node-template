#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module,
					ensure,
					decl_storage, decl_event, decl_error, dispatch};
use frame_system::ensure_signed;
use crate::dispatch::Vec;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Config> as TemplateModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		// Something get(fn something): Option<u32>;
		// type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
		Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		// SomethingStored(u32, AccountId),
		ClaimCreated(AccountId,Vec<u8>),
		ClaimRevoke(AccountId,Vec<u8>),
		ClaimTransferred(AccountId,AccountId,Vec<u8>),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
		// /// Error names should be descriptive.
		// NoneValue,
		// /// Errors should have helpful documentation associated with them.
		// StorageOverflow,
		ProofAlreadyExist,
		ClaimNotExist,
		NotClaimOwner,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[weight = 0]
		pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&claim),Error::<T>::ProofAlreadyExist);

			Proofs::<T>::insert(&claim,(sender.clone(),frame_system::Module::<T>::block_number()));

			Self::deposit_event(RawEvent::ClaimCreated(sender,claim));

			Ok(())
		}

		#[weight = 0]
		pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim),Error::<T>::ClaimNotExist);

			let (owner,_block_number) = Proofs::<T>::get(&claim);

			ensure!(owner == sender,Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);

			Self::deposit_event(RawEvent::ClaimRevoke(sender,claim));

			Ok(())
		}

		#[weight = 0]
		pub fn transfer_owner_claim(origin, reciever: T::AccountId, claim: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			//
			ensure!(Proofs::<T>::contains_key(&claim),Error::<T>::ClaimNotExist);
			//
			let (owner,_block_number) = Proofs::<T>::get(&claim);
			//
			ensure!(owner == sender,Error::<T>::NotClaimOwner);

			Proofs::<T>::insert(&claim,(reciever.clone(),frame_system::Module::<T>::block_number()));

			Self::deposit_event(RawEvent::ClaimTransferred(sender,reciever,claim));

			Ok(())
		}
	}
}
