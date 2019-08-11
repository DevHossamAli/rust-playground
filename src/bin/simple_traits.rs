/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Simple Traits Example..
/////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct Person {
    name: String,
    age: u8
}

impl ToString for Person { // Trait takes on only one function
    fn to_string(&self) -> String {
        return format!("My name is {} and I am {} years old!", self.name, self.age);
    }
}

fn main() {
    let p = Person { name: String::from("John"), age: 27 };
    
    println!("{}", p.to_string());
}