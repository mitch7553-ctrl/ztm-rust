// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:




fn main() {
    // * Use a mutable integer variable
    let mut y = 5;
    // * Use a while statement
    while y >= 1 {
        
    // * Print the variable within the while loop
        println!("{:?}", y);
        y = y - 1; 
    }
    println!("Done!");
}

