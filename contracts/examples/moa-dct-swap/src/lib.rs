#![no_std]

dharitri_wasm::imports!();
dharitri_wasm::derive_imports!();

const MOA_NUM_DECIMALS: usize = 18;

/// Converts between MOA and a wrapped MOA DCT token.
///	1 MOA = 1 wrapped MOA and is interchangeable at all times.
/// Also manages the supply of wrapped MOA tokens.
#[dharitri_wasm_derive::contract(MoaDctSwapImpl)]
pub trait MoaDctSwap {
	#[init]
	fn init(&self) {}

	// endpoints - owner-only

	#[payable("MOA")]
	#[endpoint(issueWrappedMoa)]
	fn issue_wrapped_moa(
		&self,
		token_display_name: BoxedBytes,
		token_ticker: BoxedBytes,
		initial_supply: BigUint,
		#[payment] issue_cost: BigUint,
	) -> SCResult<AsyncCall<BigUint>> {
		only_owner!(self, "only owner may call this function");

		require!(
			self.wrapped_moa_token_id().is_empty(),
			"wrapped moa was already issued"
		);

		let caller = self.get_caller();

		self.issue_started_event(&caller, token_ticker.as_slice(), &initial_supply);

		Ok(DCTSystemSmartContractProxy::new()
			.issue_fungible(
				issue_cost,
				&token_display_name,
				&token_ticker,
				&initial_supply,
				FungibleTokenProperties {
					num_decimals: MOA_NUM_DECIMALS,
					can_freeze: false,
					can_wipe: false,
					can_pause: false,
					can_mint: true,
					can_burn: false,
					can_change_owner: true,
					can_upgrade: true,
					can_add_special_roles: false,
				},
			)
			.async_call()
			.with_callback(self.callbacks().dct_issue_callback(&caller)))
	}

	#[callback]
	fn dct_issue_callback(
		&self,
		caller: &Address,
		#[payment_token] token_identifier: TokenIdentifier,
		#[payment] returned_tokens: BigUint,
		#[call_result] result: AsyncCallResult<()>,
	) {
		// callback is called with DCTTransfer of the newly issued token, with the amount requested,
		// so we can get the token identifier and amount from the call data
		match result {
			AsyncCallResult::Ok(()) => {
				self.issue_success_event(caller, &token_identifier, &returned_tokens);
				self.unused_wrapped_moa().set(&returned_tokens);
				self.wrapped_moa_token_id().set(&token_identifier);
			},
			AsyncCallResult::Err(message) => {
				self.issue_failure_event(caller, message.err_msg.as_slice());

				// return issue cost to the owner
				// TODO: test that it works
				if token_identifier.is_moa() && returned_tokens > 0 {
					self.send().direct_moa(caller, &returned_tokens, &[]);
				}
			},
		}
	}

	#[endpoint(mintWrappedMoa)]
	fn mint_wrapped_moa(&self, amount: BigUint) -> SCResult<AsyncCall<BigUint>> {
		only_owner!(self, "only owner may call this function");

		require!(
			!self.wrapped_moa_token_id().is_empty(),
			"Wrapped MOA was not issued yet"
		);

		let wrapped_moa_token_id = self.wrapped_moa_token_id().get();
		let dct_token_id = wrapped_moa_token_id.as_dct_identifier();
		let caller = self.get_caller();
		self.mint_started_event(&caller, &amount);

		Ok(DCTSystemSmartContractProxy::new()
			.mint(dct_token_id, &amount)
			.async_call()
			.with_callback(self.callbacks().dct_mint_callback(&caller, &amount)))
	}

	#[callback]
	fn dct_mint_callback(
		&self,
		caller: &Address,
		amount: &BigUint,
		#[call_result] result: AsyncCallResult<()>,
	) {
		match result {
			AsyncCallResult::Ok(()) => {
				self.mint_success_event(caller);
				self.unused_wrapped_moa()
					.update(|unused_wrapped_moa| *unused_wrapped_moa += amount);
			},
			AsyncCallResult::Err(message) => {
				self.mint_failure_event(caller, message.err_msg.as_slice());
			},
		}
	}

