pub fn test_match_int () {
  let my_age: u16 = 28;

  match my_age {
      0..20 => println!("less than 20"),
      20..40 => println!("middle age"),
      _ => println!("old as fuck")
  }
}