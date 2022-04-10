
use codec::{Decode, Encode};
use core::{fmt::Debug};
use scale_info::TypeInfo;



pub type Bytes = Vec<u8>;
pub type BalanceOf = u128;
pub type AccountIdOf = [u8; 32];

// #[derive(Clone, Eq, PartialEq, PartialOrd, Encode, Decode)]
// #[derivative(Clone(bound = ""), Eq(bound = ""), PartialEq(bound = ""), Debug(bound = ""))]
// #[codec(encode_bound())]
// #[codec(decode_bound())]
// #[scale_info(bounds(), skip_type_params(Call))]
#[derive(Clone, Eq, PartialEq, Debug, Default, Encode, Decode, TypeInfo)]
pub struct XBIFormat {
	xbi_order: XBIOrder,
	metadata: XBIMetadata,
}
#[derive(Clone, Eq, PartialEq, Debug, Encode, Decode, TypeInfo)]
pub enum XBIOrder {
	CallNative {
		payload: Bytes,
	},
	CallEvm {
		caller: AccountIdOf,
		dest: AccountIdOf,
		value: BalanceOf,
		input: Bytes,
		gas_limit: BalanceOf,
		max_fee_per_gas: Option<BalanceOf>,
		max_priority_fee_per_gas: Option<BalanceOf>,
		nonce: Option<u32>,
		access_list: Option<Bytes>,
	},
	CallWasm {
		caller: AccountIdOf,
		dest: AccountIdOf,
		value: BalanceOf,
		input: Bytes,
		additional_params: Option<Vec<Bytes>>,
	},
	CallCustom {
		caller: AccountIdOf,
		dest: AccountIdOf,
		value: BalanceOf,
		input: Bytes,
		additional_params: Option<Vec<Bytes>>,
	},
	Transfer {
		dest: AccountIdOf,
		value: BalanceOf,
		additional_params: Option<Vec<Bytes>>,
	},
	TransferMulti {
		dest: AccountIdOf,
		currency_id: AccountIdOf,
		value: BalanceOf,
		additional_params: Option<Vec<Bytes>>,
	},
	Result {
		success: bool,
		output: Bytes,
		witness: Bytes,
	},
}

impl Default for XBIOrder {
	fn default() -> Self {
		XBIOrder::CallNative { payload: vec![] }
	}
}

pub type Timeout = u128;

#[derive(Clone, Eq, PartialEq, Debug, Default, Encode, Decode, TypeInfo)]
pub struct ActionNotificationTimeouts {
	action: Timeout,
	notification: Timeout,
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Encode, Decode, TypeInfo)]
pub struct XBIMetadata {
	sent: ActionNotificationTimeouts,
	delivered: ActionNotificationTimeouts,
	executed: ActionNotificationTimeouts,
	// //   - `Sent (action timeout, notification timeout)`
	// //   - `Delivered (action timeout, notification timeout)`
	// //   - `Executed (action timeout, notification timeout)`
	// //   - `Destination / Bridge security guarantees (e.g. in confirmation no for PoW, finality proofs)`
	// //   - `max_exec_cost`: `Balance` : `Maximal cost / fees for execution of delivery`
	// //   - `max_notification_cost`: `Balance` : `Maximal cost / fees per delivering notification`
}
//
// pub enum XBIMetadata {
// 	Sent { action: Timeout, notification: Timeout },
// 	Delivered { action: Timeout, notification: Timeout },
// 	Executed { action: Timeout, notification: Timeout },
// 	// //   - `Sent (action timeout, notification timeout)`
// 	// //   - `Delivered (action timeout, notification timeout)`
// 	// //   - `Executed (action timeout, notification timeout)`
// 	// //   - `Destination / Bridge security guarantees (e.g. in confirmation no for PoW, finality proofs)`
// 	// //   - `max_exec_cost`: `Balance` : `Maximal cost / fees for execution of delivery`
// 	// //   - `max_notification_cost`: `Balance` : `Maximal cost / fees per delivering notification`
// }
