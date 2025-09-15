// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal
fn first_name() -> &'static str {
    "Mitchell"
}

fn last_name() -> &'static str {
    "Brown"
}

fn main() {
    println!("Hello {} {}!", first_name(), last_name());
}
