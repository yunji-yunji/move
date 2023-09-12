


#![allow(unused_attributes)]
#![feature(no_coverage)]
#![feature(impl_trait_in_assoc_type)]
// use serde::{Deserialize, Serialize};
use fuzzcheck::DefaultMutator;

#[derive(Clone, PartialEq, Eq, Hash, DefaultMutator)]
pub struct YJ{
    a:u8,
    b:usize,
}

use account_address::AccountAddress;

fn main() {
    println!("need to call this file");
}