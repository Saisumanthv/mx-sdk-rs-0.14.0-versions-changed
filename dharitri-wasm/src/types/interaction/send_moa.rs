use crate::abi::{OutputAbi, TypeAbi, TypeDescriptionContainer};
use crate::api::{BigUintApi, EndpointFinishApi, ErrorApi, SendApi};
use crate::io::EndpointResult;
use crate::types::{Address, BoxedBytes};
use alloc::string::String;
use alloc::vec::Vec;

pub struct SendMoa<BigUint: BigUintApi> {
	pub to: Address,
	pub amount: BigUint,
	pub data: BoxedBytes,
}

impl<FA, BigUint> EndpointResult<FA> for SendMoa<BigUint>
where
	BigUint: BigUintApi + 'static,
	FA: EndpointFinishApi + SendApi<BigUint> + ErrorApi + Clone + 'static,
{
	#[inline]
	fn finish(&self, api: FA) {
		api.direct_moa(&self.to, &self.amount, self.data.as_slice());
	}
}

impl<BigUint: BigUintApi> TypeAbi for SendMoa<BigUint> {
	fn type_name() -> String {
		"SendMoa".into()
	}

	/// No ABI output.
	fn output_abis(_: &[&'static str]) -> Vec<OutputAbi> {
		Vec::new()
	}

	fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}
