mod conditions;
mod ansi_color_codes;
mod essentials;
mod variables;
mod loops;

fn main() {
    conditions::conditions(15);
    conditions::short_condition(12);
    variables::variables();
    loops::for_loops();
    println!("\n");
    loops::loop_loops();
    println!("\n");
    loops::while_loops();
    println!("\n");
}
