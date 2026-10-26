//? Im obsessed with readable codes
mod       conditions;
mod ansi_color_codes;
mod       essentials;
mod        variables;
mod            loops;
mod        functions;


struct Player {
    name: String,
    health: i32,
    damage: i32,
}
// I need to use the keyword `pub` here aswell. Both for the Struct and the variables it contains.

impl Player {
    fn take_damage(&mut self, amount: i32) {
        self.health -= amount;
        if !self.is_alive() {
            println!("you died");
        }
        println!("health: {}", self.health);
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn print_stats(&self) {
        println!("Subject Name: {0}\nSubject Health: {1}\nSubject Damage: {2}", self.name, self.health, self.damage);
    }
}

// impl -> implement functions inside the struct.
//? `&self` -> borrows the instance immutably. Its values can only be read.
//? `&mut self` -> borrows the instance mutably. Its values can also be changed.



fn main() {
    // conditions::conditions(15);
    // conditions::short_condition(12);
    // variables::variables();
    // loops::for_loops();
    // println!("\n");
    // loops::loop_loops();
    // println!("\n");
    // loops::while_loops();


    // let result: i32 = functions::add(8, 8); // 16
    // let result_deduct: i32 = functions::deduct(8, 8); // 0
    // println!("8 + 8 = {result}\n8 - 8 = {result_deduct}");

    let mut player = Player {
        name: String::from("player_001"),
        health: 100,
        damage: 25,
    };

    let mut player2 = Player {
        name: String::from("player_002"),
        health: 85,
        damage: 35
    };

    player.print_stats();
    player.take_damage(20);
    player.take_damage(20);
    player.take_damage(20);
    player.take_damage(20);
    println!("{}", player.is_alive());
    player.take_damage(20);
    println!("{}", player.is_alive());
    player.take_damage(20);
}
