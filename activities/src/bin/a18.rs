// Topic: Result
//
// Requirements:





//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message



//   * Implement Debug print functionality using `derive`
// * Create an structure named `Adult` that represents a person aged 21 or older:
#[derive(Debug)]
struct Adult {
    //   * The structure must contain the person's name and age
  age:u8, 
  name:String,
}

// * Implement a `new` function for the `Adult` structure that returns a Result:
impl Adult {
fn new( age: u8, name: &str) -> Result<Self, &str> {
    //   * The Ok variant should contain the initialized structure, but only
    //     if the person is aged 21 or older
    if age >= 21 {
        Ok(Self {
            age, 
            name: name.to_string(),
        })
    } else {
        Err("Age must be at least 21")
    }
}
}


fn main() {
    let child = Adult::new(15, "Mike");
    let adult = Adult::new(21, "Sam");
    match child {
        Ok(child) => println!("{} is {} years old", child.name, child.age),
        Err(e) => println!("{e}"),
    }
     match adult {
        Ok(adult) => println!("{} is {} years old", adult.name, adult.age),
        Err(e) => println!("{e}"),
    }
}

