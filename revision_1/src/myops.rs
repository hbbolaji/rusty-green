use std::ops::Add;

struct Person {
    firstname: String,
    lastname: String,
}

impl Add for Person {
    type Output = Marriage;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            husband: self,
            wife: rhs,
            location: "Dubai".to_string(),
        }
    }
}

struct Marriage {
    husband: Person,
    wife: Person,
    location: String,
}

pub fn test_custom_ops() {
    let person_1 = Person {
        firstname: "Donald".to_string(),
        lastname: "Duck".to_string(),
    };
    let person_2 = Person {
        firstname: "Micky".to_string(),
        lastname: "Mouse".to_string(),
    };

    let marraige = person_1 + person_2;
    let husband = format!(
        "{} {}",
        marraige.husband.firstname, marraige.husband.lastname
    );
    let wife = format!("{} {}", marraige.wife.firstname, marraige.wife.lastname);

    println!(
        "{} got married to {} at {}",
        husband, wife, marraige.location
    );
}
