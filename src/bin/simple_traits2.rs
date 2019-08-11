/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Simple Traits Example2 - Defining Your Own Traits..
/////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct Animal {
    name: String,
    kind: String
}

// Could be treated as interface - Not really!!
trait Swim {
    fn can_swim(&self) -> bool;
}

impl Swim for Animal {

    fn can_swim(&self) -> bool {
        if self.kind == "fish" {
            return true;
        }
        return false;
    }

}

fn main() {
    let a = Animal {
        name: String::from("bubbles"),
        kind: String::from("fish")
    };

    println!("Can {} swim? {}", a.name, a.can_swim());
}