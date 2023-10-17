// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

use std::fmt;

pub mod access;
pub mod binary_views;
pub mod check_bounds;
pub mod compatibility;
#[macro_use]
pub mod errors;
pub mod constant;
pub mod control_flow_graph;
pub mod deserializer;
pub mod file_format;
pub mod file_format_common;
pub mod internals;
pub mod normalized;
#[cfg(any(test, feature = "fuzzing"))]
pub mod proptest_types;
pub mod serializer;
pub mod views;

pub mod fuzzer;
pub mod builder;

#[cfg(test)]
mod unit_tests;

pub use file_format::CompiledModule;

use serde::{Serialize, Deserialize};
use serde::{Serializer};

// #[derive(Serialize, Deserialize)]
/// Represents a kind of index -- useful for error messages.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IndexKind {
    ModuleHandle,
    StructHandle,
    FunctionHandle,
    FieldHandle,
    FriendDeclaration,
    FunctionInstantiation,
    FieldInstantiation,
    StructDefinition,
    StructDefInstantiation,
    FunctionDefinition,
    FieldDefinition,
    Signature,
    Identifier,
    AddressIdentifier,
    ConstantPool,
    LocalPool,
    CodeDefinition,
    TypeParameter,
    MemberCount,
}
impl Serialize for IndexKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S:Serializer,
    {

        match *self {
            IndexKind::ModuleHandle => serializer.serialize_str("module handle"),
            IndexKind::StructHandle => serializer.serialize_str("struct handle"),
            IndexKind::FunctionHandle => serializer.serialize_str("function handle"),
            IndexKind::FieldHandle => serializer.serialize_str("field handle"),
            IndexKind::FriendDeclaration => serializer.serialize_str("friend declaration"),
            IndexKind::StructDefInstantiation => serializer.serialize_str("struct instantiation"),
            IndexKind::FunctionInstantiation => serializer.serialize_str("function instantiation"),
            IndexKind::FieldInstantiation => serializer.serialize_str("field instantiation"),
            IndexKind::StructDefinition => serializer.serialize_str("struct definition"),
            IndexKind::FunctionDefinition => serializer.serialize_str("function definition"),
            IndexKind::FieldDefinition => serializer.serialize_str("field definition"),
            IndexKind::Signature => serializer.serialize_str("signature"),
            IndexKind::Identifier => serializer.serialize_str("identifier"),
            IndexKind::AddressIdentifier => serializer.serialize_str("address identifier"),
            IndexKind::ConstantPool => serializer.serialize_str("constant pool"),
            IndexKind::LocalPool => serializer.serialize_str("local pool"),
            IndexKind::CodeDefinition => serializer.serialize_str("code definition pool"),
            IndexKind::TypeParameter => serializer.serialize_str("type parameter"),
            IndexKind::MemberCount => serializer.serialize_str("field offset"),
        }
    }
}
impl IndexKind {
    pub fn variants() -> &'static [IndexKind] {
        use IndexKind::*;

        // XXX ensure this list stays up to date!
        &[
            ModuleHandle,
            StructHandle,
            FunctionHandle,
            FieldHandle,
            FriendDeclaration,
            StructDefInstantiation,
            FunctionInstantiation,
            FieldInstantiation,
            StructDefinition,
            FunctionDefinition,
            FieldDefinition,
            Signature,
            Identifier,
            ConstantPool,
            LocalPool,
            CodeDefinition,
            TypeParameter,
            MemberCount,
        ]
    }
}

impl fmt::Display for IndexKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use IndexKind::*;

        let desc = match self {
            ModuleHandle => "module handle",
            StructHandle => "struct handle",
            FunctionHandle => "function handle",
            FieldHandle => "field handle",
            FriendDeclaration => "friend declaration",
            StructDefInstantiation => "struct instantiation",
            FunctionInstantiation => "function instantiation",
            FieldInstantiation => "field instantiation",
            StructDefinition => "struct definition",
            FunctionDefinition => "function definition",
            FieldDefinition => "field definition",
            Signature => "signature",
            Identifier => "identifier",
            AddressIdentifier => "address identifier",
            ConstantPool => "constant pool",
            LocalPool => "local pool",
            CodeDefinition => "code definition pool",
            TypeParameter => "type parameter",
            MemberCount => "field offset",
        };

        f.write_str(desc)
    }
}

// TODO: is this outdated?
/// Represents the kind of a signature token.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SignatureTokenKind {
    /// Any sort of owned value that isn't an array (Integer, Bool, Struct etc).
    Value,
    /// A reference.
    Reference,
    /// A mutable reference.
    MutableReference,
}

impl fmt::Display for SignatureTokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use SignatureTokenKind::*;

        let desc = match self {
            Value => "value",
            Reference => "reference",
            MutableReference => "mutable reference",
        };

        f.write_str(desc)
    }
}

/// A macro which should be preferred in critical runtime paths for unwrapping an option
/// if a `PartialVMError` is expected. In debug mode, this will panic. Otherwise
/// we return an Err.
#[macro_export]
macro_rules! safe_unwrap {
    ($e:expr) => {{
        match $e {
            Some(x) => x,
            None => {
                let err = PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                    .with_message(format!("{}:{} (none)", file!(), line!()));
                if cfg!(debug_assertions) {
                    panic!("{:?}", err);
                } else {
                    return Err(err);
                }
            }
        }
    }};
}

/// Similar as above but for Result
#[macro_export]
macro_rules! safe_unwrap_err {
    ($e:expr) => {{
        match $e {
            Ok(x) => x,
            Err(e) => {
                let err = PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                    .with_message(format!("{}:{} {:#}", file!(), line!(), e));
                if cfg!(debug_assertions) {
                    panic!("{:?}", err);
                } else {
                    return Err(err);
                }
            }
        }
    }};
}

/// Similar as above, but asserts a boolean expression to be true.
#[macro_export]
macro_rules! safe_assert {
    ($e:expr) => {{
        if !$e {
            let err = PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                .with_message(format!("{}:{} (assert)", file!(), line!()));
            if cfg!(debug_assertions) {
                panic!("{:?}", err)
            } else {
                return Err(err);
            }
        }
    }};
}
