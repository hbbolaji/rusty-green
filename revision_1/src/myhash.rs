use std::collections::HashMap;

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
