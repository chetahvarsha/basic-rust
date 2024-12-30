mod area; // Include the module

use area::{Shape, Rectangle}; // Bring the necessary i

fn main() {
    let circle = Shape::Circle { radius: 10.0 };
    let triangle = Shape::Triangle { width: 10.0, height: 5.0 };
    let rectangle = Rectangle::new(10, 5);

    println!("Circle area: {}", circle.area());
    println!("Triangle area: {}", triangle.area());
    println!("Rectangle area: {}", rectangle.area());
    println!("Rectangle perimeter: {}", rectangle.perimeter());
}
