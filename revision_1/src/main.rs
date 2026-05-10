use crate::test_traits::create_person;
#[allow(unused_imports)]
use crate::{
    closure::test_closures,
    option_enum::{test_option, test_option_chartype},
    pattern_matching::{test_match_array, test_match_int},
    structure::{create_vehicle, test_create_user},
};

pub mod closure;
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
    // test_create_user();
    // create_vehicle();
    // println!("{0}", result.unwrap());
    // println!(
    //     "{:?} is the selected character",
    //     character.unwrap().to_string()
    // );
    create_person()
}
