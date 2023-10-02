#![allow(unused_attributes)]
#![feature(no_coverage)]

#![feature(impl_trait_in_assoc_type)]
use std::fmt::Debug;

use serde::{Deserialize, Serialize};
// use fuzzcheck::{DefaultMutator, Mutator};
// use fuzzcheck::mutators::tuples::TupleStructure;
//

use move_binary_format::file_format::{
    empty_module, AbilitySet, Bytecode, CodeUnit, Constant, FieldDefinition, FunctionDefinition,
    FunctionHandle, FunctionHandleIndex, IdentifierIndex, ModuleHandleIndex, Signature,
    SignatureIndex, SignatureToken,
    SignatureToken::{Address, Bool},
    StructDefinition, StructFieldInformation, StructHandle, StructHandleIndex, TypeSignature,
    Visibility,
};
use move_core_types::{account_address::AccountAddress, identifier::Identifier};
use std::str::FromStr;
use move_binary_format::file_format::CompiledModule;
// use fuzzcheck::mutators::testing_utilities::test_mutator;

use crate::stack_usage_verifiers::StackUsageVerifier;

// #[derive(Serialize, Deserialize)]
// #[derive(Clone, Debug, PartialEq, Eq, Hash, DefaultMutator)]

fn tmp_func(module: CompiledModule) {
    let _ = move_bytecode_verifier::verify_module_unmetered(&module);
}


fn main() {
    println!("yj Run bv. entry point");
    // declare
    let cm = CompiledModule::<u8, u8>::default_mutator();
    // let suv: StackUsageVerifier;

    // mutate
    tmp_func(cm);
    // launch

    // let m = CompiledModule::<u8, u8>::default_mutator();
    // test_mutator(m, 1000., 1000., false, true, 50, 50);
    //
    // let result = fuzzcheck::fuzz_test(fuzz_target2)
    //     .default_mutator()
    //     .serde_serializer()
    //     .default_sensor_and_pool()
    //     .arguments_from_cargo_fuzzcheck()
    //     .stop_after_first_test_failure(true)
    //     .launch();
    // println!("after result");

}
