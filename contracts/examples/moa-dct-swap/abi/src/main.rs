use moa_dct_swap::*;
use dharitri_wasm_debug::*;

fn main() {
	let contract = MoaDctSwapImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
