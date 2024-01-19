extern crate payable_features;
use dharitri_wasm::*;
use dharitri_wasm_debug::*;
use payable_features::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/payable-features.wasm",
		Box::new(|context| Box::new(PayableFeaturesImpl::new(context))),
	);
	contract_map
}

#[test]
fn payable_any_1() {
	parse_execute_denali("denali/payable_any_1.scen.json", &contract_map());
}

#[test]
fn payable_any_2() {
	parse_execute_denali("denali/payable_any_2.scen.json", &contract_map());
}

#[test]
fn payable_any_3() {
	parse_execute_denali("denali/payable_any_3.scen.json", &contract_map());
}

#[test]
fn payable_any_4() {
	parse_execute_denali("denali/payable_any_4.scen.json", &contract_map());
}

#[test]
fn payable_moa_0() {
	parse_execute_denali("denali/payable_moa_0.scen.json", &contract_map());
}

#[test]
fn payable_moa_1() {
	parse_execute_denali("denali/payable_moa_1.scen.json", &contract_map());
}

#[test]
fn payable_moa_2() {
	parse_execute_denali("denali/payable_moa_2.scen.json", &contract_map());
}

#[test]
fn payable_moa_3() {
	parse_execute_denali("denali/payable_moa_3.scen.json", &contract_map());
}

#[test]
fn payable_moa_4() {
	parse_execute_denali("denali/payable_moa_4.scen.json", &contract_map());
}

#[test]
fn payable_token_1() {
	parse_execute_denali("denali/payable_token_1.scen.json", &contract_map());
}

#[test]
fn payable_token_2() {
	parse_execute_denali("denali/payable_token_2.scen.json", &contract_map());
}

#[test]
fn payable_token_3() {
	parse_execute_denali("denali/payable_token_3.scen.json", &contract_map());
}

#[test]
fn payable_token_4() {
	parse_execute_denali("denali/payable_token_4.scen.json", &contract_map());
}
