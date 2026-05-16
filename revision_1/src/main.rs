#![allow(unused)]
use crate::{
    closure::test_closures, myargs::test_args, mychannel::test_channels, mydatetime::{test_chrono, test_std_time}, mydefault::test_default_impl, myfs::{test_create_dir, test_create_files, test_read_somefile, test_remove_dir}, myhash::{test_hashmap_basic, test_hashset_type}, myiter::test_rust_iterators, mymutex::test_mutext, myscopethreads::test_thread_variables, mythread::{test_spawn_thread, test_threads}, myvec::{test_vec_int, test_vec_string, test_vect_car}, option_enum::{test_option, test_option_chartype}, pattern_matching::{test_match_array, test_match_int}, structure::{create_vehicle, test_create_user}, test_trait::test_dyn_traits, test_traits::create_person
};

pub mod closure;
pub mod mychannel;
pub mod mydatetime;
pub mod myfs;
pub mod myhash;
pub mod myiter;
pub mod mymutex;
pub mod myscopethreads;
pub mod mythread;
pub mod myvec;
pub mod option_enum;
pub mod pattern_matching;
pub mod structure;
pub mod test_traits;
pub mod myargs;
pub mod test_trait;
pub mod mydefault;

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
    // test_thread_variables();
    // test_mutext();
    // test_channels();

    // test_create_dir();
    // test_create_files();
    // test_remove_dir();
    // test_read_somefile();

    // test_args();
    
    // test_dyn_traits();
    
    test_default_impl();
}
