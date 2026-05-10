#![allow(unused)]
struct Dog {}

impl Animal for Dog {}
impl NotDangerous for Dog {}

struct Cat {}

impl Animal for Cat {}
impl NotDangerous for Cat {}

struct Person<T>
where
    T: Animal + NotDangerous,
{
    firstname: String,
    pet: T,
}

trait Animal {
    fn make_sound(&self, sound: &str) -> String {
        println!("{sound}");
        format!("{}", sound)
    }
}

trait NotDangerous {}

pub fn create_person() {
    let pet1 = Dog {};
    let pets: Vec<Box<dyn Animal>> = vec![Box::from(Dog{}), Box::from(Cat{})];
    pet1.make_sound("bark");
    let person_1 = Person {
        firstname: "luke".into(),
        pet: pet1,
    };
}
