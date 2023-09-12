// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0
#![allow(unused_attributes)]
#![feature(no_coverage)]
#![feature(impl_trait_in_assoc_type)]
// use serde::{Deserialize, Serialize};
use fuzzcheck::DefaultMutator;
use fuzzcheck::mutators::testing_utilities::test_mutator;
use fuzzcheck::mutators::tuples::TupleStructure;
use move_vm_integration_tests::tests::loader_tests::Adapter;

use move_core_types::account_address::AccountAddress;
// use move_core_types::account_address::AccountAddress;

use hex::FromHex;
use rand::{rngs::OsRng, Rng};
use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use std::{convert::TryFrom, fmt, str::FromStr,
          collections::{HashMap, VecDeque},};
use move_analyzer::{
    compiler::{as_module, as_script, compile_units},
    // address::AccountAddress,
    // address::AccountAddress3,
    // completion::on_completion_request,
    // context::Context,
    // symbols,
    // vfs::{on_text_document_sync_notification, VirtualFileSystem},
};
use std::{collections::BTreeSet, sync::Arc};
//
// use move_vm_runtime::{
//     config::VMConfig, data_cache::{AccountDataCache, TransactionDataCache}, native_extensions::NativeContextExtensions,
//     native_functions::{NativeFunctions, NativeFunction, make_table_from_iter, NativeFunctionTable},
//     // runtime::loader::{Function, Loader},
//     loader::{ModuleCache, TypeCache, ScriptCache},
//     // runtime::VMRuntime,
//     // session::Session,
// };
// use move_binary_format::{
//     errors::{Location, VMResult},
//     CompiledModule,
// };
// use move_core_types::{
//     account_address::AccountAddress, identifier::Identifier, language_storage::ModuleId,
//     metadata::Metadata, resolver::MoveResolver,
//     vm_status::StatusCode,
//     value::MoveTypeLayout,
// };
// use crate::compiler::{as_module, as_script, compile_units};
use move_bytecode_verifier::VerifierConfig;
// use move_core_types::account_address::AccountAddress2;
// use move_vm_runtime::{config::VMConfig};
use move_vm_runtime::{config::VMConfig, move_vm::MoveVM};
use move_vm_test_utils::InMemoryStorage;
use move_vm_types::gas::UnmeteredGasMeter;
// use move_vm_types::values::Value;
// use move_vm_runtime::native_functions::{make_table_from_iter, NativeFunctionTable};
// use move_stdlib::natives::GasParameters;

use tracing::warn;
use sha3::{Digest, Sha3_256};
/*
// #[derive(Serialize, Deserialize)]
// #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
// #[cfg_attr(any(test, feature = "fuzzing"), derive(proptest_derive::Arbitrary))]
// #[cfg_attr(any(test, feature = "fuzzing"), derive(arbitrary::Arbitrary))]
pub struct AccountAddress2([u8; AccountAddress2::LENGTH]);
// pub struct AccountAddress2([u8; 20]); // 20, 32, 16
impl AccountAddress2 {
    pub const fn new(address: [u8; Self::LENGTH]) -> Self {
        Self(address)
    }

    pub const LENGTH: usize = if 1==1 {
        20
    } else if 2==4 {
        32
    } else {
        16
    };
    pub fn from_hex_literal(literal: &str) -> Result<Self, AccountAddress2ParseError> {
        if !literal.starts_with("0x") {
            return Err(AccountAddress2ParseError);
        }

        let hex_len = literal.len() - 2;

        // If the string is too short, pad it
        if hex_len < Self::LENGTH * 2 {
            let mut hex_str = String::with_capacity(Self::LENGTH * 2);
            for _ in 0..Self::LENGTH * 2 - hex_len {
                hex_str.push('0');
            }
            hex_str.push_str(&literal[2..]);
            AccountAddress2::from_hex(hex_str)
        } else {
            AccountAddress2::from_hex(&literal[2..])
        }
    }

    pub fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, AccountAddress2ParseError> {
        <[u8; Self::LENGTH]>::from_hex(hex)
            .map_err(|_| AccountAddress2ParseError)
            .map(Self)
    }

}

impl FromStr for AccountAddress2 {
    type Err = AccountAddress2ParseError;

    fn from_str(s: &str) -> Result<Self, AccountAddress2ParseError> {
        // Accept 0xADDRESS or ADDRESS
        if let Ok(address) = AccountAddress2::from_hex_literal(s) {
            Ok(address)
        } else {
            Self::from_hex(s)
        }
    }
}

impl<'de> Deserialize<'de> for AccountAddress2 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let s = <String>::deserialize(deserializer)?;
            AccountAddress2::from_str(&s).map_err(D::Error::custom)
        } else {
            // In order to preserve the Serde data model and help analysis tools,
            // make sure to wrap our value in a container with the same name
            // as the original type.
            #[derive(::serde::Deserialize)]
            #[serde(rename = "AccountAddress2")]
            struct Value([u8; AccountAddress2::LENGTH]);

            let value = Value::deserialize(deserializer)?;
            Ok(AccountAddress2::new(value.0))
        }
    }
}


impl Serialize for AccountAddress2 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            self.to_hex().serialize(serializer)
        } else {
            // See comment in deserialize.
            serializer.serialize_newtype_struct("AccountAddress2", &self.0)
        }
    }
}

use std::{convert::TryFrom, fmt, str::FromStr};

#[derive(Clone, Copy, Debug)]
pub struct AccountAddress2ParseError;

impl fmt::Display for AccountAddress2ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Unable to parse AccountAddress2 (must be hex string of length {})",
            AccountAddress2::LENGTH
        )
    }
}


 */

