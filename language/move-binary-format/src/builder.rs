use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::process::exit;
// use std::result::Result;

use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{self, Result, Write};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
// use crate::CompiledModule;
use crate::file_format::{CompiledModule, empty_module, json_to_module, module_to_json, mutate_module};

/*
pub trait FuzzTestFunction<T, FT: ?Sized, ImplId> {
    type NormalizedFunction: for<'a> Fn(&'a T) -> bool;
    fn test_function(self) -> Self::NormalizedFunction;
}
 */

pub struct FuzzerBuilder<F>
    where
    // F: Fn(&CompiledModule) -> bool,
        F: Fn(&CompiledModule),
{
    pub test_function: F,
    // pub in_corpus_dir: PathBuf,
    // pub exec_path_file: PathBuf,
    // pub yj: usize,
}

pub fn fuzz_test<F>(test_function: F) ->  FuzzerBuilder<F>
    where
        F: Fn(&CompiledModule),
{
    FuzzerBuilder {
        test_function: test_function,
        // in_corpus_dir: Default::default(),
        // exec_path_file: Default::default(),
    }
}

impl<F> FuzzerBuilder<F>
    where
        F: Fn(&CompiledModule) + 'static,
{
    pub fn launch(self) -> Result<bool> {
        crate::fuzzer::launch(Box::new(self.test_function),
                              /*self.in_corpus_dir, self.exec_path_file*/);
        Ok(true)
    }
}