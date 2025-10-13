// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:

struct Numbers {
    values: i32,
}

fn main() {
    // * Use a vector to store 4 numbers
    let my_values = vec![
        Numbers { values: 10 },
        Numbers { values: 20 },
        Numbers { values: 30 },
        Numbers { values: 40 },
    ];
    // * Iterate through the vector using a for..in loop
    for num in &my_values {
        // * Determine whether to print the number or print "thirty" inside the loop
        if num.values == 30 {
            println!("thirty");
        }
    }
    // * Use the .len() function to print the number of elements in a vector
    println!("{}", my_values.len());
}
