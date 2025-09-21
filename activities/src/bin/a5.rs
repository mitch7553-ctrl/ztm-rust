// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:





fn main() {
    // * Use a mutable integer variable
    let mut i = 1;
    // * Use a loop statement
    loop {
        // * Print the variable within the loop statement
        println!("{:?}", i);
        i = i + 1;
        if i == 5 {
            // * Use break to exit the loop
            break;
        }
    }
    println!("We have reached number 4!")
}

