// #![allow(unused_attributes)]
// #![feature(no_coverage)]
// #![feature(impl_trait_in_assoc_type)]
// use fuzzcheck::DefaultMutator;
// use fuzzcheck::mutators::testing_utilities::test_mutator;
// use fuzzcheck::mutators::tuples::TupleStructure;
//
// use hex::FromHex;
// use rand::{rngs::OsRng, Rng};
// use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
// use std::{convert::TryFrom, fmt, str::FromStr};
//
//
// use crate::compiler::{as_module, as_script, compile_units};
//
// use move_bytecode_verifier::VerifierConfig;
// // use move_core_types::account_address::AccountAddress3;
// use move_vm_runtime::{config::VMConfig, move_vm::MoveVM};
// use move_vm_test_utils::InMemoryStorage;
// use move_vm_types::gas::UnmeteredGasMeter;
//
// use move_vm_runtime::native_functions::{make_table_from_iter, NativeFunctionTable};
//
//
//
//
//
//
// // ============================
// // Debug,
// #[derive(Clone, PartialEq, Eq, Hash, DefaultMutator)]
// pub struct AccountAddress3([u8; AccountAddress3::LENGTH]);
//
// impl AccountAddress3 {
//     pub const fn new(address: [u8; Self::LENGTH]) -> Self {
//         Self(address)
//     }
//
//     /// The number of bytes in an address.
//     /// Default to 16 bytes, can be set to 20 bytes with address20 feature.
//     pub const LENGTH: usize = if cfg!(feature = "address20") {
//         20
//     } else if cfg!(feature = "address32") {
//         32
//     } else {
//         16
//     };
//
//     /// Hex address: 0x0
//     pub const ZERO: Self = Self([0u8; Self::LENGTH]);
//
//     /// Hex address: 0x1
//     pub const ONE: Self = Self::get_hex_address_one();
//
//     /// Hex address: 0x2
//     pub const TWO: Self = Self::get_hex_address_two();
//
//     const fn get_hex_address_one() -> Self {
//         let mut addr = [0u8; AccountAddress3::LENGTH];
//         addr[AccountAddress3::LENGTH - 1] = 1u8;
//         Self(addr)
//     }
//
//     const fn get_hex_address_two() -> Self {
//         let mut addr = [0u8; AccountAddress3::LENGTH];
//         addr[AccountAddress3::LENGTH - 1] = 2u8;
//         Self(addr)
//     }
//
//     pub fn random() -> Self {
//         let mut rng = OsRng;
//         let buf: [u8; Self::LENGTH] = rng.gen();
//         Self(buf)
//     }
//
//     /// Return a canonical string representation of the address
//     /// Addresses are hex-encoded lowercase values of length ADDRESS_LENGTH (16, 20, or 32 depending on the Move platform)
//     /// e.g., 0000000000000000000000000000000a, *not* 0x0000000000000000000000000000000a, 0xa, or 0xA
//     /// Note: this function is guaranteed to be stable, and this is suitable for use inside
//     /// Move native functions or the VM.
//     pub fn to_canonical_string(&self) -> String {
//         hex::encode(self.0)
//     }
//
//     pub fn short_str_lossless(&self) -> String {
//         let hex_str = hex::encode(self.0).trim_start_matches('0').to_string();
//         if hex_str.is_empty() {
//             "0".to_string()
//         } else {
//             hex_str
//         }
//     }
//
//     pub fn to_vec(&self) -> Vec<u8> {
//         self.0.to_vec()
//     }
//
//     pub fn into_bytes(self) -> [u8; Self::LENGTH] {
//         self.0
//     }
//
//     pub fn from_hex_literal(literal: &str) -> Result<Self, AccountAddress3ParseError> {
//         if !literal.starts_with("0x") {
//             return Err(AccountAddress3ParseError);
//         }
//
//         let hex_len = literal.len() - 2;
//
//         // If the string is too short, pad it
//         if hex_len < Self::LENGTH * 2 {
//             let mut hex_str = String::with_capacity(Self::LENGTH * 2);
//             for _ in 0..Self::LENGTH * 2 - hex_len {
//                 hex_str.push('0');
//             }
//             hex_str.push_str(&literal[2..]);
//             AccountAddress3::from_hex(hex_str)
//         } else {
//             AccountAddress3::from_hex(&literal[2..])
//         }
//     }
//
//     pub fn to_hex_literal(&self) -> String {
//         format!("0x{}", self.short_str_lossless())
//     }
//
//     pub fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, AccountAddress3ParseError> {
//         <[u8; Self::LENGTH]>::from_hex(hex)
//             .map_err(|_| AccountAddress3ParseError)
//             .map(Self)
//     }
//
//     pub fn to_hex(&self) -> String {
//         format!("{:x}", self)
//     }
//
//     pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, AccountAddress3ParseError> {
//         <[u8; Self::LENGTH]>::try_from(bytes.as_ref())
//             .map_err(|_| AccountAddress3ParseError)
//             .map(Self)
//     }
// }
//
// impl AsRef<[u8]> for AccountAddress3 {
//     fn as_ref(&self) -> &[u8] {
//         &self.0
//     }
// }
//
// impl std::ops::Deref for AccountAddress3 {
//     type Target = [u8; Self::LENGTH];
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
//
// impl fmt::Display for AccountAddress3 {
//     fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{:x}", self)
//     }
// }
//
// impl fmt::Debug for AccountAddress3 {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:x}", self)
//     }
// }
//
// impl fmt::LowerHex for AccountAddress3 {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         if f.alternate() {
//             write!(f, "0x")?;
//         }
//
//         for byte in &self.0 {
//             write!(f, "{:02x}", byte)?;
//         }
//
//         Ok(())
//     }
// }
//
// impl fmt::UpperHex for AccountAddress3 {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         if f.alternate() {
//             write!(f, "0x")?;
//         }
//
//         for byte in &self.0 {
//             write!(f, "{:02X}", byte)?;
//         }
//
//         Ok(())
//     }
// }
//
// impl From<[u8; AccountAddress3::LENGTH]> for AccountAddress3 {
//     fn from(bytes: [u8; AccountAddress3::LENGTH]) -> Self {
//         Self::new(bytes)
//     }
// }
//
// impl TryFrom<&[u8]> for AccountAddress3 {
//     type Error = AccountAddress3ParseError;
//
//     /// Tries to convert the provided byte array into Address.
//     fn try_from(bytes: &[u8]) -> Result<AccountAddress3, AccountAddress3ParseError> {
//         Self::from_bytes(bytes)
//     }
// }
//
// impl TryFrom<Vec<u8>> for AccountAddress3 {
//     type Error = AccountAddress3ParseError;
//
//     /// Tries to convert the provided byte buffer into Address.
//     fn try_from(bytes: Vec<u8>) -> Result<AccountAddress3, AccountAddress3ParseError> {
//         Self::from_bytes(bytes)
//     }
// }
//
// impl From<AccountAddress3> for Vec<u8> {
//     fn from(addr: AccountAddress3) -> Vec<u8> {
//         addr.0.to_vec()
//     }
// }
//
// impl From<&AccountAddress3> for Vec<u8> {
//     fn from(addr: &AccountAddress3) -> Vec<u8> {
//         addr.0.to_vec()
//     }
// }
//
// impl From<AccountAddress3> for [u8; AccountAddress3::LENGTH] {
//     fn from(addr: AccountAddress3) -> Self {
//         addr.0
//     }
// }
//
// impl From<&AccountAddress3> for [u8; AccountAddress3::LENGTH] {
//     fn from(addr: &AccountAddress3) -> Self {
//         addr.0
//     }
// }
//
// impl From<&AccountAddress3> for String {
//     fn from(addr: &AccountAddress3) -> String {
//         ::hex::encode(addr.as_ref())
//     }
// }
//
// impl TryFrom<String> for AccountAddress3 {
//     type Error = AccountAddress3ParseError;
//
//     fn try_from(s: String) -> Result<AccountAddress3, AccountAddress3ParseError> {
//         Self::from_hex(s)
//     }
// }
//
// impl FromStr for AccountAddress3 {
//     type Err = AccountAddress3ParseError;
//
//     fn from_str(s: &str) -> Result<Self, AccountAddress3ParseError> {
//         // Accept 0xADDRESS or ADDRESS
//         if let Ok(address) = AccountAddress3::from_hex_literal(s) {
//             Ok(address)
//         } else {
//             Self::from_hex(s)
//         }
//     }
// }
//
// impl<'de> Deserialize<'de> for AccountAddress3 {
//     fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
//         where
//             D: Deserializer<'de>,
//     {
//         if deserializer.is_human_readable() {
//             let s = <String>::deserialize(deserializer)?;
//             AccountAddress3::from_str(&s).map_err(D::Error::custom)
//         } else {
//             // In order to preserve the Serde data model and help analysis tools,
//             // make sure to wrap our value in a container with the same name
//             // as the original type.
//             #[derive(::serde::Deserialize)]
//             #[serde(rename = "AccountAddress3")]
//             struct Value([u8; AccountAddress3::LENGTH]);
//
//             let value = Value::deserialize(deserializer)?;
//             Ok(AccountAddress3::new(value.0))
//         }
//     }
// }
//
// impl Serialize for AccountAddress3 {
//     fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
//         where
//             S: Serializer,
//     {
//         if serializer.is_human_readable() {
//             self.to_hex().serialize(serializer)
//         } else {
//             // See comment in deserialize.
//             serializer.serialize_newtype_struct("AccountAddress3", &self.0)
//         }
//     }
// }
//
// #[derive(Clone, Copy, Debug)]
// pub struct AccountAddress3ParseError;
//
// impl fmt::Display for AccountAddress3ParseError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
//         write!(
//             f,
//             "Unable to parse AccountAddress3 (must be hex string of length {})",
//             AccountAddress3::LENGTH
//         )
//     }
// }
//
// impl std::error::Error for AccountAddress3ParseError {}
//
//
// const TEST_ADDR: AccountAddress3 = AccountAddress3::new([42; AccountAddress3::LENGTH]);
//
//
// pub fn all_natives(
//     move_std_addr: AccountAddress3,
//     gas_params: GasParameters,
// ) -> NativeFunctionTable {
//     let mut natives = vec![];
//
//     macro_rules! add_natives {
//         ($module_name: expr, $natives: expr) => {
//             natives.extend(
//                 $natives.map(|(func_name, func)| ($module_name.to_string(), func_name, func)),
//             );
//         };
//     }
//
//     add_natives!("bcs", bcs::make_all(gas_params.bcs));
//     add_natives!("hash", hash::make_all(gas_params.hash));
//     add_natives!("signer", signer::make_all(gas_params.signer));
//     add_natives!("string", string::make_all(gas_params.string));
//     add_natives!("type_name", type_name::make_all(gas_params.type_name));
//     add_natives!("vector", vector::make_all(gas_params.vector));
//     #[cfg(feature = "testing")]
//     {
//         add_natives!("unit_test", unit_test::make_all(gas_params.unit_test));
//     }
//
//     make_table_from_iter(move_std_addr, natives)
// }
//
//
//
// fn test_publish_module_with_nested_loops(test_addr: &AccountAddress3) {
//     // Compile the modules and scripts.
//     // TODO: find a better way to include the Signer module.
//     let code = r#"
//         module {{ADDR}}::M {
//             fun foo() {
//                 let i = 0;
//                 while (i < 10) {
//                     let j = 0;
//                     while (j < 10) {
//                         j = j + 1;
//                     };
//                     i = i + 1;
//                 };
//             }
//         }
//     "#;
//     let code = code.replace("{{ADDR}}", &format!("0x{:?}", *test_addr));
//     // let code = code.replace("{{ADDR}}", &format!("0x{}", TEST_ADDR));
//     let mut units = compile_units(&code).unwrap();
//
//     let m = as_module(units.pop().unwrap());
//     let mut m_blob = vec![];
//     m.serialize(&mut m_blob).unwrap();
//
//     // Should succeed with max_loop_depth = 2
//     {
//         let storage = InMemoryStorage::new();
//         let vm = MoveVM::new_with_config(
//             // move_stdlib::natives::all_natives(
//                 all_natives(
//                 AccountAddress3::from_hex_literal("0x1").unwrap(),
//                 move_stdlib::natives::GasParameters::zeros(),
//             ),
//             VMConfig {
//                 verifier: VerifierConfig {
//                     max_loop_depth: Some(2),
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             },
//         )
//             .unwrap();
//
//         let mut sess = vm.new_session(&storage);
//         sess.publish_module(m_blob.clone(), TEST_ADDR, &mut UnmeteredGasMeter)
//             .unwrap();
//     }
//
//     // Should fail with max_loop_depth = 1
//     {
//         let storage = InMemoryStorage::new();
//         let vm = MoveVM::new_with_config(
//             // move_stdlib::natives::all_natives(
//                 all_natives(
//                 AccountAddress3::from_hex_literal("0x1").unwrap(),
//                 move_stdlib::natives::GasParameters::zeros(),
//             ),
//             VMConfig {
//                 verifier: VerifierConfig {
//                     max_loop_depth: Some(1),
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             },
//         )
//             .unwrap();
//
//         let mut sess = vm.new_session(&storage);
//         sess.publish_module(m_blob, TEST_ADDR, &mut UnmeteredGasMeter)
//             .unwrap_err();
//     }
// }
//
//
//
// #[test]
// fn test_run_script_with_nested_loops() {
//     // Compile the modules and scripts.
//     // TODO: find a better way to include the Signer module.
//     let code = r#"
//         script {
//             fun main() {
//                 let i = 0;
//                 while (i < 10) {
//                     let j = 0;
//                     while (j < 10) {
//                         j = j + 1;
//                     };
//                     i = i + 1;
//                 };
//             }
//         }
//     "#;
//     let code = code.replace("{{ADDR}}", &format!("0x{}", TEST_ADDR));
//     let mut units = compile_units(&code).unwrap();
//
//     let s = as_script(units.pop().unwrap());
//     let mut s_blob: Vec<u8> = vec![];
//     s.serialize(&mut s_blob).unwrap();
//
//     // Should succeed with max_loop_depth = 2
//     {
//         let storage = InMemoryStorage::new();
//         let vm = MoveVM::new_with_config(
//             // move_stdlib::natives::all_natives(
//             all_natives(
//                 AccountAddress3::from_hex_literal("0x1").unwrap(),
//                 move_stdlib::natives::GasParameters::zeros(),
//             ),
//             VMConfig {
//                 verifier: VerifierConfig {
//                     max_loop_depth: Some(2),
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             },
//         )
//             .unwrap();
//
//         let mut sess = vm.new_session(&storage);
//         let args: Vec<Vec<u8>> = vec![];
//         sess.execute_script(s_blob.clone(), vec![], args, &mut UnmeteredGasMeter)
//             .unwrap();
//     }
//
//     // Should fail with max_loop_depth = 1
//     {
//         let storage = InMemoryStorage::new();
//         let vm = MoveVM::new_with_config(
//             // move_stdlib::natives::all_natives(
//                 all_natives(
//
//                 AccountAddress3::from_hex_literal("0x1").unwrap(),
//                 move_stdlib::natives::GasParameters::zeros(),
//             ),
//             VMConfig {
//                 verifier: VerifierConfig {
//                     max_loop_depth: Some(1),
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             },
//         )
//             .unwrap();
//
//         let mut sess = vm.new_session(&storage);
//         let args: Vec<Vec<u8>> = vec![];
//         sess.execute_script(s_blob, vec![], args, &mut UnmeteredGasMeter)
//             .unwrap_err();
//     }
// }
//
// fn fuzz_target(ta: &AccountAddress3) {
//
//     test_publish_module_with_nested_loops(ta);
//     println!("fuzz_target called in main");
// }
//
// fn main() {
//     let m = AccountAddress3::default_mutator();
//     // let m = SampleStruct2::<u8, u8>::default_mutator();
//     test_mutator(m, 1000., 1000., false, true, 50, 50);
//     let result = fuzzcheck::fuzz_test(fuzz_target)
//         .default_mutator()
//         .serde_serializer()
//         .default_sensor_and_pool()
//         .arguments_from_cargo_fuzzcheck()
//         .stop_after_first_test_failure(true)
//         .launch();
//     println!("main for address.rs");
// }