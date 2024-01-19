#![no_std]
#![allow(clippy::string_lit_as_bytes)]
#![allow(clippy::redundant_clone)]

dharitri_wasm::imports!();

/// Contract that only tests the call value features,
/// i.e. the framework/Arwen functionality for accepting MOA and DCT payments.
#[dharitri_wasm_derive::contract(PayableFeaturesImpl)]
pub trait PayableFeatures {
	#[view]
	#[payable("*")]
	fn check_call_value(
		&self,
	) -> MultiResult5<BigUint, BigUint, TokenIdentifier, BigUint, TokenIdentifier> {
		let (pair_call_value, pair_token_name) = self.call_value().payment_token_pair();
		(
			self.call_value().moa_value(),
			self.call_value().dct_value(),
			self.call_value().token(),
			pair_call_value,
			pair_token_name,
		)
			.into()
	}

	#[endpoint]
	#[payable("*")]
	fn payable_any_1(
		&self,
		#[payment] payment: BigUint,
		#[payment_token] token: TokenIdentifier,
	) -> MultiResult2<BigUint, TokenIdentifier> {
		(payment, token).into()
	}

	#[endpoint]
	#[payable("*")]
	fn payable_any_2(&self, #[payment] payment: BigUint) -> MultiResult2<BigUint, TokenIdentifier> {
		let token = self.call_value().token();
		(payment, token).into()
	}

	#[endpoint]
	#[payable("*")]
	fn payable_any_3(
		&self,
		#[payment_token] token: TokenIdentifier,
	) -> MultiResult2<BigUint, TokenIdentifier> {
		let (payment, _) = self.call_value().payment_token_pair();
		(payment, token).into()
	}

	#[endpoint]
	#[payable("*")]
	fn payable_any_4(&self) -> MultiResult2<BigUint, TokenIdentifier> {
		self.call_value().payment_token_pair().into()
	}

	/// Will issue a warning, but this is ok, this is the test.
	#[endpoint]
	#[payable]
	fn payable_moa_0(
		&self,
		#[payment] payment: BigUint,
		#[payment_token] token: TokenIdentifier,
	) -> MultiResult2<BigUint, TokenIdentifier> {
		(payment, token).into()
	}

	#[endpoint]
	#[payable("MOA")]
	fn payable_moa_1(
		&self,
		#[payment_token] token: TokenIdentifier,
	) -> MultiResult2<BigUint, TokenIdentifier> {
		let payment = self.call_value().moa_value();
		(payment, token).into()
	}

	#[endpoint]
	#[payable("MOA")]
	fn payable_moa_2(
		&self,
		#[payment] payment: BigUint,
	) -> MultiResult2<BigUint, TokenIdentifier> {
		let token = self.call_value().token();
		(payment, token).into()
	}

	#[endpoint]
	#[payable("MOA")]
	fn payable_moa_3(
		&self,
		#[payment_token] token: TokenIdentifier,
	) -> MultiResult2<BigUint, TokenIdentifier> {
		let payment = self.call_value().moa_value();
		(payment, token).into()
	}

	#[endpoint]
	#[payable("MOA")]
	fn payable_moa_4(&self) -> MultiResult2<BigUint, TokenIdentifier> {
		let payment = self.call_value().moa_value();
		let token = self.call_value().token();
		(payment, token).into()
	}

	#[endpoint]
	#[payable("PAYABLE-FEATURES-TOKEN")]
	fn payable_token_1(
		&self,
		#[payment] payment: BigUint,
		#[payment_token] token: TokenIdentifier,
	) -> MultiResult2<BigUint, TokenIdentifier> {
		(payment, token).into()
	}

	#[endpoint]
	#[payable("PAYABLE-FEATURES-TOKEN")]
	fn payable_token_2(
		&self,
		#[payment] payment: BigUint,
	) -> MultiResult2<BigUint, TokenIdentifier> {
		let token = self.call_value().token();
		(payment, token).into()
	}

	#[endpoint]
	#[payable("PAYABLE-FEATURES-TOKEN")]
	fn payable_token_3(
		&self,
		#[payment_token] token: TokenIdentifier,
	) -> MultiResult2<BigUint, TokenIdentifier> {
		let payment = self.call_value().dct_value();
		(payment, token).into()
	}

	#[endpoint]
	#[payable("PAYABLE-FEATURES-TOKEN")]
	fn payable_token_4(&self) -> MultiResult2<BigUint, TokenIdentifier> {
		let payment = self.call_value().dct_value();
		let token = self.call_value().token();
		(payment, token).into()
	}
}
