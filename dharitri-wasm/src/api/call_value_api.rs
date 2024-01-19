use super::{BigUintApi, ErrorApi};
use crate::err_msg;
use crate::types::{DctTokenType, TokenIdentifier};

pub trait CallValueApi<BigUint>: ErrorApi + Sized
where
	BigUint: BigUintApi + 'static,
{
	fn check_not_payable(&self);

	/// Retrieves the MOA call value from the VM.
	/// Will return 0 in case of an DCT transfer (cannot have both MOA and DCT transfer simultaneously).
	fn moa_value(&self) -> BigUint;

	/// Retrieves the DCT call value from the VM.
	/// Will return 0 in case of an MOA transfer (cannot have both MOA and DCT transfer simultaneously).
	fn dct_value(&self) -> BigUint;

	/// Returns the call value token identifier of the current call.
	/// The identifier is wrapped in a TokenIdentifier object, to hide underlying logic.
	///
	/// A note on implementation: even though the underlying api returns an empty name for MOA,
	/// but the MOA TokenIdentifier is serialized as `MOA`.
	fn token(&self) -> TokenIdentifier;

	/// Returns the nonce of the received DCT token.
	/// Will return 0 in case of MOA or fungible DCT transfer.
	fn dct_token_nonce(&self) -> u64;

	/// Returns the DCT token type.
	/// Will return "Fungible" for MOA.
	fn dct_token_type(&self) -> DctTokenType;

	/// Will return the MOA call value,
	/// but also fail with an error if DCT is sent.
	/// Especially used in the auto-generated call value processing.
	fn require_moa(&self) -> BigUint {
		if !self.token().is_moa() {
			self.signal_error(err_msg::NON_PAYABLE_FUNC_DCT);
		}
		self.moa_value()
	}

	/// Will return the DCT call value,
	/// but also fail with an error if MOA or the wrong DCT token is sent.
	/// Especially used in the auto-generated call value processing.
	fn require_dct(&self, token: &[u8]) -> BigUint {
		if self.token() != token {
			self.signal_error(err_msg::BAD_TOKEN_PROVIDED);
		}
		self.dct_value()
	}

	/// Returns both the call value (either MOA or DCT) and the token identifier.
	/// Especially used in the `#[payable("*")] auto-generated snippets.
	/// The method might seem redundant, but there is such a hook in Arwen
	/// that might be used in this scenario in the future.
	fn payment_token_pair(&self) -> (BigUint, TokenIdentifier) {
		let token = self.token();
		if token.is_moa() {
			(self.moa_value(), token)
		} else {
			(self.dct_value(), token)
		}
	}
}
