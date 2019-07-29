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
    let _is_small: bool = false; // With defined type
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
}
