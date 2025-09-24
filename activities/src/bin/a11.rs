// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter





// * Use a struct for the grocery item
struct Grocery {
    // * Use two i32 fields for the quantity and id number
    id: i32,
    quantity: i32,
}

// * Create a function to display the quantity, with the struct as a parameter
fn display_quanity(grocery: &Grocery) {
    println!("quantity: {:?}", grocery.quantity);
}
// * Create a function to display the id number, with the struct as a parameter
fn display_id(grocery: &Grocery) {
    println!("ID numebr: {:?}", grocery.id);
}



fn main() {
    let groceries = Grocery {
        id: 999,
        quantity: 44
    };

    display_id(&groceries);
    display_quanity(&groceries);
}

