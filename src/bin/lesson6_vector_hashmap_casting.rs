/////////////////////////////////////////////////////////////////////////
/// Vectors, HashMaps, Casting, If-Let, While-Let, and the Result Enum...
/////////////////////////////////////////////////////////////////////////
use std::collections::HashMap;
use std::fs::File;


#[derive(Debug)]
enum Example { // Option Enum
    Float(f64),
    Int(i32),
    Text(String)
}


fn main() {

    //#########################################################################
    // Simple Vector Definition
    //#########################################################################

    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    for i in &v {
        println!("{}", i);
    }

    v.push(10);

    println!("{:?} {} {}", &v, v.len(), v.capacity());

    // will return option Some(v) in case of value available or None in case there's no value
    println!("{:?}", v.pop());

    //#########################################################################


    //#########################################################################
    // Vector Of Enums
    //#########################################################################

    let r = vec![
        Example::Int(142),
        Example::Float(12.32),
        Example::Text(String::from("String"))
    ];

    println!("{:?}", &r);

    //#########################################################################


    //#########################################################################
    // Hash Map
    //#########################################################################

    let mut hm = HashMap::new();

    hm.insert(String::from("random"), 12);
    hm.insert(String::from("strings"), 49);
    hm.remove(&String::from("strings"));

    for (k, v) in &hm {
        println!("{} {}", k, v);
    }

    match hm.get(&String::from("random")) {
        Some(&n) => println!("{}", n),
        _ => println!("No match")
    }


    //#########################################################################


    //#########################################################################
    // match vs if let
    //#########################################################################

    let s = Some('c');

    match s {
        Some(i) => println!("{}", i),
        None => {}
    }

    if let Some(i) = s {
        println!("{}", i);
    }


    // More complex example

    let mut x = Some(0);

    // The verbos way!
    loop {
        match x {
            Some(i) => if i > 19 {
                println!("Quit");
                x = None;
            } else {
                println!("{}", i);
                x = Some(i + 2);
            },
            None => {
                break;
            },
        }
    }

    let mut y = Some(0);

    // The same example with in a better way using while let
    while let Some(i) = y {
        if i > 19 {
            println!("Quit");
            y = None;
        } else {
            println!("{}", i);
            y = Some(i + 2);
        }
    }

    //##########################################################################


    //#########################################################################
    // Type casting
    //##########################################################################

    let f = 114.4321_f32;
    let i = f as u8; // Explicit casting
    let c = i as char;

    println!("{} {} {}", f, i, c);

    println!("{}", 255 as char);

    //##########################################################################


    //##########################################################################
    // Result Enum - mostly used for error checking
    //##########################################################################

    let f = File::open("test.txt");

    match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        }
    };

    //##########################################################################
}