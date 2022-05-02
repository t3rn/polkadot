// Copyright 2020 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

use crate::traits::{
	ClaimAssets, ConvertOrigin, DropAssets, FilterAssetLocation, InvertLocation, OnResponse,
	ShouldExecute, TransactAsset, VersionChangeNotifier, WeightBounds, WeightTrader,
};
use frame_support::{
	dispatch::{Dispatchable, Parameter},
	weights::{GetDispatchInfo, PostDispatchInfo},
};
use xcm::latest::SendXcm;


// - `call(instance_id/bridge_id)`: `modifications`
//   - `call_native`: `trigger Scale encoded native call`
//     - `payload: Bytes`
//   - `call_evm`:  `trigger smart contract call`
//     - `caller: AccountId`
//     - `dest: AccountId`
//     - `value: Balance`
//     - `input: Bytes`
//     - `gas_limit: Balance`
//     - `max_fee_per_gas: Option<Balance>`
//     - `max_priority_fee_per_gas: Option<Balance>`
//     - `nonce: Option<u32>`
//     - `access_list: Option<Bytes>`
//   - `call_wasm`: `trigger smart contract call`
//     - `caller: AccountId`
//     - `dest: MultiAddress<AccountId, ()>`
//     - `value: Balance`
//     - `input: Bytes`
//     - `additional_params: Option<Vec<ABIType>>`
//   - `call_custom`
//     - `caller: AccountId`
//     - `dest: MultiAddress<AccountId, ()>`
//     - `value: Balance`
//     - `input: Bytes`
//     - `additional_params: Option<Vec<ABIType>>`
// - `query`: `access state / read-only` // worth making a batch/related call.
//   - `query_evm`:
//     - `address: AccountId`
//     - `storage_key: Bytes`
//       - read("winner")
//       - read(\0)
//   - `query_wasm`:
// - `result`: `(success|failure, <output|failruedetails>, <dest_parachain_witness>)`
// - `metadata`: `Lifecycle status notifications`
//   - `Sent (action timeout, notification timeout)`
//   - `Delivered (action timeout, notification timeout)`
//   - `Executed (action timeout, notification timeout)`
//   - `Destination / Bridge security guarantees (e.g. in confirmation no for PoW, finality proofs)`
//   - `max_exec_cost`: `Balance` : `Maximal cost / fees for execution of delivery`
//   - `max_notification_cost`: `Balance` : `Maximal cost / fees per delivering notification`

pub type Bytes = Vec<u8>;

pub enum XBIFormat<T: frame_system::Config> {
	CallNative { payload: Bytes },
	CallEvm {
		caller: T::AccountId,
    	dest: T::AccountId,
    	value: BalanceOf<T>,
    	input: Bytes,
    	gas_limit: BalanceOf<T>,
		max_fee_per_gas: Option<BalanceOf<T>>,
		max_priority_fee_per_gas: Option<BalanceOf<T>>,
		nonce: Option<u32>,
		access_list: Option<Bytes>,
	},
	CallWasm {
		caller: T::AccountId,
		dest: T::AccountId,
		value: BalanceOf<T>,
		input: Bytes,
		additional_params:  Option<Vec<Bytes>>,
	},
	CallCustom {
		caller: T::AccountId,
		dest: T::AccountId,
		value: BalanceOf<T>,
		input: Bytes,
		additional_params:  Option<Vec<Bytes>>,
	},
	Transfer {
		dest: T::AccountId,
		value: BalanceOf<T>,
		additional_params:  Option<Vec<Bytes>>,
	},
	TransferMulti {
		dest: T::AccountId,
		currency_id: T::AccountId,
		value: BalanceOf<T>,
		additional_params:  Option<Vec<Bytes>>,
	},
	Result {
		success: bool,
		output: Bytes,
		witness: Bytes,
	},
}

pub type Timeout = u128;

pub enum XBIMetadata<T: frame_system::Config> {
	Sent {
		action: Timeout,
		notification: Timeout,
	},
	Delivered {
		action: Timeout,
		notification: Timeout,
	},
	Executed {
		action: Timeout,
		notification: Timeout,
	},
	// //   - `Sent (action timeout, notification timeout)`
	// //   - `Delivered (action timeout, notification timeout)`
	// //   - `Executed (action timeout, notification timeout)`
	// //   - `Destination / Bridge security guarantees (e.g. in confirmation no for PoW, finality proofs)`
	// //   - `max_exec_cost`: `Balance` : `Maximal cost / fees for execution of delivery`
	// //   - `max_notification_cost`: `Balance` : `Maximal cost / fees per delivering notification`
}

/// The trait to parameterize the `XcmExecutor`.
pub trait Config {
	/// The outer call dispatch type.
	type Call: Parameter + Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo;

	/// How to send an onward XCM message.
	type XcmSender: SendXcm;

	/// How to withdraw and deposit an asset.
	type AssetTransactor: TransactAsset;

	/// How to get a call origin from a `OriginKind` value.
	type OriginConverter: ConvertOrigin<<Self::Call as Dispatchable>::Origin>;

	/// Combinations of (Location, Asset) pairs which we trust as reserves.
	type IsReserve: FilterAssetLocation;

	/// Combinations of (Location, Asset) pairs which we trust as teleporters.
	type IsTeleporter: FilterAssetLocation;

	/// Means of inverting a location.
	type LocationInverter: InvertLocation;

	/// Whether we should execute the given XCM at all.
	type Barrier: ShouldExecute;

	/// The means of determining an XCM message's weight.
	type Weigher: WeightBounds<Self::Call>;

	/// The means of purchasing weight credit for XCM execution.
	type Trader: WeightTrader;

	/// What to do when a response of a query is found.
	type ResponseHandler: OnResponse;

	/// The general asset trap - handler for when assets are left in the Holding Register at the
	/// end of execution.
	type AssetTrap: DropAssets;

	/// The handler for when there is an instruction to claim assets.
	type AssetClaims: ClaimAssets;

	/// How we handle version subscription requests.
	type SubscriptionService: VersionChangeNotifier;
}
