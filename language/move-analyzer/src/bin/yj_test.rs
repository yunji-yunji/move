// #![allow(unused_attributes)]
#![feature(no_coverage)]
// #![feature(impl_trait_in_assoc_type)]
#[cfg(all(fuzzing, test))]
use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use fuzzcheck::{DefaultMutator, Mutator};
use fuzzcheck::mutators::tuples::TupleStructure;

#[derive(Serialize, Deserialize)]
//#[derive(fuzzcheck::DefaultMutator)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, DefaultMutator)]
struct SampleStruct1<T, U> {
    x: T,
    y: U,
    n: T,
}


#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, DefaultMutator)]
//#[derive(fuzzcheck::DefaultMutator)]
struct SampleStruct2<T, U> {
    x: T,
    y: U,
    n: U,
}



fn fuzz_target(s: &SampleStruct2<u16, u16>) {
    //if s.x > 3 {
    //    return;
    //} else {
    //    let _tmp = s.x + s.y + s.z;
    //}
    let mut i = 0;
    let mut t_x = s.x;
    let mut t_y = s.y;
    while t_x <= s.n {
        // println!("in target = {} {} {} {}", inp.x, inp.y, inp.n, i);
        if t_y >= t_x {
            t_x += 1;
        } else {
            t_y += 1;
        }
        i += 1;
        if i >5 { // if this is on, outgoing edge is two
            break;
        }
    }



//    if s.len() > 1 {
//        println!("length {:?}", s.len());
//    }
}

use fuzzcheck::mutators::testing_utilities::test_mutator;

fn main() {
    // my_m();
    println!("Hello, world! yjyj test main in test_prog");
//    mutator1();

    // let u16m1 :U16Mutator;
    // u16m1.initialize();

//    let m = YJStruct::default_mutator();

    let m = SampleStruct2::<u8, u8>::default_mutator();
    test_mutator(m, 1000., 1000., false, true, 50, 50);

//    let s1 = SampleStruct1::<u16, u16>::new((4, 5));
//    let s2 = SampleStruct2::<u16, u16>::new((4, 5, 99));
//    fuzz_target(s2);
//
    let result = fuzzcheck::fuzz_test(fuzz_target)
        .default_mutator()
        .serde_serializer()
        .default_sensor_and_pool()
        .arguments_from_cargo_fuzzcheck()
        .stop_after_first_test_failure(true)
        .launch();
    println!("after result");
    //    println!("after result {:?}", result);
    //    println!("u 16 mutator {:?} {:?}", u16m1.shuffled_integers, u16m1.rng);
    //let result = fuzz_test(fuzz_target);
    //println!("result booleanr = {:?}", result);
}

