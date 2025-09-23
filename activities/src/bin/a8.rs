// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor



// * Use an enum to create different flavors of drinks
enum Drinks {
    Grape,
    Lemon,
    Strawberry
}
// * Use a struct to store drink flavor and fluid ounce information
struct DrinkInfo {
    flavor: Drinks,
    ounce: i32,

}
// * Use a function to print out the drink flavor and ounces
fn print_drinks(drink: DrinkInfo) {
    match drink.flavor {
        Drinks::Grape => println!("Flavor: Grape"),
        Drinks::Lemon => println!("Flavor: Lemon"),
        Drinks::Strawberry => println!("Flavor: Strawberry"),
    }
    println!("Ounces: {}", drink.ounce);
}




fn main() {
   let drink_selection = DrinkInfo {
    flavor: Drinks::Grape,
    ounce: 64,
   };

 print_drinks(drink_selection);

}

