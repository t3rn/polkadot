use codec::{Decode, Encode};
use frame_support::dispatch::{DispatchResult, DispatchResultWithPostInfo};
use frame_system::pallet_prelude::OriginFor;
use scale_info::TypeInfo;
use sp_std::{boxed::Box, collections::btree_map::BTreeMap, vec::Vec};
use sp_core::{
    crypto::{ByteArray, KeyTypeId},
    OpaqueMetadata, H160, H256, U256,
};

pub trait Evm<T: frame_system::Config> {
    fn call(
        origin: OriginFor<T>,
        source: H160,
        target: H160,
        input: Vec<u8>,
        value: U256,
        gas_limit: u64,
        max_fee_per_gas: U256,
        max_priority_fee_per_gas: Option<U256>,
        nonce: Option<U256>,
        access_list: Vec<(H160, Vec<H256>)>,
    ) -> DispatchResultWithPostInfo;
}
