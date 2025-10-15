// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:


// * Use a match expression while iterating the vector to print the ticket info



// * Use an enum for the tickets with data associated with each variant
#[derive(Debug)]
enum Tickets {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64, String)
}




fn main() {
    // * Create one of each ticket and place into a vector
    let concert = vec![Tickets::Backstage(100.0,"Mike".to_owned()), Tickets::Vip(50.00,"Murphy".to_owned()), Tickets::Standard(25.00, "Max".to_owned())];
    for tickets in concert {
        match tickets {
            Tickets::Backstage(price, holder) => println!("Holder {:?} price: {:?}", price, holder),
            Tickets::Vip(price, holder) => println!("Holder {:?} price: {:?}", price, holder),
            Tickets::Standard(price, holder) => println!("Holder {:?} price: {:?}", price, holder),
        }
        }
    }


