// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0
#![feature(no_coverage)]
#![allow(clippy::new_without_default)]

mod storage;

pub mod gas_schedule;
pub use storage::{BlankStorage, DeltaStorage, InMemoryStorage};
