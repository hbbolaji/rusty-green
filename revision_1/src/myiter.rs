#![allow(unused)]

pub fn test_rust_iterators() {
    let fruit_list = vec!["Strawberry", "Blueberry", "Mango", "Orange", "Apple"];

    let nut_list = vec!["Walnut", "Almonds", "Pecans", "Brazil Nuts"];

    let mut fruit_iter = fruit_list.iter();
    let item_1 = fruit_iter.next();
    println!("first item in iterator is {}", item_1.unwrap());

    // for fruit in fruit_iter {
    //   println!("{}", fruit)
    // }

    let food_aggregate = fruit_list.iter().chain(&nut_list);

    let all_foods: Vec<&&str> = food_aggregate.clone().collect();

    for food in food_aggregate {
        println!("Eating {}", food);
    }

    fruit_list
        .iter()
        .map(|f| String::from(*f))
        .map(|mut e| {
            e.push_str(" fruit");
            return e;
        })
        .for_each(|e| println!("{}", e));

    let first_names = vec!["Hashim", "Zaid", "Ibrahim", "Muhsin"];
    let first_name_strings = first_names.iter().map(|e| String::from(*e));

    let last_names = vec!["Bello", "Almatari", "Halim", "Ahmad"];
    let last_name_strings = last_names.iter().map(|e| String::from(*e));

    let full_names = first_name_strings.zip(last_name_strings);
    full_names.clone().for_each(|e| println!("{} {}", e.0, e.1));

    for (index, value) in full_names.enumerate() {
        println!("Index: {}, value: {}, {}", index, value.0, value.1)
    }

    let foods = vec![("Potatoes", 21), ("Yam", 3), ("Rice", 6)];
    let food_quantity = foods.iter().fold(0u32, |acc, e| acc + e.1);
    println!("The total food quantity is {}", food_quantity);

    // iter methods
    // skip(num_to_skip)
    // take(num_to_take)
    // fold() -> similar to js reduce, initial
    // peakable().peek()
}
