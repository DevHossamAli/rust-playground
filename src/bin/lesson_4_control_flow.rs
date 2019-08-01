fn main() {

    //#########################################################################
    // Conditional Statements - nested if else
    //#########################################################################

    let n = 2;

    if n % 4 == 0 {
        println!("n is divisible by 4");
    } else if n % 3 == 0 {
        println!("n is divisible by 3");
    } else if n % 2 == 0 {
        println!("n is divisible by 2");
    } else {
        println!("n is not divisible by 4, 3 or 2");
    }

    //#########################################################################


    //#########################################################################
    // Using if statements in bindings
    //#########################################################################

    let x = true;

    // types within the block shoud be the same
    let num = if x {
        50
    } else {
        0
    };

    println!("num: {}", num);

    //#########################################################################


    //#########################################################################
    // The match statement == switch
    //#########################################################################

    let x = 5;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("default"),
    }

    let z = 15;
    // More complex match statment with ranges
    match z {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13...19 => println!("A teen"), // this range includes 19
        _ => println!("adult!"),
    }

    let t = (0, 5);
    // More complex match with tuples
    match t {
        (0, y) => println!("y: {}", y), // This matches the tuple pattern by destructuring
        (x, 0) => println!("x: {}", x),
        _ => println!("No match"),
    }

    let pair = (5, 5);
    // More complex match with tuples & guards
    match t {
        (x, y) if x == y => println!("Equal"),
        (x, y) if x + y == 0 => println!("Equal Zero"),
        (x, _) if x % 2 == 0 => println!("X is even"),
        _ => println!("No match"),
    }

    let p = 5;
    // More complex match - bind based on match in range
    match p {
        n @ 1...12 => println!("Range1 contains {}", n),
        n @ 13...19 => println!("Range2 contains {}", n),
        _ => println!("No match"),
    }

    let p = 5;
    // More complex match - bind based on match in range
    // returns the match back to n
    let n = match p {
        n@ 1...12 => n,
        n@ 13...19 => n,
        _ => 0
    };

    println!("n: {}", n);

    //#########################################################################


    //#########################################################################
    // Loops & Iterations
    //#########################################################################

    // Infinite loop!
    // loop {
    //     println!("Infinite Loop!!");
    // }

    let mut c = 0;

    // Conditional loop
    loop { // Will print from 0 to 9
        println!("{}", c);
        c += 1;
        if c >= 10 {
            break;
        }
    }

    //#########################################################################


    //#########################################################################
    // Labeling nested loops
    //#########################################################################

    // 'a:loop {
    //     println!("Loop a");
    //     'b:loop {
    //         println!("Loop b");
    //         'c:loop {
    //             println!("Loop c");
    //             break 'b;

    //             if true {
    //                 continue;
    //             }else {
    //                 break;
    //             }
    //         }
    //     }
    // }

    //#########################################################################


    //#########################################################################
    // Using loop statements as bindings
    //#########################################################################

    let y = loop {
        break 10; // Will bind 10 to y
    };

    println!("y: {}", y);

    //#########################################################################


    //#########################################################################
    // The while loop
    //#########################################################################

    let mut b = 10;

    while b != 0 {
        println!("{}!", b);
        b = b - 1;
    }

    //#########################################################################


    //#########################################################################
    // The Foor loop
    //#########################################################################

    // Looping over vector type
    let v = vec![10, 20, 30, 40, 50];

    for i in v {
        println!("i: {}", i);
    }

    // Looping over range (exclusive range)
    for i in 1..101 { // Won't include 101
        println!("i: {}", i);
    }

    //#########################################################################

}