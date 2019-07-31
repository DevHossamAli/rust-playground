fn main() {

    //#########################################################################
    // Simple Ownership With Literal Data (Stack)
    //#########################################################################

    // Simple ownership example x owns 1
    // x value will be stored on the stack because it's literal
    let x = 1;
    // Ownership moved from x to y because it's only one ownership
    // at a time for a single peace of data
    let y = x;

    //#########################################################################


    //#########################################################################
    // How Scopes Works In Rust
    //#########################################################################

    // Defining a scope in Rust
    {
        let a = 5;
        // Rust supports nested scopes
        {
            let b = x + a + 10;
            println!("{}", b);
        }
    }

    // Exception: because a and x are not in the same scope
    // x + a; // -> Throws exception

    //#########################################################################


    //#########################################################################
    // Ownership With More Complex Data Stored In The Heap 
    //#########################################################################

    // Ownership works on a different way with heap
    let s = String::from("String");
    // We're moving the reference from s to s2 (Moving ownership)
    // with no way back to s which means that s will disappear completely 
    // from the scope Only one reference can own a peace of data at a time
    //let s2 = s; // Will be re-defined in line 43 using borrowing
    // We're trying to use moved value s which will throgh an exception
    // println!("{}", s); // -> Throws exception
    // To fix this we need to use what's called borrowing
    let s2 = &s; // Which means s2 equals a reference for s `&s`
    // Will work because now s2 borrows a copy from s using it's reference 
    println!("{}", s);

    //#########################################################################


    //#########################################################################
    // Ownership Using Functions (Moving ownership from function to another)
    //#########################################################################

    // Defining a vector of a dynamic size in the heap
    let mut v = Vec::new();
    // Filling the vector with numbers from 1 to 1000
    for i in 1..1000 {
        v.push(i);
    }

    // Here we're the ownership is taken from the main fn to take fn
    // And we never return the ownership back to the main fn which in
    // this case called (Moving the ownership) moving the resurce from
    // one function to another.
    moveOwnership(v);
    // println!("{}", v[0]);
    println!("Finished!");

    //#########################################################################


    //#########################################################################
    // Ownership Using Functions (Copying ownership from function to another)
    //#########################################################################

    let n1 = 30;
    let n2 = 50;

    // They are not being moved but being copied because they exist on the stack
    // not the heap. we could of course do the same with heap but it's more
    // efficient to it with the stack data.
    copyOwnership(n1, n2);

    println!("n1: {} & n2: {} remains under main function custody!", n1, n2);

    //#########################################################################


    //#########################################################################
    // Borrowing Using Functions (Maintain ownership)
    //#########################################################################

    let mut vc = Vec::new(); // Creates a new vector

    for i in 1..1000 {
        vc.push(i);
    }

    // Testing the ownership using borrowing concept..
    vc = returnOwnership(vc); // The ownership still under main function!
    println!("The main function still owns vc: {}, {}", vc[0], vc[1]);

    borrow1(&vc);
    println!("The main function still owns vc: {}, {}", vc[0], vc[1]);

    borrow2(&vc);
    println!("The main function still owns vc: {}, {}", vc[0], vc[1]);

    // Borrowing by loop and function using resource reference - last example
    let vec = vec![4,5,3,4,7,8,6,7,2,1,3,5,4];

    for &j in &vec {
        let r = count(&vec, j);
        println!("{} is repeated {} times", j, r);
    }

    // vec ownership still owned by the main function!
    println!("{}", vec[1]); // Testing main function ownership for vec

    //#########################################################################

}

/**
 * Takes the ownership from the main function and never return it back.
 */
fn moveOwnership(v: Vec<i32>) {
    println!("W took v: {}", v[10] + v[100]);
}

/**
 * Copy a reference value from the main function without changing the
 * ownership state.
 */
fn copyOwnership(x: i32, y: i32) {
    println!("{}", x + y);
}

/**
 * Takes in a vector and returns it back
 */
fn returnOwnership(v: Vec<i32>) -> Vec<i32> {
    println!("{}", v[10] + v[20]);
    v // no return keyword!
}

/**
 * Borrows the resource by passing the reference as argument
 * which mains that the main function still have the ownership
 */
fn borrow1(v: &Vec<i32>) {
     // Using the pointer of the reference which means the location value
    println!("{}", (*v)[10] + (*v)[20]);
}

/**
 * Borrows the resource by passing the reference as argument
 * which mains that the main function still have the ownership
 */
fn borrow2(v: &Vec<i32>) {
    // Rust will follow that reference automatically to the actual data value
    // and won't print a memory address! Cool!!
    println!("{}", v[10] + v[20]);
}

/**
 * Loops for a vector for number occurence count and returns number of occurences
 */
fn count(v: &Vec<i32>, val: i32) -> usize {
    v.into_iter().filter(|&&x| x == val).count()
}