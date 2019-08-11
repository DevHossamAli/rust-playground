/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Simple Reference Example..
/////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {

    let mut x = 5;

    //{ // Adding a scope in this case dosn't make any difference!
    // Create a reference of x
    let y = &mut x; // You cannot borrow x as mutable -> x should be mutable
    *y += 1; // Should use * to access the reference value
    println!("y = {}", y); // should print 6
    //}

    // Here, x goes out of scope. But because y does not have ownership of what it refers to, nothing happens
    // x should be changed through *y and should be 6 as well
    println!("x = {}", x);

}