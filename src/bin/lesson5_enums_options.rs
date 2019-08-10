#![allow(dead_code)]

//////////////////////////////////////////////////////////////////////
/// Enums Example 1
//////////////////////////////////////////////////////////////////////
/// 
#[derive(Debug)]
enum Direction {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}

#[derive(Debug)]
enum Keys {
    UpKey(String),
    DownKey(String),
    LeftKey(String),
    RightKey(String),
}

impl Direction {
    fn match_direction(&self) -> Keys {
        match *self {
            Direction::Up(_) => Keys::UpKey(String::from("Pressed w")),
            Direction::Down(_) => Keys::DownKey(String::from("Pressed s")),
            Direction::Left(_) => Keys::LeftKey(String::from("Pressed a")),
            Direction::Right(_) => Keys::RightKey(String::from("Pressed d")),
        }
    }
}

impl Keys {
    fn destruct(&self) -> &String {
        match *self {
            Keys::UpKey(ref s) => s,
            Keys::DownKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
            Keys::RightKey(ref s) => s,
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

//////////////////////////////////////////////////////////////////////
/// Enums Example 2
//////////////////////////////////////////////////////////////////////
enum Shape {
    Rectangle {width: u32, height: u32},
    Square(u32),
    Circle(f64),
}

// This is a kind of polymorphism we're using one method fro calculating
// the area for three different shapes
impl Shape {
    fn area(&self) -> f64 {
        match *self {
            Shape::Rectangle {width, height} => (width * height) as f64,
            Shape::Square(ref s) => (s * s) as f64,
            Shape::Circle(ref r) => 3.14 * (r * r),
        }
    }
}

//////////////////////////////////////////////////////////////////////
/// Options Example 3
//////////////////////////////////////////////////////////////////////
// enum Option<T> {
//     Some(T),
//     None,
// }

// std::option::Option
fn division(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}


fn main() {
    // Example 1
    // Instance of Direction Up type
    let u = Direction::Up(Point {x: 0, y: 1});
    let k = u.match_direction();
    let x = k.destruct();

    println!("{:?}", k);
    println!("{:?}", x);


    // Example 2
    let r = Shape::Rectangle{width: 10, height: 70};
    let s = Shape::Square(10);
    let c = Shape::Circle(4.5);

    let ar = r.area();
    println!("Area of Rectangle: {}", ar);

    let aq = s.area();
    println!("Area of Square: {}", aq);

    let ac = c.area();
    println!("Area of Circle: {}", ac);



    // Example 3
    let res = division(5.0, 7.0);
    match res {
        Some(x) =>println!("{:.7}", x),
        None => println!("Cannot divide by 0")
    }



    // How to manipulate references
    let u = 10;
    let v = &u;
    let ref z = u; // ANother way to create a reference

    if z == v {
        println!("They are equal!");
    }
}