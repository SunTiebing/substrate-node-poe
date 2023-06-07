use crate::{Config, Kitties, Kitty, KittyId, Pallet};
use frame_support::{
	migration::storage_key_iter, pallet_prelude::*, storage::StoragePrefixedMap,
	traits::GetStorageVersion, weights::Weight, Blake2_128Concat,
};

#[derive(
	Encode, Decode, Default, Clone, Copy, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen,
)]
pub struct V0Kitty(pub [u8; 16]);

/// version 0 to version 2
pub fn migrate<T: Config>() -> Weight {
	let on_chain_version = Pallet::<T>::on_chain_storage_version();
	let current_version = Pallet::<T>::current_storage_version();

	// only works for version 0 to 2
	if on_chain_version != 0 {
		return Weight::zero()
	}
	if current_version != 2 {
		return Weight::zero()
	}

	let module = Kitties::<T>::module_prefix();
	let item = Kitties::<T>::storage_prefix();

	// using drain() method to abandon old data
	for (index, kitty) in
		storage_key_iter::<KittyId, V0Kitty, Blake2_128Concat>(module, item).drain()
	{
		// using the first eight dna to be default name
		let mut name: [u8; 8] = [0; 8];
		name.clone_from_slice(&kitty.0[..8]);
		let new_kitty = Kitty { dna: kitty.0, name };
		Kitties::<T>::insert(index, &new_kitty);
	}

	Weight::zero()
}
