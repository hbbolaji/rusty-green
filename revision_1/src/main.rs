#![allow(unused)]
use crate::{
    closure::test_closures,
    mydatetime::{test_chrono, test_std_time},
    myhash::{test_hashmap_basic, test_hashset_type},
    myiter::test_rust_iterators,
    myscopethreads::test_thread_variables,
    mythread::{test_spawn_thread, test_threads},
    myvec::{test_vec_int, test_vec_string, test_vect_car},
    option_enum::{test_option, test_option_chartype},
    pattern_matching::{test_match_array, test_match_int},
    structure::{create_vehicle, test_create_user},
    test_traits::create_person,
};

pub mod closure;
pub mod mydatetime;
pub mod myhash;
pub mod myiter;
pub mod myscopethreads;
pub mod mythread;
pub mod myvec;
pub mod option_enum;
pub mod pattern_matching;
pub mod structure;
pub mod test_traits;

fn main() {
    // test_closures();
    // test_match_int();
    // test_match_array();

    // let result = test_option();
    // let character = test_option_chartype();
    // println!("{0}", result.unwrap());
    // println!(
    //     "{:?} is the selected character",
    //     character.unwrap().to_string()
    // );

    // test_create_user();
    // create_vehicle();
    // create_person()

    // test_vec_int();
    // test_vec_string();
    // test_vect_car();

    // test_hashmap_basic();
    // test_hashset_type();
    // test_rust_iterators();

    // test_std_time();
    // test_chrono();

    // test_threads();
    // test_spawn_thread();

    test_thread_variables();
}
