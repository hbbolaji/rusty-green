#[derive(Debug)]
struct LastName(String);

impl Default for LastName {
  fn default() -> Self {
      Self("Bello".into())
  }
}

#[derive(Debug)]
struct Person {
  firstname: String,
  lastname: LastName,
  age: u8,
  location: String
}

impl Default for Person {
  fn default() -> Self {
      Self {
        firstname: "Hashim".to_string(),
        lastname: LastName::default(),
        age: 28,
        location: "Lagos, Nigeria".to_string()
      }
  }
}

pub fn test_default_impl() {
  let mut person_1 = Person::default();
  let mut person_2 = Person {
    firstname: "Ibrahim".into(),
    age: 25,
    ..Person::default()
  };

  // let mut person_3 = 
  person_1.location = "London, UK".into();
  person_2.location = "Zanzibar, TZ".into();
  println!("{:?}", person_1);
  println!("{:?}", person_2);
}