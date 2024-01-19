extern crate moa_dct_swap;
use moa_dct_swap::*;
use dharitri_wasm::*;
use dharitri_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/moa-dct-swap.wasm",
		Box::new(|context| Box::new(MoaDctSwapImpl::new(context))),
	);
	contract_map
}

#[test]
fn wrap_moa_test() {
	parse_execute_denali("denali/wrap_moa.scen.json", &contract_map());
}

#[test]
fn wrap_then_unwrap_moa_test() {
	parse_execute_denali("denali/unwrap_moa.scen.json", &contract_map());
}
