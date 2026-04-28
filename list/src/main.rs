fn main() {
    let mut nums = [1, 2, 3, 4, 5];
    nums[2] = 8;
    println!("{:?}", nums);

    let mut nums_vec = Vec::from([1, 2, 3, 4, 5, 6]);
    nums_vec.push(7);
    println!("{:?}", nums_vec);
    nums_vec.pop();
    nums_vec.pop();
    println!("{:?}, length = {}", nums_vec, nums_vec.len());
    println!("Sum of array numbers is {}", sum_of_numbers(&nums));
    println!("Sum of vector numbers is {}", sum_of_numbers(&nums_vec));

    // Slice
    let arr_slice = &nums[1..3];
    println!("Array slice: {:?}", arr_slice);

    let vec_slice = &nums_vec[..=3];
    println!("Vector slice: {:?}", vec_slice);

    // Using a vector macros
    let another_vec = vec![1, 2, 3];
    println!("{:?}", another_vec);
}

fn sum_of_numbers(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for i in numbers {
        sum += i;
    }
    return sum;
}
