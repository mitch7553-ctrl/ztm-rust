// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:




// * Use an enum with color names as variants
enum Colors {
    Red,
    Blue,
    Green
}
// * Use a function to print the color name
// * The function must use the enum as a parameter
fn print_colors(which_color: Colors){
// * Use a match expression to determine which color
match which_color {
    Colors::Red => println!("The color is red."),
    Colors::Blue => println!("The color is blue."),
    Colors::Green => println!("The color is green."),
}
}



fn main() {
    //name to print

    print_colors(Colors::Red);
    print_colors(Colors::Blue);
    print_colors(Colors::Green);



}

