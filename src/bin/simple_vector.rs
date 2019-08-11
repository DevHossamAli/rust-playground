/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// Simple Vector Example..
/////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    // Another way of defining a vector type
    // let _my_vector: Vec<u32> = Vec::new();

    let mut v = vec![1, 2, 3, 4];
   
    {
        let item = v.last(); // Important notes check line 19
        println!("item: {:?}", item);
    }

    v.push(5);
    v.remove(1);

    // *** Important - read the following four lines... *******************************************************************************
    // You'd be accessing uninitialized memory but the memory of the vector is reallocated after using push fn on that vector
    // So the item is pointing to no were
    // To solve it check line 10,11,12,13 using a block to isolate that memory ref
    // println!("item: {:?}", item); // Compile time error - cannot borrow `v` as mutable because it is also borrowed as immutable
    // ********************************************************************************************************************************


    // printing out the vector items using for loop
    // iter fn maintain the ownership of the vector
    for n in v.iter() {
        println!("{}", n);
    }
}