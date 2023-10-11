#![allow(unused_attributes)]
#![feature(no_coverage)]

#![feature(impl_trait_in_assoc_type)]
use std::fmt::Debug;

extern crate serde_json;
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
use move_binary_format::file_format::{CompiledModule, json_to_module, module_to_json, mutate_module};
// use rand::{rngs::StdRng, Rng};
// use move_binary_format::BinaryData;
use move_binary_format::file_format_common::BinaryData;
use move_binary_format::serializer::*;
// use fuzzcheck::mutators::testing_utilities::test_mutator;

// use crate::stack_usage_verifiers::StackUsageVerifier;

// #[derive(Serialize, Deserialize)]
// #[derive(Clone, Debug, PartialEq, Eq, Hash, DefaultMutator)]

fn fuzz_target(cm: CompiledModule) {
    println!("in fuzz target {:?}", cm);
    // let vm_config : VMConfig = Default::default();
    let verifier_config : VerifierConfig= VerifierConfig::default();
    // let _ = move_bytecode_verifier::verify_module_unmetered(&module);
    // let _ = move_bytecode_verifier::verify_module(&module);
    move_bytecode_verifier::verify_module_with_config(&verifier_config, &cm);

}

use move_bytecode_verifier::{self, VerifierConfig, cyclic_dependencies, dependencies};

fn main() {
    println!("yj Run bv. entry point");
    /// declare
    // let cm = CompiledModule::<u8, u8>::default_mutator();
    // let cm: CompiledModule::<u8, u8>;
    // let cm: CompiledModule = Default::default();
    let mut original_m = json_to_module();
    println!("before mutate {:?}", original_m);

    let cm= mutate_module(&mut original_m);
    // .map_err(expect_no_verification_errors)?;
    println!("after mutate {:?}", cm);

    /// flatten.. == serialize?
    /// serialize
    ///
    // let mut bins : Vec<BinaryData> = vec![];
    // let idx = SignatureIndex(3);
    // let tmp = CompiledModule::serialize_address_identifier_index(&mut bins, &idx);
    // println!("result in bv {:?}", tmp);
    /// deserialize
    // let module_id = CompiledModule::deserialize(&fs::read()?);
    // let serialized_cm = serde_json::to_string(&cm).unwrap();
    // println!("Serialized: {}", serialized_cm);
    // let suv: StackUsageVerifier;

    // mutate
    fuzz_target(cm.clone());
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
