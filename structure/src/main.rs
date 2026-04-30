struct Creature {
    name: String,
    health: i32,
    max_health: i32,
    dmg: i32
}

impl Creature {
    fn print_status(&self) {
        if self.health <= 0 {
            println!("{} is dead", self.name)
        } else {
            println!("{} has {} / {} health and deals {} damages", self.name, self.health, self.max_health, self.dmg)
        }
    }

    fn fight_creature(&mut self, opp: &mut Creature) {
        self.health -= opp.dmg;
        opp.health -= self.dmg;
    }
}

fn main() {
    let mut minator = Creature {
        name: "Minator".into(),
        health: 8,
        max_health: 8,
        dmg: 3
    };

    let mut goblin = Creature {
        name: "Goblin".into(),
        health: 2,
        max_health: 2,
        dmg: 5
    };

    minator.print_status();
    goblin.print_status();

    println!("{} is fighting {}...", minator.name, goblin.name);
    minator.fight_creature(&mut goblin);

    minator.print_status();
    goblin.print_status(); 
}

