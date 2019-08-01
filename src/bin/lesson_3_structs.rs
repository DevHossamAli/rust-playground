use std::fmt;

#[derive(Debug)]
struct Object {
    width: u32,
    height: u32,
}

// Implementing the Display trait for the Object type by using fmt lib
impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}

// Methods
impl Object {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn show(&self) {
        println!("{}x{} with area: {}", self.width, self.height, self.area());
    }
}

// Related Functions
impl Object {
    fn new(width: u32, height: u32) -> Object {
        Object { width, height }
    }
}

fn main() {
    let o = Object {
        width: 20,
        height: 10,
    };

    let obj = Object::new(30, 40); // Using related function way

    o.show();
    obj.show();

    println!("{:?}", o); // Needs a debug trait
    println!("{:#?}", obj);

    println!("{}", o); // Requires a display trait (manualy)
}