// =====================================

use move_vm_runtime::runtime::VMRuntime;
// use move_binary_format::errors::PartialVMResult;
// use move_vm_types::natives::function::PartialVMResult;
/*
//===============
pub struct NativeFunctions2(
    HashMap<AccountAddress2, HashMap<String, HashMap<String, NativeFunction>>>,
);

impl NativeFunctions2 {
    // pub fn resolve(
    //     &self,
    //     addr: &AccountAddress,
    //     module_name: &str,
    //     func_name: &str,
    // ) -> Option<NativeFunction> {
    //     self.0.get(addr)?.get(module_name)?.get(func_name).cloned()
    // }

    pub fn new<I>(natives: I) -> PartialVMResult<Self>
        where
            I: IntoIterator<Item = (AccountAddress2, Identifier, Identifier, NativeFunction)>,
    {
        let mut map = HashMap::new();
        for (addr, module_name, func_name, func) in natives.into_iter() {
            let modules = map.entry(addr).or_insert_with(HashMap::new);
            let funcs = modules
                .entry(module_name.into_string())
                .or_insert_with(HashMap::new);

            if funcs.insert(func_name.into_string(), func).is_some() {
                return Err(PartialVMError::new(StatusCode::DUPLICATE_NATIVE_FUNCTION));
            }
        }
        Ok(Self(map))
    }
}

// =================
use std::collections::btree_map::BTreeMap;

pub struct TransactionDataCache2<'r, 'l, S> {
    // pub(crate) struct TransactionDataCache<'r, 'l, S> {
    remote: &'r S,
    loader: &'l Loader2,
    account_map: BTreeMap<AccountAddress, AccountDataCache>,
    event_data: Vec<(Vec<u8>, u64, Type, MoveTypeLayout, Value)>,
}

impl<'r, 'l, S: MoveResolver> TransactionDataCache2<'r, 'l, S> {
    /// Create a `TransactionDataCache` with a `RemoteCache` that provides access to data
    /// not updated in the transaction.
    // yunji edit
    pub fn new(remote: &'r S, loader: &'l Loader2) -> Self {
        TransactionDataCache2 {
            remote,
            loader,
            account_map: BTreeMap::new(),
            event_data: vec![],
        }
    }
}
use parking_lot::RwLock;
use move_binary_format::{
    access::{ ScriptAccess},
    binary_views::BinaryIndexedView,
    file_format::{
        AbilitySet, Bytecode, CompiledScript, Constant, ConstantPoolIndex,
        FieldHandleIndex, FieldInstantiationIndex, FunctionDefinition, FunctionDefinitionIndex,
        FunctionHandleIndex, FunctionInstantiationIndex, Signature, SignatureIndex, SignatureToken,
        StructDefInstantiationIndex, StructDefinition, StructDefinitionIndex,
        StructFieldInformation, TableIndex, TypeParameterIndex, Visibility,
    },
};

pub struct Loader2 {
    // yunji edit
    scripts: RwLock<ScriptCache>,
    module_cache: RwLock<ModuleCache>,
    type_cache: RwLock<TypeCache>,
    natives: NativeFunctions2,
    invalidated: RwLock<bool>,

    // Collects the cache hits on module loads. This information can be read and reset by
    // an adapter to reason about read/write conflicts of code publishing transactions and
    // other transactions.
    module_cache_hits: RwLock<BTreeSet<ModuleId>>,

    vm_config: VMConfig,
}

impl Loader2 {
    pub fn new(natives: NativeFunctions2, vm_config: VMConfig) -> Self {
        Self {
            scripts: RwLock::new(ScriptCache::new()),
            module_cache: RwLock::new(ModuleCache::new()),
            type_cache: RwLock::new(TypeCache::new()),
            natives,
            invalidated: RwLock::new(false),
            module_cache_hits: RwLock::new(BTreeSet::new()),
            vm_config,
        }
    }

    pub fn vm_config(&self) -> &VMConfig {
        &self.vm_config
    }

    /// Gets and clears module cache hits. A cache hit may also be caused indirectly by
    /// loading a function or a type. This not only returns the direct hit, but also
    /// indirect ones, that is all dependencies.
    pub fn get_and_clear_module_cache_hits(&self) -> BTreeSet<ModuleId> {
        let mut result = BTreeSet::new();
        let hits: BTreeSet<ModuleId> = std::mem::take(&mut self.module_cache_hits.write());
        for id in hits {
            self.transitive_dep_closure(&id, &mut result)
        }
        result
    }

    fn transitive_dep_closure(&self, id: &ModuleId, visited: &mut BTreeSet<ModuleId>) {
        if !visited.insert(id.clone()) {
            return;
        }
        let deps = self
            .module_cache
            .read()
            .modules
            .get(id)
            .unwrap()
            .module
            .immediate_dependencies();
        for dep in deps {
            self.transitive_dep_closure(&dep, visited)
        }
    }

    /// Flush this cache if it is marked as invalidated.
    pub fn flush_if_invalidated(&self) {
        let mut invalidated = self.invalidated.write();
        if *invalidated {
            *self.scripts.write() = ScriptCache::new();
            *self.module_cache.write() = ModuleCache::new();
            *self.type_cache.write() = TypeCache::new();
            *invalidated = false;
        }
    }

    /// Mark this cache as invalidated.
    // yunji edit
    pub fn mark_as_invalid(&self) {
        *self.invalidated.write() = true;
    }

    /// Check whether this cache is invalidated.
    pub fn is_invalidated(&self) -> bool {
        *self.invalidated.read()
    }

    /// Copies metadata out of a modules bytecode if available.
    pub fn get_metadata(&self, module: ModuleId, key: &[u8]) -> Option<Metadata> {
        let cache = self.module_cache.read();
        cache
            .modules
            .get(&module)
            .and_then(|module| module.module.metadata.iter().find(|md| md.key == key))
            .cloned()
    }

    //
    // Script verification and loading
    //

    // Scripts are verified and dependencies are loaded.
    // Effectively that means modules are cached from leaf to root in the dependency DAG.
    // If a dependency error is found, loading stops and the error is returned.
    // However all modules cached up to that point stay loaded.

    // Entry point for script execution (`MoveVM::execute_script`).
    // Verifies the script if it is not in the cache of scripts loaded.
    // Type parameters are checked as well after every type is loaded.
    pub fn load_script(
        &self,
        script_blob: &[u8],
        ty_args: &[TypeTag],
        data_store: &impl DataStore,
    ) -> VMResult<(Arc<Function>, LoadedFunctionInstantiation)> {
        // retrieve or load the script
        let mut sha3_256 = Sha3_256::new();
        sha3_256.update(script_blob);
        let hash_value: [u8; 32] = sha3_256.finalize().into();

        let mut scripts = self.scripts.write();
        let (main, parameters, return_) = match scripts.get(&hash_value) {
            Some(cached) => cached,
            None => {
                let ver_script = self.deserialize_and_verify_script(script_blob, data_store)?;
                let script = Script::new(ver_script, &hash_value, &self.module_cache.read())?;
                scripts.insert(hash_value, script)
            }
        };

        // verify type arguments
        let mut type_arguments = vec![];
        for ty in ty_args {
            type_arguments.push(self.load_type(ty, data_store)?);
        }
        self.verify_ty_args(main.type_parameters(), &type_arguments)
            .map_err(|e| e.finish(Location::Script))?;
        let instantiation = LoadedFunctionInstantiation {
            type_arguments,
            parameters,
            return_,
        };
        Ok((main, instantiation))
    }

    // The process of deserialization and verification is not and it must not be under lock.
    // So when publishing modules through the dependency DAG it may happen that a different
    // thread had loaded the module after this process fetched it from storage.
    // Caching will take care of that by asking for each dependency module again under lock.
    fn deserialize_and_verify_script(
        &self,
        script: &[u8],
        data_store: &impl DataStore,
    ) -> VMResult<CompiledScript> {
        let script = match CompiledScript::deserialize_with_max_version(
            script,
            self.vm_config.max_binary_format_version,
        ) {
            Ok(script) => script,
            Err(err) => {
                error!("[VM] deserializer for script returned error: {:?}", err,);
                let msg = format!("Deserialization error: {:?}", err);
                return Err(PartialVMError::new(StatusCode::CODE_DESERIALIZATION_ERROR)
                    .with_message(msg)
                    .finish(Location::Script));
            }
        };

        match self.verify_script(&script) {
            Ok(_) => {
                // verify dependencies
                let loaded_deps = script
                    .immediate_dependencies()
                    .into_iter()
                    .map(|module_id| self.load_module(&module_id, data_store))
                    .collect::<VMResult<_>>()?;
                self.verify_script_dependencies(&script, loaded_deps)?;
                Ok(script)
            }
            Err(err) => {
                error!(
                    "[VM] bytecode verifier returned errors for script: {:?}",
                    err
                );
                Err(err)
            }
        }
    }

    // Script verification steps.
    // See `verify_module()` for module verification steps.
    fn verify_script(&self, script: &CompiledScript) -> VMResult<()> {
        fail::fail_point!("verifier-failpoint-3", |_| { Ok(()) });

        move_bytecode_verifier::verify_script_with_config(&self.vm_config.verifier, script)
    }

    fn verify_script_dependencies(
        &self,
        script: &CompiledScript,
        dependencies: Vec<Arc<Module>>,
    ) -> VMResult<()> {
        let mut deps = vec![];
        for dep in &dependencies {
            deps.push(dep.module());
        }
        dependencies::verify_script(script, deps)
    }

    //
    // Module verification and loading
    //

    // Entry point for function execution (`MoveVM::execute_function`).
    // Loading verifies the module if it was never loaded.
    // Type parameters are checked as well after every type is loaded.
    pub fn load_function(
        &self,
        module_id: &ModuleId,
        function_name: &IdentStr,
        ty_args: &[TypeTag],
        data_store: &impl DataStore,
    ) -> VMResult<(Arc<Module>, Arc<Function>, LoadedFunctionInstantiation)> {
        let module = self.load_module(module_id, data_store)?;
        let idx = self
            .module_cache
            .read()
            .resolve_function_by_name(function_name, module_id)
            .map_err(|err| err.finish(Location::Undefined))?;
        let func = self.module_cache.read().function_at(idx);

        let parameters = func
            .parameters
            .0
            .iter()
            .map(|tok| {
                self.module_cache
                    .read()
                    .make_type(BinaryIndexedView::Module(module.module()), tok)
            })
            .collect::<PartialVMResult<Vec<_>>>()
            .map_err(|err| err.finish(Location::Undefined))?;

        let return_ = func
            .return_
            .0
            .iter()
            .map(|tok| {
                self.module_cache
                    .read()
                    .make_type(BinaryIndexedView::Module(module.module()), tok)
            })
            .collect::<PartialVMResult<Vec<_>>>()
            .map_err(|err| err.finish(Location::Undefined))?;

        // verify type arguments
        let type_arguments = ty_args
            .iter()
            .map(|ty| self.load_type(ty, data_store))
            .collect::<VMResult<Vec<_>>>()?;
        self.verify_ty_args(func.type_parameters(), &type_arguments)
            .map_err(|e| e.finish(Location::Module(module_id.clone())))?;

        let loaded = LoadedFunctionInstantiation {
            type_arguments,
            parameters,
            return_,
        };
        Ok((module, func, loaded))
    }

}


//=================
/// An instantiation of the MoveVM.
pub struct VMRuntime2 {
    // pub(crate) struct VMRuntime {
    loader: Loader2,
}
use move_binary_format::{
    access::ModuleAccess,
    compatibility::Compatibility,
    errors::{verification_error, Location, PartialVMError, PartialVMResult, VMResult},
    file_format::LocalIndex,
    normalized, CompiledModule, IndexKind,
};
impl VMRuntime2 {
    // yunji edit
    pub fn new(
        natives: impl IntoIterator<Item=(AccountAddress2, Identifier, Identifier, NativeFunction)>,
        vm_config: VMConfig,
    ) -> PartialVMResult<Self> {
        Ok(VMRuntime2 {
            loader: Loader2::new(NativeFunctions2::new(natives)?, vm_config),
        })
    }

    pub fn new_session<'r, S: MoveResolver>(&self, remote: &'r S) -> Session2<'r, '_, S> {
        self.new_session_with_extensions(remote, NativeContextExtensions::default())
    }

    pub fn new_session_with_extensions<'r, S: MoveResolver>(
        &self,
        remote: &'r S,
        native_extensions: NativeContextExtensions<'r>,
    ) -> Session2<'r, '_, S> {
        Session2 {
            runtime: self,
            data_cache: TransactionDataCache::new(remote, &self.loader),
            native_extensions,
        }
    }

    pub(crate) fn publish_module_bundle(
        &self,
        modules: Vec<Vec<u8>>,
        sender: AccountAddress,
        data_store: &mut impl DataStore,
        _gas_meter: &mut impl GasMeter,
        compat: Compatibility,
    ) -> VMResult<()> {
        // deserialize the modules. Perform bounds check. After this indexes can be
        // used with the `[]` operator
        let compiled_modules = match modules
            .iter()
            .map(|blob| {
                CompiledModule::deserialize_with_max_version(
                    blob,
                    self.loader.vm_config().max_binary_format_version,
                )
            })
            .collect::<PartialVMResult<Vec<_>>>()
        {
            Ok(modules) => modules,
            Err(err) => {
                warn!("[VM] module deserialization failed {:?}", err);
                return Err(err.finish(Location::Undefined));
            }
        };

        // Make sure all modules' self addresses matches the transaction sender. The self address is
        // where the module will actually be published. If we did not check this, the sender could
        // publish a module under anyone's account.
        for module in &compiled_modules {
            if module.address() != &sender {
                return Err(verification_error(
                    StatusCode::MODULE_ADDRESS_DOES_NOT_MATCH_SENDER,
                    IndexKind::AddressIdentifier,
                    module.self_handle_idx().0,
                )
                    .finish(Location::Undefined));
            }
        }

        // Collect ids for modules that are published together
        let mut bundle_unverified = BTreeSet::new();

        // For now, we assume that all modules can be republished, as long as the new module is
        // backward compatible with the old module.
        //
        // TODO: in the future, we may want to add restrictions on module republishing, possibly by
        // changing the bytecode format to include an `is_upgradable` flag in the CompiledModule.
        for module in &compiled_modules {
            let module_id = module.self_id();

            if data_store.exists_module(&module_id)? && compat.need_check_compat() {
                let old_module_ref = self.loader.load_module(&module_id, data_store)?;
                let old_module = old_module_ref.module();
                let old_m = normalized::Module::new(old_module);
                let new_m = normalized::Module::new(module);
                compat
                    .check(&old_m, &new_m)
                    .map_err(|e| e.finish(Location::Undefined))?;
            }
            if !bundle_unverified.insert(module_id) {
                return Err(PartialVMError::new(StatusCode::DUPLICATE_MODULE_NAME)
                    .finish(Location::Undefined));
            }
        }

        // Perform bytecode and loading verification. Modules must be sorted in topological order.
        self.loader
            .verify_module_bundle_for_publication(&compiled_modules, data_store)?;


        // All modules verified, publish them to data cache
        for (module, blob) in compiled_modules.into_iter().zip(modules.into_iter()) {
            let is_republishing = data_store.exists_module(&module.self_id())?;
            if is_republishing {
                // This is an upgrade, so invalidate the loader cache, which still contains the
                // old module.
                self.loader.mark_as_invalid();
            }
            data_store.publish_module(&module.self_id(), blob, is_republishing)?;
        }
        Ok(())
    }

    pub fn loader(&self) -> &Loader2 {
        &self.loader
    }

}
// ==================


pub struct MoveVM2 {
    runtime: VMRuntime2,
}

impl MoveVM2 {
    pub fn new(
        natives: impl IntoIterator<Item = (AccountAddress2, Identifier, Identifier, NativeFunction)>,
    ) -> VMResult<Self> {
        Self::new_with_config(natives, VMConfig::default())
    }

    pub fn new_with_config(
        natives: impl IntoIterator<Item = (AccountAddress2, Identifier, Identifier, NativeFunction)>,
        vm_config: VMConfig,
    ) -> VMResult<Self> {
        Ok(Self {
            runtime: VMRuntime2::new(natives, vm_config)
                .map_err(|err| err.finish(Location::Undefined))?,
        })
    }

    pub fn new_session<'r, S: MoveResolver>(&self, remote: &'r S) -> Session2<'r, '_, S> {
        self.runtime.new_session(remote)
    }

    /// Create a new session, as in `new_session`, but provide native context extensions.
    pub fn new_session_with_extensions<'r, S: MoveResolver>(
        &self,
        remote: &'r S,
        extensions: NativeContextExtensions<'r>,
    ) -> Session2<'r, '_, S> {
        self.runtime.new_session_with_extensions(remote, extensions)
    }

    /// Load a module into VM's code cache
    pub fn load_module<'r, S: MoveResolver>(
        &self,
        module_id: &ModuleId,
        remote: &'r S,
    ) -> VMResult<Arc<CompiledModule>> {
        self.runtime
            .loader()
            .load_module(
                module_id,
                &TransactionDataCache::new(remote, self.runtime.loader()),
            )
            .map(|arc_module| arc_module.arc_module())
    }

    pub fn mark_loader_cache_as_invalid(&self) {
        self.runtime.loader().mark_as_invalid()
    }

    pub fn is_loader_cache_invalidated(&self) -> bool {
        self.runtime.loader().is_invalidated()
    }

    pub fn flush_loader_cache_if_invalidated(&self) {
        self.runtime.loader().flush_if_invalidated()
    }

    pub fn get_and_clear_module_cache_hits(&self) -> BTreeSet<ModuleId> {
        self.runtime.loader().get_and_clear_module_cache_hits()
    }

    pub fn get_module_metadata(&self, module: ModuleId, key: &[u8]) -> Option<Metadata> {
        self.runtime.loader().get_metadata(module, key)
    }
}






//==============
#[derive(Clone, PartialEq, Eq, Hash, DefaultMutator)]
pub struct AccountAddress2([u8; AccountAddress2::LENGTH]);

impl AccountAddress2 {
    pub const fn new(address: [u8; Self::LENGTH]) -> Self {
        Self(address)
    }

    /// The number of bytes in an address.
    /// Default to 16 bytes, can be set to 20 bytes with address20 feature.
    pub const LENGTH: usize = if cfg!(feature = "address20") {
        20
    } else if cfg!(feature = "address32") {
        32
    } else {
        16
    };

    /// Hex address: 0x0
    pub const ZERO: Self = Self([0u8; Self::LENGTH]);

    /// Hex address: 0x1
    pub const ONE: Self = Self::get_hex_address_one();

    /// Hex address: 0x2
    pub const TWO: Self = Self::get_hex_address_two();

    const fn get_hex_address_one() -> Self {
        let mut addr = [0u8; AccountAddress2::LENGTH];
        addr[AccountAddress2::LENGTH - 1] = 1u8;
        Self(addr)
    }

    const fn get_hex_address_two() -> Self {
        let mut addr = [0u8; AccountAddress2::LENGTH];
        addr[AccountAddress2::LENGTH - 1] = 2u8;
        Self(addr)
    }

    pub fn random() -> Self {
        let mut rng = OsRng;
        let buf: [u8; Self::LENGTH] = rng.gen();
        Self(buf)
    }

    /// Return a canonical string representation of the address
    /// Addresses are hex-encoded lowercase values of length ADDRESS_LENGTH (16, 20, or 32 depending on the Move platform)
    /// e.g., 0000000000000000000000000000000a, *not* 0x0000000000000000000000000000000a, 0xa, or 0xA
    /// Note: this function is guaranteed to be stable, and this is suitable for use inside
    /// Move native functions or the VM.
    pub fn to_canonical_string(&self) -> String {
        hex::encode(self.0)
    }

    pub fn short_str_lossless(&self) -> String {
        let hex_str = hex::encode(self.0).trim_start_matches('0').to_string();
        if hex_str.is_empty() {
            "0".to_string()
        } else {
            hex_str
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn into_bytes(self) -> [u8; Self::LENGTH] {
        self.0
    }

    pub fn from_hex_literal(literal: &str) -> Result<Self, AccountAddress2ParseError> {
        if !literal.starts_with("0x") {
            return Err(AccountAddress2ParseError);
        }

        let hex_len = literal.len() - 2;

        // If the string is too short, pad it
        if hex_len < Self::LENGTH * 2 {
            let mut hex_str = String::with_capacity(Self::LENGTH * 2);
            for _ in 0..Self::LENGTH * 2 - hex_len {
                hex_str.push('0');
            }
            hex_str.push_str(&literal[2..]);
            AccountAddress2::from_hex(hex_str)
        } else {
            AccountAddress2::from_hex(&literal[2..])
        }
    }

    pub fn to_hex_literal(&self) -> String {
        format!("0x{}", self.short_str_lossless())
    }

    pub fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, AccountAddress2ParseError> {
        <[u8; Self::LENGTH]>::from_hex(hex)
            .map_err(|_| AccountAddress2ParseError)
            .map(Self)
    }

    pub fn to_hex(&self) -> String {
        format!("{:x}", self)
    }

    pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, AccountAddress2ParseError> {
        <[u8; Self::LENGTH]>::try_from(bytes.as_ref())
            .map_err(|_| AccountAddress2ParseError)
            .map(Self)
    }
}

impl AsRef<[u8]> for AccountAddress2 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl std::ops::Deref for AccountAddress2 {
    type Target = [u8; Self::LENGTH];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for AccountAddress2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:x}", self)
    }
}

impl fmt::Debug for AccountAddress2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self)
    }
}

impl fmt::LowerHex for AccountAddress2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }

        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
    }
}

impl fmt::UpperHex for AccountAddress2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }

        for byte in &self.0 {
            write!(f, "{:02X}", byte)?;
        }

        Ok(())
    }
}

impl From<[u8; AccountAddress2::LENGTH]> for AccountAddress2 {
    fn from(bytes: [u8; AccountAddress2::LENGTH]) -> Self {
        Self::new(bytes)
    }
}

impl TryFrom<&[u8]> for AccountAddress2 {
    type Error = AccountAddress2ParseError;

    /// Tries to convert the provided byte array into Address.
    fn try_from(bytes: &[u8]) -> Result<AccountAddress2, AccountAddress2ParseError> {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<Vec<u8>> for AccountAddress2 {
    type Error = AccountAddress2ParseError;

    /// Tries to convert the provided byte buffer into Address.
    fn try_from(bytes: Vec<u8>) -> Result<AccountAddress2, AccountAddress2ParseError> {
        Self::from_bytes(bytes)
    }
}

impl From<AccountAddress2> for Vec<u8> {
    fn from(addr: AccountAddress2) -> Vec<u8> {
        addr.0.to_vec()
    }
}

impl From<&AccountAddress2> for Vec<u8> {
    fn from(addr: &AccountAddress2) -> Vec<u8> {
        addr.0.to_vec()
    }
}

impl From<AccountAddress2> for [u8; AccountAddress2::LENGTH] {
    fn from(addr: AccountAddress2) -> Self {
        addr.0
    }
}

impl From<&AccountAddress2> for [u8; AccountAddress2::LENGTH] {
    fn from(addr: &AccountAddress2) -> Self {
        addr.0
    }
}

impl From<&AccountAddress2> for String {
    fn from(addr: &AccountAddress2) -> String {
        ::hex::encode(addr.as_ref())
    }
}

impl TryFrom<String> for AccountAddress2 {
    type Error = AccountAddress2ParseError;

    fn try_from(s: String) -> Result<AccountAddress2, AccountAddress2ParseError> {
        Self::from_hex(s)
    }
}

impl FromStr for AccountAddress2 {
    type Err = AccountAddress2ParseError;

    fn from_str(s: &str) -> Result<Self, AccountAddress2ParseError> {
        // Accept 0xADDRESS or ADDRESS
        if let Ok(address) = AccountAddress2::from_hex_literal(s) {
            Ok(address)
        } else {
            Self::from_hex(s)
        }
    }
}

impl<'de> Deserialize<'de> for AccountAddress2 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let s = <String>::deserialize(deserializer)?;
            AccountAddress2::from_str(&s).map_err(D::Error::custom)
        } else {
            // In order to preserve the Serde data model and help analysis tools,
            // make sure to wrap our value in a container with the same name
            // as the original type.
            #[derive(::serde::Deserialize)]
            #[serde(rename = "AccountAddress2")]
            struct Value([u8; AccountAddress2::LENGTH]);

            let value = Value::deserialize(deserializer)?;
            Ok(AccountAddress2::new(value.0))
        }
    }
}

impl Serialize for AccountAddress2 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        if serializer.is_human_readable() {
            self.to_hex().serialize(serializer)
        } else {
            // See comment in deserialize.
            serializer.serialize_newtype_struct("AccountAddress2", &self.0)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AccountAddress2ParseError;

impl fmt::Display for AccountAddress2ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Unable to parse AccountAddress2 (must be hex string of length {})",
            AccountAddress2::LENGTH
        )
    }
}

impl std::error::Error for AccountAddress2ParseError {}



//===================
use move_vm_types::{
    data_store::DataStore,
    gas::GasMeter,
    loaded_data::runtime_types::{CachedStructIndex, StructType, Type},
};

pub struct Session2<'r, 'l, S> {
    pub(crate) runtime: &'l VMRuntime2,
    pub(crate) data_cache: TransactionDataCache<'r, 'l, S>,
    pub(crate) native_extensions: NativeContextExtensions<'r>,
}

impl<'r, 'l, S: MoveResolver> Session2<'r, 'l, S> {
    pub fn publish_module(
        &mut self,
        module: Vec<u8>,
        sender: AccountAddress2,
        gas_meter: &mut impl GasMeter,
    ) -> VMResult<()> {
        self.publish_module_bundle(vec![module], sender, gas_meter)
    }
}
// =================

use move_stdlib::natives::bcs;
use move_stdlib::natives::debug;
use move_stdlib::natives::event;
use move_stdlib::natives::hash;
use move_stdlib::natives::signer;
use move_stdlib::natives::string;
use move_stdlib::natives::type_name;
use move_stdlib::natives::unit_test;
use move_stdlib::natives::vector;
/*

#[derive(Debug, Clone)]
pub struct GasParameters {
    pub bcs: bcs::GasParameters,
    pub hash: hash::GasParameters,
    pub signer: signer::GasParameters,
    pub string: string::GasParameters,
    pub type_name: type_name::GasParameters,
    pub vector: vector::GasParameters,

    #[cfg(feature = "testing")]
    pub unit_test: unit_test::GasParameters,
}

impl GasParameters {
    pub fn zeros() -> Self {
        Self {
            bcs: bcs::GasParameters {
                to_bytes: bcs::ToBytesGasParameters {
                    per_byte_serialized: 0.into(),
                    legacy_min_output_size: 0.into(),
                    failure: 0.into(),
                },
            },

            hash: hash::GasParameters {
                sha2_256: hash::Sha2_256GasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                    legacy_min_input_len: 0.into(),
                },
                sha3_256: hash::Sha3_256GasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                    legacy_min_input_len: 0.into(),
                },
            },
            type_name: type_name::GasParameters {
                get: type_name::GetGasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
            },
            signer: signer::GasParameters {
                borrow_address: signer::BorrowAddressGasParameters { base: 0.into() },
            },
            string: string::GasParameters {
                check_utf8: string::CheckUtf8GasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
                is_char_boundary: string::IsCharBoundaryGasParameters { base: 0.into() },
                sub_string: string::SubStringGasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
                index_of: string::IndexOfGasParameters {
                    base: 0.into(),
                    per_byte_pattern: 0.into(),
                    per_byte_searched: 0.into(),
                },
            },
            vector: vector::GasParameters {
                empty: vector::EmptyGasParameters { base: 0.into() },
                length: vector::LengthGasParameters { base: 0.into() },
                push_back: vector::PushBackGasParameters {
                    base: 0.into(),
                    legacy_per_abstract_memory_unit: 0.into(),
                },
                borrow: vector::BorrowGasParameters { base: 0.into() },
                pop_back: vector::PopBackGasParameters { base: 0.into() },
                destroy_empty: vector::DestroyEmptyGasParameters { base: 0.into() },
                swap: vector::SwapGasParameters { base: 0.into() },
            },
            #[cfg(feature = "testing")]
            unit_test: unit_test::GasParameters {
                create_signers_for_testing: unit_test::CreateSignersForTestingGasParameters {
                    base_cost: 0.into(),
                    unit_cost: 0.into(),
                },
            },
        }
    }
}
*/

