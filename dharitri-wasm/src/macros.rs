/// Handy way of casting to a contract proxy trait.
/// Would make more sense to be in dharitri-wasm-derive, but Rust "cannot export macro_rules! macros from a `proc-macro` crate type currently".
#[macro_export]
macro_rules! contract_call {
	($s:expr, $address:expr, $proxy_trait:ident) => {
		$proxy_trait::<Self::SendApi, BigInt, BigUint>::new($s.send(), $address)
	};
}

/// Getting all imports needed for a smart contract.
#[macro_export]
macro_rules! imports {
	() => {
		use core::ops::{Add, Div, Mul, Rem, Sub};
		use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
		use core::ops::{BitAnd, BitOr, BitXor, Shl, Shr};
		use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};
		use dharitri_wasm::api::{BigIntApi, BigUintApi, CallValueApi, ContractHookApi, SendApi};
		use dharitri_wasm::dharitri_codec::{DecodeError, NestedDecode, NestedEncode, TopDecode};
		use dharitri_wasm::err_msg;
		use dharitri_wasm::dct::*;
		use dharitri_wasm::io::*;
		use dharitri_wasm::non_zero_util::*;
		use dharitri_wasm::storage::mappers::*;
		use dharitri_wasm::types::*;
		use dharitri_wasm::types::{SCResult::Err, SCResult::Ok};
		use dharitri_wasm::{Box, Vec};
	};
}

/// Imports required for deriving serialization and TypeAbi.
#[macro_export]
macro_rules! derive_imports {
	() => {
		use dharitri_wasm::dharitri_codec;
		use dharitri_wasm::dharitri_codec::dharitri_codec_derive::{
			NestedDecode, NestedEncode, TopDecode, TopDecodeOrDefault, TopEncode,
			TopEncodeOrDefault,
		};
		use dharitri_wasm_derive::TypeAbi;
	};
}

/// Compact way of returning a static error message.
#[macro_export]
macro_rules! sc_error {
	($s:expr) => {
		dharitri_wasm::types::SCResult::Err(dharitri_wasm::types::SCError::from($s.as_bytes()))
	};
}

/// Equivalent of the ? operator for SCResult.
#[macro_export]
macro_rules! sc_try {
	($s:expr) => {
		match $s {
			dharitri_wasm::types::SCResult::Ok(t) => t,
			dharitri_wasm::types::SCResult::Err(e) => {
				return dharitri_wasm::types::SCResult::Err(e);
			},
		}
	};
}

/// Allows us to write Solidity style `require!(<condition>, <error_msg>)` and avoid if statements.
///
/// It can only be used in a function that returns `SCResult<_>` where _ can be any type.
///
/// ```rust
/// # use dharitri_wasm::*;
/// # use dharitri_wasm::types::{*, SCResult::Ok};
/// # pub trait ExampleContract<BigInt, BigUint>: dharitri_wasm::api::ContractHookApi<BigInt, BigUint>
/// # where
/// #   BigInt: dharitri_wasm::api::BigIntApi<BigUint> + 'static,
/// #   BigUint: dharitri_wasm::api::BigUintApi + 'static,
/// # {
/// fn only_callable_by_owner(&self) -> SCResult<()> {
///     require!(self.get_caller() == self.get_owner_address(), "Caller must be owner");
///     Ok(())
/// }
/// # }
/// ```
#[macro_export]
macro_rules! require {
	($expression:expr, $error_msg:expr) => {
		if (!($expression)) {
			return sc_error!($error_msg);
		}
	};
}

/// Very compact way of not allowing anyone but the owner to call a function.
///
/// It can only be used in a function that returns `SCResult<_>` where _ can be any type.
///
/// ```rust
/// # use dharitri_wasm::*;
/// # use dharitri_wasm::types::{*, SCResult::Ok};
/// # pub trait ExampleContract<BigInt, BigUint>: dharitri_wasm::api::ContractHookApi<BigInt, BigUint>
/// # where
/// #   BigInt: dharitri_wasm::api::BigIntApi<BigUint> + 'static,
/// #   BigUint: dharitri_wasm::api::BigUintApi + 'static,
/// # {
/// fn only_callable_by_owner(&self) -> SCResult<()> {
///     only_owner!(self, "Caller must be owner");
///     Ok(())
/// }
/// # }
/// ```
#[macro_export]
macro_rules! only_owner {
	($trait_self: expr, $error_msg:expr) => {
		if ($trait_self.get_caller() != $trait_self.get_owner_address()) {
			return sc_error!($error_msg);
		}
	};
}

/// Compact way to represent the BorrowedMutStorage type.
#[macro_export]
macro_rules! mut_storage (
    ($t:ty) => (
        BorrowedMutStorage<T, $t>
    )
);

/// Converts usize to NonZeroUsize or returns SCError.
#[macro_export]
macro_rules! non_zero_usize {
	($input: expr, $error_msg:expr) => {
		if let Some(nz) = NonZeroUsize::new($input) {
			nz
		} else {
			return sc_error!($error_msg);
		}
	};
}
