use std::env;

#[derive(Debug)]
struct Dog {
  name: String,
  year: u16
}

impl Dog {
  fn new(name: String, year: u16) -> Self {
    Self { name, year }
  }
}

pub fn test_args() {
  let mut my_args = env::args().collect::<Vec<String>>();
  
  if my_args.len() != 3 {
    println!("Hey you didn't have specify two arguments");
    return;
  }

  let name: String  = String::from(&my_args[1]);
  let year = my_args[2].parse::<u16>().ok().unwrap();

  let dog_1 = Dog::new(name, year);

  // println!("{:?}", dog_1)
}