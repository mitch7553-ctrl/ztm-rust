// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:


// * Use the println macro to display messages to the terminal

fn main() {
// * Use a variable set to any integer value
let x = 10;

// * Use an if..else if..else block to determine which message to display
if x < 5 {
    println!("This is my message!");
}
else if x > 5 {
    println!("This is another message.");
} else {
    println!("This is the else message.");
}
}