	// endpoints

	#[payable("MOA")]
	#[endpoint(wrapMoa)]
	fn wrap_moa(&self, #[payment] payment: BigUint) -> SCResult<()> {
		require!(payment > 0, "Payment must be more than 0");
		require!(
			!self.wrapped_moa_token_id().is_empty(),
			"Wrapped MOA was not issued yet"
		);

		let mut unused_wrapped_moa = self.unused_wrapped_moa().get();
		require!(
			unused_wrapped_moa > payment,
			"Contract does not have enough wrapped MOA. Please try again once more is minted."
		);
		unused_wrapped_moa -= &payment;
		self.unused_wrapped_moa().set(&unused_wrapped_moa);

		let caller = self.get_caller();
		self.send().direct_dct_via_transf_exec(
			&caller,
			self.wrapped_moa_token_id().get().as_dct_identifier(),
			&payment,
			b"wrapping",
		);

		self.wrap_moa_event(&caller, &payment);

		Ok(())
	}

	#[payable("*")]
	#[endpoint(unwrapMoa)]
	fn unwrap_moa(
		&self,
		#[payment] wrapped_moa_payment: BigUint,
		#[payment_token] token_identifier: TokenIdentifier,
	) -> SCResult<()> {
		require!(
			!self.wrapped_moa_token_id().is_empty(),
			"Wrapped MOA was not issued yet"
		);
		require!(token_identifier.is_dct(), "Only DCT tokens accepted");

		let wrapped_moa_token_identifier = self.wrapped_moa_token_id().get();

		require!(
			token_identifier == wrapped_moa_token_identifier,
			"Wrong dct token"
		);

		require!(wrapped_moa_payment > 0, "Must pay more than 0 tokens!");
		// this should never happen, but we'll check anyway
		require!(
			wrapped_moa_payment <= self.get_sc_balance(),
			"Contract does not have enough funds"
		);

		self.unused_wrapped_moa()
			.update(|unused_wrapped_moa| *unused_wrapped_moa += &wrapped_moa_payment);

		// 1 wrapped MOA = 1 MOA, so we pay back the same amount
		let caller = self.get_caller();
		self.send()
			.direct_moa(&caller, &wrapped_moa_payment, b"unwrapping");

		self.unwrap_moa_event(&caller, &wrapped_moa_payment);

		Ok(())
	}

	#[view(getLockedMoaBalance)]
	fn get_locked_moa_balance(&self) -> BigUint {
		self.get_sc_balance()
	}

	// storage

	#[view(getWrappedMoaTokenIdentifier)]
	#[storage_mapper("wrapped_moa_token_id")]
	fn wrapped_moa_token_id(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;

	#[view(getUnusedWrappedMoa)]
	#[storage_mapper("unused_wrapped_moa")]
	fn unused_wrapped_moa(&self) -> SingleValueMapper<Self::Storage, BigUint>;

	// events

	#[event("issue-started")]
	fn issue_started_event(
		&self,
		#[indexed] caller: &Address,
		#[indexed] token_ticker: &[u8],
		initial_supply: &BigUint,
	);

	#[event("issue-success")]
	fn issue_success_event(
		&self,
		#[indexed] caller: &Address,
		#[indexed] token_identifier: &TokenIdentifier,
		initial_supply: &BigUint,
	);

	#[event("issue-failure")]
	fn issue_failure_event(&self, #[indexed] caller: &Address, message: &[u8]);

	#[event("mint-started")]
	fn mint_started_event(&self, #[indexed] caller: &Address, amount: &BigUint);

	#[event("mint-success")]
	fn mint_success_event(&self, #[indexed] caller: &Address);

	#[event("mint-failure")]
	fn mint_failure_event(&self, #[indexed] caller: &Address, message: &[u8]);

	#[event("wrap-moa")]
	fn wrap_moa_event(&self, #[indexed] user: &Address, amount: &BigUint);

	#[event("unwrap-moa")]
	fn unwrap_moa_event(&self, #[indexed] user: &Address, amount: &BigUint);
}
