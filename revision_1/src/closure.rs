struct Person {
  firstname: String,
  lastname: String
}

pub fn test_closures() {
  let add = |x: i16, y: i16| {
    println!("x = {}, y = {}", x, y);
    x + y
  };
  let result = add(5, 8);

  let print_result = move || println!("The result is {}", result);
  print_result();
  println!("is result = {} still valid", result);

  let mut p1 = Person { firstname: "hashim".into(), lastname: "bello".into()};
  let mut change_name = || p1.lastname = "bunyamin".into();
  change_name();
}