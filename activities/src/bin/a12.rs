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
// * Use an enum for the box color
enum Color {
    Red,
    Blue,
    Brown,
}
// * Implement functionality on the box struct to create a new box
impl ShippingBox {
fn newBox() -> Self {
    color:Color::Red,
    weight:25.5

}

// * Implement functionality on the box struct to print the characteristics
fn print_box_type(&self) {
    println!("Box color: {:?} , box weight: {:?}",self.color, self.weight )
}
}



fn main() {
    let new = ShippingBox::newBox();
    new.print_box_type();
}

