// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0
#![allow(unused_attributes)]
// #![feature(no_coverage)]
#![feature(impl_trait_in_assoc_type)]
use serde::{Deserialize, Serialize};
// use fuzzcheck::DefaultMutator;
// use fuzzcheck::mutators::testing_utilities::test_mutator;

use crate::compiler::{as_module, compile_units};
use move_core_types::{
    account_address::AccountAddress,
    identifier::Identifier,
    language_storage::ModuleId,
    value::{serialize_values, MoveValue},
    vm_status::StatusType,
};
use move_vm_runtime::move_vm::MoveVM;
use move_vm_test_utils::{BlankStorage, InMemoryStorage};
use move_vm_types::gas::UnmeteredGasMeter;

const TEST_ADDR: AccountAddress = AccountAddress::new([42; AccountAddress::LENGTH]);

// yunji code =============================================
// #[derive(Serialize, Deserialize)]
// #[derive(Clone, Debug, PartialEq, Eq, Hash, DefaultMutator)]
// struct SampleStruct2<T, U> {
//     x: T,
//     y: U,
//     n: U,
// }
// fn fuzz_target(s: &SampleStruct2<u16, u16>) {
//     println!("fuzz_target called in main");
// }
//
// fn main() {
//     println!("Main function in bad_entry_point file.");
//
//     let m = SampleStruct2::<u8, u8>::default_mutator();
//     test_mutator(m, 1000., 1000., false, true, 50, 50);
//     let result = fuzzcheck::fuzz_test(fuzz_target)
//         .default_mutator()
//         .serde_serializer()
//         .default_sensor_and_pool()
//         .arguments_from_cargo_fuzzcheck()
//         .stop_after_first_test_failure(true)
//         .launch();
//     println!("DONE execution yjyj");
// }
// yunji code =============================================



#[test]
fn call_non_existent_module() {
    let vm = MoveVM::new(vec![]).unwrap();
    let storage = BlankStorage;

    let mut sess = vm.new_session(&storage);
    let module_id = ModuleId::new(TEST_ADDR, Identifier::new("M").unwrap());
    let fun_name = Identifier::new("foo").unwrap();

    let err = sess
        .execute_function_bypass_visibility(
            &module_id,
            &fun_name,
            vec![],
            serialize_values(&vec![MoveValue::Signer(TEST_ADDR)]),
            &mut UnmeteredGasMeter,
        )
        .unwrap_err();

    assert_eq!(err.status_type(), StatusType::Verification);
}

#[test]
fn call_non_existent_function() {
    let code = r#"
        module {{ADDR}}::M {}
    "#;
    let code = code.replace("{{ADDR}}", &format!("0x{}", TEST_ADDR));

    let mut units = compile_units(&code).unwrap();
    let m = as_module(units.pop().unwrap());
    let mut blob = vec![];
    m.serialize(&mut blob).unwrap();

    let mut storage = InMemoryStorage::new();
    let module_id = ModuleId::new(TEST_ADDR, Identifier::new("M").unwrap());
    storage.publish_or_overwrite_module(module_id.clone(), blob);

    let vm = MoveVM::new(vec![]).unwrap();
    let mut sess = vm.new_session(&storage);

    let fun_name = Identifier::new("foo").unwrap();

    let err = sess
        .execute_function_bypass_visibility(
            &module_id,
            &fun_name,
            vec![],
            serialize_values(&vec![MoveValue::Signer(TEST_ADDR)]),
            &mut UnmeteredGasMeter,
        )
        .unwrap_err();

    assert_eq!(err.status_type(), StatusType::Verification);
}
