pub fn test_match_int() {
    let my_age: u16 = 28;

    match my_age {
        0..20 => println!("less than 20"),
        20..40 => println!("middle age"),
        _ => println!("old as fuck"),
    }
}

pub fn test_match_array() {
    let prices: [u32; 4] = [30000, 50000, 90000, 120000];

    match &prices[0..=1] {
        [30000, 50000] => println!("You have some reasonable priced cars"),
        [50000, 90000] => println!("not too bad"),
        [90001, 120000] => println!("I need to work hard"),
        _ => println!("we don't want"),
    }
}
