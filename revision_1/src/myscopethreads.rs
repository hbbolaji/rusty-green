use std::thread;

struct Person {
    firstname: String,
}

pub fn test_thread_variables() {
    let person_1 = Person {
        firstname: "Hashim ".to_string(),
    };
    let age = 34;

    let print_age = || {
        println!("This is the child closure ");
        println!("Your age is {}", age);
        println!("Your name is {}", &person_1.firstname);
    };

    // thread::spawn(print_age).join();
    thread::scope(|scope| {
        scope.spawn(print_age);
    });

    println!("Your age is {}", age);
    println!("Your name is {}", person_1.firstname);
    println!("Finish printing age...");
}
