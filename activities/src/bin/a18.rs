// Topic: Result
//
// Requirements:




//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
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
    if age 
}
}


fn main() {}

