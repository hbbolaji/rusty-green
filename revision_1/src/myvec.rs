#![allow(unused)]
#[derive(Debug)]
struct Car {
    manufacturer: String,
    model: String,
}

pub fn test_vec_int() {
    let mut my_ints: Vec<i32> = Vec::new();
    my_ints.push(30);
    my_ints.push(60);
    my_ints.push(90);
    println!("{:?}", my_ints);
    println!(
        "size = {:?}, capacity = {:?}",
        my_ints.len(),
        my_ints.capacity()
    );
    println!("The first element is {:?}", &my_ints[0..]);
}

pub fn test_vec_string() {
    let firstnames = vec!["Trevor", "Jon", "Jimmy", "Steven", "James"];
    for firstname in &firstnames {
        println!("Processing {}", firstname)
    }

    println!("names are {:?}", firstnames)
}

pub fn test_vect_car() {
    let mut car_lists = Vec::with_capacity(10);
    car_lists.push(Car {
        manufacturer: "Porshe".to_string(),
        model: "992".into(),
    });
    car_lists.push(Car {
        manufacturer: "Mercedes".to_string(),
        model: "Amg gt 63".into(),
    });
    println!("{:?}", car_lists);
    // car_lists.pop();
    car_lists.insert(
        2,
        Car {
            manufacturer: "Proton".into(),
            model: "Saga".into(),
        },
    );
    println!(
        "size = {:?}, capacity = {:?}",
        car_lists.len(),
        car_lists.capacity()
    );
    println!("{:?}", car_lists);
    println!(
        "{:?}",
        car_lists.iter().filter(|c| c.model == "Saga".to_string())
    );
    println!(
        "{:?}",
        car_lists.iter().filter(|c| c.model != "Saga".to_string())
    );
}
