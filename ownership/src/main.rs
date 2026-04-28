fn main() {
    let a: i32 = 8;
    let b = a;
    println!("a = {} and b = {}", a, b);

    {
        let c = 15;
        println!("c = {}", c);
    }

    let target_name;

    {
        // let mut name = "Rust";
        let mut name: String = String::from("Rust");
        name += " rocks!";
        println!("{}", name);
        target_name = name;
        // println!("{}", name); Value of name has been moved, name cannot be used anymore
    }

    println!("target name is {}", target_name);

    let message = String::from("Hello There!");
    let message_1 = &message;
    let message_2 = &message;
    println!("message = {}, message_1 = {}, message_2 = {}", message, message_1, message_2);

    let mut numbers = String::from("1");
    numbers.push_str(", 2");
    let numbers_borrowed = &mut numbers;
    numbers_borrowed.push_str(", 3");

    let mut alphabet = String::from("a");
    alphabet.push_str(", b");
    let observer = &alphabet;
    println!("Current alphabet string: {}", observer);

    let outer_scope_ref: &String;
    {
        let inner_scop_string = String::from("inner scope");
        outer_scope_ref = &inner_scop_string;
    }

    // println!("Scoped string is {}", outer_scope_ref);
}
