// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics







// * Use a struct to encapsulate the box characteristics
struct ShippingBox {
    color: Color,
    weight: f64,
}

struct Dimensions {
    width: f64,
    height: i32,
    depth: i32,
}


impl Dimensions {

}
// * Use an enum for the box color
enum Color {
    Red,
    Blue,
    Brown,
}

impl Color {
    fn print(&self) {
         match self {
            Color::Red println!("red"),
            Color::Blue println!("blue"),
            Color::Brown println!("brown")
         }
    }
}
// * Implement functionality on the box struct to create a new box
impl ShippingBox {
fn newBox(weight: f64, color: Color, dimensions: Dimensions) -> Self {
   Self {
    weight,
    color,
    dimensions,
   }
}

// * Implement functionality on the box struct to print the characteristics
fn print_box_type(&self) {
    self.color.print();
}



fn main() {
    let new = ShippingBox::newBox();
    new.print_box_type();
}

