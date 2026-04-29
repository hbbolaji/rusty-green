fn func_that_panics(num: u8) {
    if num > 5 {
        panic!("I can't handle numbers that large");
    }
    println!("This number is okay");
}

fn funct_that_returns_result(num: u8) -> Result<u8, String> {
    if num > u8::MAX / 2 {
        return Err("This number would overflow if doubled".into())
    }
    Ok(num * 2)
}

fn main() {
    // func_that_panics(3);
    // func_that_panics(10);
    // println!("150 * 2 = {}", funct_that_returns_result(150).expect("Function panic at 150"))

    // match
    // let double_num = match funct_that_returns_result(150) {
    //     Ok(result) => result,
    //     Err(e) => {
    //         println!("There was an error: {}", e);
    //         println!("Defaulting to original number: 150");
    //         150
    //     }
    // };

    // println!("the resulting number is {}", double_num);

    if let Ok(value) = funct_that_returns_result(150) {
        println!("The resulting number is {}", value);
    } else {
        println!("Could not double number")
    }
}