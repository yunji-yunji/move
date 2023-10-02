// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0
#![allow(unused_attributes)]
#![feature(no_coverage)]
#![feature(impl_trait_in_assoc_type)]
// use serde::{Deserialize, Serialize};
use fuzzcheck::DefaultMutator;
use fuzzcheck::mutators::testing_utilities::test_mutator;
use fuzzcheck::mutators::tuples::TupleStructure;
// use move_vm_integration_tests::tests::loader_tests::Adapter;

use move_core_types::account_address::AccountAddress;
// use move_core_types::account_address::AccountAddress;

use hex::FromHex;
use rand::{rngs::OsRng, Rng};
use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use std::{convert::TryFrom, fmt, str::FromStr,
          collections::{HashMap, VecDeque},};
use move_analyzer::{
    compiler::{as_module, as_script, compile_units},
};
// use move_vm_integration_tests::compilers::*;

use move_bytecode_verifier::VerifierConfig;
use move_vm_runtime::{config::VMConfig, move_vm::MoveVM};
use move_vm_test_utils::InMemoryStorage;
use move_vm_types::gas::UnmeteredGasMeter;

use tracing::warn;
use sha3::{Digest, Sha3_256};

// pub struct AccountAddress2([u8; AccountAddress2::LENGTH]);
const TEST_ADDR: AccountAddress = AccountAddress::new([42; AccountAddress::LENGTH]);

// #[test]
// test_publish_module_with_nested_loops
// fn fuzz_target(test_addr: &AccountAddress) {
    fn fuzz_target(test_addr: AccountAddress) {
    // fn test_publish_module_with_nested_loops(test_addr: &AccountAddress) {
    // Compile the modules and scripts.
    // TODO: find a better way to include the Signer module.
    let code = r#"
        module {{ADDR}}::M {
            fun foo() {
                let i = 0;
                while (i < 10) {
                    let j = 0;
                    while (j < 10) {
                        j = j + 1;
                    };
                    i = i + 1;
                };
            }
        }
    "#;
    // let code = code.replace("{{ADDR}}", &format!("0x{:?}", *test_addr));
    let code = code.replace("{{ADDR}}", &format!("0x{:?}", test_addr));
    println!("print in fuzztarget 2!!!!");
    // let code = code.replace("{{ADDR}}", &format!("0x{}", TEST_ADDR));
    let mut units = compile_units(&code).unwrap();
    println!("print in fuzztarget 2.1");

    let m = as_module(units.pop().unwrap());
    println!("print in fuzztarget 2.2");
    let mut m_blob = vec![];
    println!("print in fuzztarget 2.3");
    m.serialize(&mut m_blob).unwrap();
    println!("print in fuzztarget 2.4");

    // Should succeed with max_loop_depth = 2
    {
        let storage = InMemoryStorage::new();
        let vm = MoveVM::new_with_config(
            move_stdlib::natives::all_natives(
                // all_natives(
                AccountAddress::from_hex_literal("0x1").unwrap(),
                move_stdlib::natives::GasParameters::zeros(),
            ),
            VMConfig {
                verifier: VerifierConfig {
                    max_loop_depth: Some(2),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
            .unwrap();
        println!("print in fuzztarget 4");

        let mut sess = vm.new_session(&storage);
        sess.publish_module(m_blob.clone(), TEST_ADDR, &mut UnmeteredGasMeter)
            .unwrap();
    }
    println!("print in fuzztarget 5");

    // Should fail with max_loop_depth = 1
    {
        let storage = InMemoryStorage::new();
        let vm = MoveVM::new_with_config(
            move_stdlib::natives::all_natives(
                // all_natives(
                AccountAddress::from_hex_literal("0x1").unwrap(),
                move_stdlib::natives::GasParameters::zeros(),
            ),
            VMConfig {
                verifier: VerifierConfig {
                    max_loop_depth: Some(1),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
            .unwrap();

        let mut sess = vm.new_session(&storage);
        sess.publish_module(m_blob, TEST_ADDR, &mut UnmeteredGasMeter)
            .unwrap_err();
    }
    println!("End of fuzz target ");

}

#[test]
fn test_run_script_with_nested_loops() {
    // Compile the modules and scripts.
    // TODO: find a better way to include the Signer module.
    let code = r#"
        script {
            fun main() {
                let i = 0;
                while (i < 10) {
                    let j = 0;
                    while (j < 10) {
                        j = j + 1;
                    };
                    i = i + 1;
                };
            }
        }
    "#;
    let code = code.replace("{{ADDR}}", &format!("0x{}", TEST_ADDR));
    let mut units = compile_units(&code).unwrap();

    let s = as_script(units.pop().unwrap());
    let mut s_blob: Vec<u8> = vec![];
    s.serialize(&mut s_blob).unwrap();

    // Should succeed with max_loop_depth = 2
    {
        let storage = InMemoryStorage::new();
        let vm = MoveVM::new_with_config(
            move_stdlib::natives::all_natives(
                // all_natives(
                AccountAddress::from_hex_literal("0x1").unwrap(),
                move_stdlib::natives::GasParameters::zeros(),
            ),
            VMConfig {
                verifier: VerifierConfig {
                    max_loop_depth: Some(2),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
            .unwrap();

        let mut sess = vm.new_session(&storage);
        let args: Vec<Vec<u8>> = vec![];
        sess.execute_script(s_blob.clone(), vec![], args, &mut UnmeteredGasMeter)
            .unwrap();
    }

    // Should fail with max_loop_depth = 1
    {
        let storage = InMemoryStorage::new();
        let vm = MoveVM::new_with_config(
            move_stdlib::natives::all_natives(
            // all_natives(
                AccountAddress::from_hex_literal("0x1").unwrap(),
                move_stdlib::natives::GasParameters::zeros(),
            ),
            VMConfig {
                verifier: VerifierConfig {
                    max_loop_depth: Some(1),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
            .unwrap();

        let mut sess = vm.new_session(&storage);
        let args: Vec<Vec<u8>> = vec![];
        sess.execute_script(s_blob, vec![], args, &mut UnmeteredGasMeter)
            .unwrap_err();
    }
}

fn main() {
    println!("Main function in bad_entry_point file.");
    // let aa = AccountAddress2::new([42; AccountAddress2::LENGTH]);
    // let m = AccountAddress::default_mutator();
    // // let m = SampleStruct2::<u8, u8>::default_mutator();
    // test_mutator(m, 1000., 1000., false, true, 50, 50);
    // let result = fuzzcheck::fuzz_test(fuzz_target)
    //     .default_mutator()
    //     .serde_serializer()
    //     .default_sensor_and_pool()
    //     .arguments_from_cargo_fuzzcheck()
    //     .stop_after_first_test_failure(true)
    //     .launch();
    fuzz_target(TEST_ADDR);
    println!("DONE execution yjyj");
}

