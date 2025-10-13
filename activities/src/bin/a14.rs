// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:

// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed

// * Use a struct for a persons age, name, and favorite color
struct Person {
    age: i32,
    // * The color and name should be stored as a String
    name: String,
    favorite_color: String,
}

// * The name and colors should be printed using a function
fn print_info(name: &str, color: &str) {
    println!("Name: {:?}, Favorite Color: {:?}", name, color);
}

fn main() {
    // * Create and store at least 3 people in a vector
    let people = vec![
        Person {
            age: 19,
            name: String::from("John"),
            favorite_color: String::from("Blue"),
        },
        Person {
            age: 27,
            name: String::from("Mike"),
            favorite_color: String::from("Red"),
        },
        Person {
            age: 30,
            name: String::from("Steve"),
            favorite_color: String::from("Green"),
        },
    ];

    for person in people {
        if person.age < 25 {
            print_info(&person.name, &person.favorite_color);
        } else {
            println!(
                "{:?} is over 25 years old. He/she is {:?}.",
                person.name, person.age
            );
        }
    }
}
