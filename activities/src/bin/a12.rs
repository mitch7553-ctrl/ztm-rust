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
    dimensions: Dimensions,
}

struct Dimensions {
    width: f64,
    height: i32,
    depth: i32,
}


impl Dimensions {
fn print(&self) {
    println!("width: {:?}", self.width);
    println!("height: {:?}", self.height);
    println!("depth: {:?}",self.depth);
}
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
            Color::Red => println!("red"),
            Color::Blue => println!("blue"),
            Color::Brown => println!("brown")
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
    self.dimensions.print();
    println!("weight: {:?}", self.weight);
}


}



fn main() {
    let small_dimensions = Dimensions {
        width: 1.0,
        height: 1,
        depth: 9,
    };
    let small_box = ShippingBox::newBox(5.0,Color::Brown, small_dimensions);
    small_box.print_box_type();
}

