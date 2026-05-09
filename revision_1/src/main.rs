use crate::{closure::test_closures, pattern_matching::{test_match_array, test_match_int}};

pub mod closure;
pub mod pattern_matching;

fn main() {
    test_closures();
    test_match_int();
    test_match_array();
}
