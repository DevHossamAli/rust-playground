use std::mem;

fn main() {
    // Common premitive types in Rust:
    // Integer: i8, u8, i16, u16, i32, u32, i64, u64, isize, usize
    // Float: f32, f64
    // Note. isize (size number) takes the maximum machine based architecture size.

    // By default variables are immutable in Rust
    let x = 5;
    println!("{}", x);
    // But also could defined as mutable
    let mut y = 7;
    println!("{}", y);
    y = 10;
    println!("{}", y); // Should be 10

    // Note. Rust follows by default type inference concept.
    // Note. _ before varaible indicates that it might be unused.
    // In Rust variables definition should follow the snake case name ie. my_var.

    // Mathematical operations
    let _a = 1 + 1;
    let _b = 5 - 3;
    let _c = 4 * 4;
    let _d = 8 / 4;
    let _e = 51 % 2;

    // bool type
    // With type definition
    let _is_small: bool = false;
    // char type
    let _c: char = 'x';
    // Tuple type
    let _my_tuple: (i32, f64, char) = (8, 7.14, 'z');
    // Tuple destructuring
    let (_i, _, ch) = _my_tuple;
    println!("{}", ch);
    // Array type
    let my_arr = [1, 2, 3, 4, 5];
    // Accessing array element by index
    let a1 = my_arr[0];
    println!("{}", a1);

    // Empty Tuple
    let _empty_tuple = ();
    // Note. any function that doesn't return anything actually returns empty tuple!
    // Nesting Tuples
    let t = (2, 's', true);
    let t2 = (7, 'e', (1, 'a', false));
    // {:?} tells Rustc to get the tuple value using debug flag
    // tuple.{index} to access the tuple values
    println!("{} {:?}", t.0, t2);
    // Debug flag with prettier flag {:#?}
    println!("{} {:#?}", t.0, t2);

    // Array definition with type signature of i32 and length of 5
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{} {} {}", arr[1], arr.len(), mem::size_of_val(&arr));
    // Slicing the array
    let slice = &arr[2..4];
    println!("Parent Array: {:?}", arr);
    println!("Sliced Array: {:?}", slice);

    // String types
    let s1 = "String"; // Slice of string (as character sequence)
    let str1 = s1.to_string(); // Actual String
    let s2 = String::from("myString"); // Convert to actual string using from fn
    println!("{} {} {}", s1, str1, s2);
    // Take a slice form string literals
    let str_slice = &s1[1..3];
    println!("{}", str_slice);
    // String concatenation
    let h = String::from("Hello ");
    let w = String::from("World!");
    let hw = h + &w;
    println!("{}", hw);
}
