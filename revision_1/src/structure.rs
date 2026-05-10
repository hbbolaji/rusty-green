#![allow(unused)]
use std::cell::Cell;

#[derive(Debug)]
enum VehicleColor {
    Silver,
    Black,
    White,
    Red,
    Blue,
}

impl Copy for VehicleColor {}
impl Clone for VehicleColor {
    fn clone(&self) -> Self {
        *self
    }
}
struct Person<'p> {
    active: bool,
    username: Cell<&'p str>,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Vehicle {
    manufacturer: String,
    model: String,
    year: u16,
    color: Cell<VehicleColor>,
}

impl Vehicle {
    fn new(manufacturer: String, model: String, year: u16, color: Cell<VehicleColor>) -> Self {
        Self {
            manufacturer,
            model: model,
            year: year,
            color: color,
        }
    }
    fn paint(&self, color: VehicleColor) {
        self.color.set(color);
    }
}

#[derive(Debug)]
struct Color(u8, u8, u8);

fn new_person() -> Person<'static> {
    let p1 = Person {
        active: true,
        username: Cell::from("bolaji"),
        email: "bolaj@gmail.com".into(),
        sign_in_count: 6,
    };
    return p1;
}

fn new_vehicle() -> Vehicle {
    // let v1 = Vehicle {
    //     manufacturer: "Hyundai".into(),
    //     model: "Sonata".into(),
    //     year: 2015,
    //     color: Cell::from(VehicleColor::Black),
    // };
    let v1 = Vehicle::new(
        "Lexus".to_string(),
        "Rx 350h".into(),
        2026,
        Cell::from(VehicleColor::Silver),
    );
    v1.paint(VehicleColor::Blue);
    return v1;
}

pub fn create_vehicle() {
    let my_vehicle = new_vehicle();
    println!("{:?}", my_vehicle)
}

pub fn test_create_user() {
    let my_person: Person<'_> = new_person();
    my_person.username.set("@bolaji");
    println!(
        "{} username: {} has an email of {} and has sign in {} times",
        my_person.active,
        my_person.username.get(),
        my_person.email,
        my_person.sign_in_count
    )
}