pub fn all_natives(
    move_std_addr: AccountAddress2,
    gas_params: GasParameters,
) -> NativeFunctionTable {
    let mut natives = vec![];

    macro_rules! add_natives {
        ($module_name: expr, $natives: expr) => {
            natives.extend(
                $natives.map(|(func_name, func)| ($module_name.to_string(), func_name, func)),
            );
        };
    }

    add_natives!("bcs", bcs::make_all(gas_params.bcs));
    add_natives!("hash", hash::make_all(gas_params.hash));
    add_natives!("signer", signer::make_all(gas_params.signer));
    add_natives!("string", string::make_all(gas_params.string));
    add_natives!("type_name", type_name::make_all(gas_params.type_name));
    add_natives!("vector", vector::make_all(gas_params.vector));
    #[cfg(feature = "testing")]
    {
        add_natives!("unit_test", unit_test::make_all(gas_params.unit_test));
    }

    make_table_from_iter(move_std_addr, natives)
}

*/

// pub struct AccountAddress2([u8; AccountAddress2::LENGTH]);
const TEST_ADDR: AccountAddress = AccountAddress::new([42; AccountAddress::LENGTH]);

// #[test]
fn fuzz_target(test_addr: &AccountAddress) {
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
    let code = code.replace("{{ADDR}}", &format!("0x{:?}", *test_addr));
    // let code = code.replace("{{ADDR}}", &format!("0x{}", TEST_ADDR));
    let mut units = compile_units(&code).unwrap();

    let m = as_module(units.pop().unwrap());
    let mut m_blob = vec![];
    m.serialize(&mut m_blob).unwrap();

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
        sess.publish_module(m_blob.clone(), TEST_ADDR, &mut UnmeteredGasMeter)
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
        sess.publish_module(m_blob, TEST_ADDR, &mut UnmeteredGasMeter)
            .unwrap_err();
    }
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


// yunji code =============================================
// #[derive(Serialize, Deserialize)]
// #[derive(Clone, Debug, PartialEq, Eq, Hash, DefaultMutator)]
// struct SampleStruct2<T, U> {
//     x: T,
//     y: U,
//     n: U,
// }


// const TEST_ADDR: AccountAddress2 = AccountAddress2::new([42; AccountAddress2::LENGTH]);



// fn fuzz_target(ta: &AccountAddress) {
//
//     test_publish_module_with_nested_loops(ta);
//     println!("fuzz_target called in main");
// }

fn main() {
    println!("Main function in bad_entry_point file.");
    // let aa = AccountAddress2::new([42; AccountAddress2::LENGTH]);
    let m = AccountAddress::default_mutator();
    // let m = SampleStruct2::<u8, u8>::default_mutator();
    test_mutator(m, 1000., 1000., false, true, 50, 50);
    let result = fuzzcheck::fuzz_test(fuzz_target)
        .default_mutator()
        .serde_serializer()
        .default_sensor_and_pool()
        .arguments_from_cargo_fuzzcheck()
        .stop_after_first_test_failure(true)
        .launch();
    println!("DONE execution yjyj");
}


// yunji code =============================================
