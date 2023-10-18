#![allow(unused_attributes)]
#![feature(no_coverage)]
#![feature(impl_trait_in_assoc_type)]

extern crate serde_json;
use std::fmt::Debug;
use move_binary_format::file_format::{
    CompiledModule, module_to_json, json_to_module, mutate_module};
use move_binary_format::builder;
use move_binary_format::fuzzer::Fuzzer;
use move_bytecode_verifier::{self, VerifierConfig};

// fn _verify_module_with_config() {
//     println!("empty");
// }

fn fuzz_target1(cm: &CompiledModule) {
// fn fuzz_target(cm: &CompiledModule) -> bool {
//     println!("in fuzz target");
    let verifier_config : VerifierConfig= VerifierConfig::default();
    let _tmp_yj = move_bytecode_verifier::verify_module_with_config(&verifier_config, cm);
    // return true;
}

fn main() {
    println!("yj Run bv. entry point");
    // let m = SampleStruct2::<u8, u8>::default_mutator();
    // let result = fuzzcheck::fuzz_test(fuzz_target)



    // let initial_inputs = fuzzer.read_input_corpus();
    // println!("[{:?}] initial seeds = {:?}",
    //          initial_inputs.as_ref().expect("length").len(), initial_inputs.as_ref());

    /// read single seed input file and transform to CM
    let mut original_m = json_to_module();
    // println!("before mutate {:?}", original_m);

    /// mutate
    let cm= mutate_module(&mut original_m);
    // println!("after mutate {:?}", cm);

    /// execute PUT with mutated input
    // fuzz_target(&cm);
    let res = builder::fuzz_test(fuzz_target1).launch();
    // let verifier_config : VerifierConfig= VerifierConfig::default();
    // let res = builder::fuzz_test(move_bytecode_verifier::verify_module_with_config).launch();
    // fuzzer1.set_fuzz_test(fuzz_target);
    println!("run test last part builder={:?}", res);

}
