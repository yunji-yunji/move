#![allow(unused_attributes)]
#![feature(no_coverage)]
#![feature(impl_trait_in_assoc_type)]

extern crate serde_json;
use std::fmt::Debug;
use move_binary_format::file_format::{CompiledModule, json_to_module, module_to_json, mutate_module};

fn fuzz_target(cm: CompiledModule) {
    println!("in fuzz target {:?}", cm);
    let verifier_config : VerifierConfig= VerifierConfig::default();
    move_bytecode_verifier::verify_module_with_config(&verifier_config, &cm);

}

use move_bytecode_verifier::{self, VerifierConfig, cyclic_dependencies, dependencies};

fn main() {
    println!("yj Run bv. entry point");
    /// read single seed input file and transform to CM
    let mut original_m = json_to_module();
    println!("before mutate {:?}", original_m);

    /// mutate
    let cm= mutate_module(&mut original_m);
    println!("after mutate {:?}", cm);

    /// execute PUT with mutated input
    fuzz_target(cm.clone());

    /// Codes using Fuzzcheck-rs
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
