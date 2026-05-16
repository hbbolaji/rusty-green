pub fn test_dyn_traits() {
  let dog_1: &dyn AnimalSound = &Dog{};
  let antelop_1: &dyn AnimalSound = &Antelope {};
  make_some_nose(dog_1);
  make_some_nose(antelop_1);

  let dog_1: &dyn AnimalEating = &Dog{};
  eat_some_food(dog_1);

  let bear_1 = get_animal();
  // bear_1.eat_food();
  eat_some_food(bear_1.as_ref());
}

fn make_some_nose(animal: &dyn AnimalSound){
  animal.make_sound();
}

fn eat_some_food(animal: &dyn AnimalEating) {
  animal.eat_food();
}

fn get_animal() -> Box<dyn Animal> {
  Box::from(Bear{})
}

struct  Dog {}
struct Antelope{}
struct Bear {}

trait AnimalEating {
  fn eat_food(&self) -> ();
}

trait AnimalSound {
  fn make_sound(&self) -> ();
}

trait Animal: AnimalEating + AnimalSound {}

impl AnimalEating for Dog {
  fn eat_food(&self) {
      println!("Dog is eating dog food");
  }
}

impl AnimalSound for Dog {
  fn make_sound(&self) {
    println!("Dog is barking");
  }
}

impl Animal for Dog {}

impl AnimalEating for Antelope {
  fn eat_food(&self) {
    println!("Antelope is eating natural desert plants");
  }
}

impl AnimalSound for Antelope {
  fn make_sound(&self) {
    println!("Antelope is bleating");
  }
}

impl Animal for Antelope{}

impl AnimalEating for Bear {
  fn eat_food(&self) {
      println!("Bear is eating other animals");
  }
}

impl AnimalSound for Bear {
  fn make_sound(&self) {
      println!("Bear is roaring");
  }
}
impl Animal for Bear {}