use std::ops::AddAssign;

pub fn sum_of_elements_generic<T: AddAssign + Copy>(nums: &[T]) -> T {
    let mut sum: T = nums[0 ];
    for i in 1..nums.len() {
        sum += nums[i]
    }
    return sum
}

fn main() {
    let my_nums = [1, 2, 3, 4, 5];
    let your_nums = [6, 7, 8, 9, 10];

    let my_sum = sum_of_elements_generic(&my_nums);
    let your_sum = sum_of_elements_generic(&your_nums);

    let my_i64_nums: [i64; 5] = [11, 12, 13, 14, 15];
    let my_i64_sum = sum_of_elements_generic(&my_i64_nums);

    println!("{my_sum}, {your_sum}, {my_i64_sum}"); 
}
