/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Simple Passing By Reference & Ownership Example..
/////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct Person {
    name: String,
}

//** greet(p: Person) If we didn't use the reference instead which means we're moving the ownership to the greet fn */
fn greet(p: &Person) {
    println!("Hello {}!", p.name);
}

fn main() {
    let p = Person { name: String::from("John") };

    greet(&p);
   
    // Will break line 21 because we already moved the ownership to greet fn
    //greet(p);

    // will break
    //greet(p); // value used here after move
}