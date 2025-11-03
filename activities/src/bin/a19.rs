use std::collections::HashMap;
// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:

fn main() {
    // * Use a HashMap for the furniture store stock
    let mut furniture_store: HashMap<&str, i32> = HashMap::new();
    furniture_store.insert("Chairs", 5);
    furniture_store.insert("Beds", 3);
    furniture_store.insert("Tables", 2);
    furniture_store.insert("Couches", 0);

    let mut total_item = 0;
    for (furniture, &count) in furniture_store.iter() {
        if count == 0 {
            println!("{}: out of stock", furniture);
        } else {
            println!("{}: {}", furniture, count);
            total_item += count;
        }
        println!("Total items in stock: {}", total_item);
    }
}
