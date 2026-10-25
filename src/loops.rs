use crate::ansi_color_codes;



pub fn for_loops() {
    println!("{}A for loop{}", ansi_color_codes::GREEN, ansi_color_codes::RESET);
    for i in 1..=5 {
        print!("{i} ");
    }
    println!();
}

pub fn loop_loops() {
    let mut n = 0;
    println!("{}A Loop using the `loop` keyword {}", ansi_color_codes::MAGENTA, ansi_color_codes::RESET);
    loop {
        print!("{n} • ");
        n+=1;

        if n > 5 { // This makes this loop inclusive. n >= 5 would make it exclusive.
            print!("END");
            println!();
            break;
        }
    }
}

pub fn while_loops() {
    let mut n = 0;
    println!("{}A while loop {}", ansi_color_codes::CYAN, ansi_color_codes::RESET);
    while n > -20 {
        print!("{n} • ");
        n -= 1;
    }
    println!();
}