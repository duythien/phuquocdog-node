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

#[frame_support::pallet]
pub mod pallet {
	use sp_std::vec::Vec;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	pub type Amount = u128;
	pub type CurrencyId = Vec<u8>;

	#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
	pub struct CurrencyInfo {
		pub decimal: u16,
		pub rpc_url: Vec<u8>,
		pub native: bool,
	}
	#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
	pub struct CurrencyBalance {
		pub free: u128,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	#[pallet::storage]
	#[pallet::getter(fn accounts)]
	pub type Accounts<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		CurrencyId,
		CurrencyBalance
	>;
	
	#[pallet::storage]
	#[pallet::getter(fn currency)]
	pub type Currency<T: Config> = StorageMap<_, Blake2_128Concat, CurrencyId, CurrencyInfo>;

	#[pallet::storage]
	#[pallet::getter(fn currencies)]
	pub(super) type Currencies<T: Config> = StorageValue<_, Vec<Vec<u8>>>;
	

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		/// Currency add success. [currency_id, who]
		NewCurrencyAdded(CurrencyId, T::AccountId),
		/// Update balance success. [currency_id, amount, to, who]
		BalanceUpdated(CurrencyId, Amount, T::AccountId, T::AccountId),
		/// Currency transfer success. [currency_id, amount, to, who]
		Transferred(CurrencyId, Amount, T::AccountId, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		CurrencyExist,
		CurrencyNotExist,
		BadOrigin,
		InsufficientFunds,
		OverflowAmount,
		InsufficientAmount,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn add_currency(
			origin: OriginFor<T>,
			currency_id: Vec<u8>,
			decimal: u16,
			rpc_url: Vec<u8>,
			native: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let currency = Self::currency(currency_id.clone());
			let currency_info = CurrencyInfo { decimal, rpc_url, native };

			ensure!(currency.is_none(), Error::<T>::CurrencyExist);

			let mut currencies = Self::currencies().unwrap_or_default();
			currencies.push(currency_id.clone());

			Currency::<T>::insert(&currency_id, &currency_info);
			Currencies::<T>::put(currencies);

			Self::deposit_event(Event::NewCurrencyAdded(currency_id, who));

			Ok(())
		}
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn update_balance(
			origin: OriginFor<T>,
			to: T::AccountId,
			currency_id: CurrencyId,
			amount: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let currency = Self::currency(currency_id.clone());

			ensure!(currency.is_some(), Error::<T>::CurrencyNotExist);
			ensure!(amount > 0, Error::<T>::InsufficientAmount);

			let receiver_balance =
				Self::accounts(&to, &currency_id).unwrap_or(CurrencyBalance { free: 0 });
			let updated_to_balance =
				receiver_balance.free.checked_add(amount).ok_or(Error::<T>::OverflowAmount)?;

			Accounts::<T>::insert(&to, &currency_id, CurrencyBalance { free: updated_to_balance });

			Self::deposit_event(Event::BalanceUpdated(currency_id, amount, to, who));

			Ok(())
		}

		/// Transfer tokens from one account to another
		//#[pallet::weight(T::WeightInfo::transfer(currency_id.len() as u32 ))]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			currency_id: CurrencyId,
			amount: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let currency = Self::currency(currency_id.clone());

			ensure!(currency.is_some(), Error::<T>::CurrencyNotExist);
			ensure!(who != to, Error::<T>::BadOrigin);
			ensure!(amount > 0, Error::<T>::InsufficientAmount);

			let sender_balance =
				Self::accounts(&who, &currency_id).unwrap_or(CurrencyBalance { free: 0 });
			let receiver_balance =
				Self::accounts(&to, &currency_id).unwrap_or(CurrencyBalance { free: 0 });

			// Calculate new balances
			let updated_from_balance =
				sender_balance.free.checked_sub(amount).ok_or(Error::<T>::InsufficientFunds)?;
			let updated_to_balance =
				receiver_balance.free.checked_add(amount).ok_or(Error::<T>::OverflowAmount)?;

			// Write new balances to storage
			Accounts::<T>::insert(
				&who,
				&currency_id,
				CurrencyBalance { free: updated_from_balance },
			);
			Accounts::<T>::insert(&to, &currency_id, CurrencyBalance { free: updated_to_balance });

			Self::deposit_event(Event::Transferred(currency_id, amount, to, who));

			Ok(())
		}
	}
}
