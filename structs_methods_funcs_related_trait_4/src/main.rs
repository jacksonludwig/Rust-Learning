fn main() {
    let shape = Shape {
        width: 35,
        height: 55,
    };
    println!("{}x{} == {}", shape.width, shape.height, shape.area());

    let shape2 = Shape::new(50, 10);
    shape2.show();

    println!("{:?}", shape2);
    println!("{}", shape2);
}

use std::fmt;

#[derive(Debug)]
struct Shape {
    width: u32,
    height: u32,
}

impl Shape {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn show(&self) {
        println!("{}x{} == {}", self.width, self.height, self.area());
    }
}

impl Shape {
    fn new(width: u32, height: u32) -> Shape {
        Shape {
            width: width,
            height: height,
        }
    }
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}

/*
fn area(obj: &Shape) -> u32 {
    obj.width * obj.height
}
*/
