use crate::{
    closure::test_closures,
    option_enum::{test_option, test_option_chartype},
    pattern_matching::{test_match_array, test_match_int},
};

pub mod closure;
pub mod option_enum;
pub mod pattern_matching;

fn main() {
    test_closures();
    test_match_int();
    test_match_array();
    let result = test_option();
    let character = test_option_chartype();
    println!("{0}", result.unwrap());
    println!("{:?} is the selected character", character.unwrap().to_string())
}
