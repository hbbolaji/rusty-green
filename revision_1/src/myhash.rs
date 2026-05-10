use std::collections::{HashMap, HashSet};

pub fn test_hashmap_basic() {
    let mut stock_list: HashMap<String, f32> = HashMap::new();
    println!("{}", stock_list.len());
    println!(
        "capacity: {}, isEmpty: {}",
        stock_list.capacity(),
        stock_list.is_empty()
    );

    stock_list.insert("NVDA".into(), 1200.25);
    stock_list.insert("AAPL".into(), 1900.0);
    stock_list.insert("TSLA".into(), 23.09);
    stock_list.insert("AAPL".into(), 90.78);

    println!("{:?}", stock_list);
    stock_list.remove("AAPL");
    println!("{:?}", stock_list);
    stock_list.entry("META".to_string()).or_insert(67.90);
    println!("{:?}", stock_list);

    for (ticker, current_price) in &stock_list {
        println!("{} is trading at {}", ticker, current_price);
    }

    println!("{:?}", stock_list);
}

pub fn test_hashset_type() {
  let mut planet_list = HashSet::from(["Mercury", "Venus", "Earth"]);
  for planet in &planet_list {
    println!("Thanks for adding {}", planet);
  }

  let planet_list_more = HashSet::from(["Earth", "Mars", "Jupiter"]);
  let planet_diff = planet_list.difference(&planet_list_more);
  for planet in planet_diff {
    println!("Diff: Thanks for adding {}", planet);
  }

  let planet_sym_diff = planet_list.symmetric_difference(&planet_list_more);
  for planet in planet_sym_diff {
    println!("sym diff: Thanks for adding {}", planet);
  }

  planet_list.insert("Saturn");
  planet_list.insert("Uranus");
  planet_list.insert("Neptune");
  planet_list.insert("Pluto");
  println!("{:?}", planet_list)
}