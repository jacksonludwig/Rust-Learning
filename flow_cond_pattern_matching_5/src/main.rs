fn main() {
    let n = true;

    let b = if n { 50 } else { 76 };

    println!("{}", b);

    let x = loop {
        break 10;
    };

    for i in 1..=101 {
        println!("test inclusive");
    }

    let d = 5;
    match d {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("who knows"),
    }

    let e = 15;
    match e {
        1 => println!("one"),
        2 | 3 | 5 | 7 | 11 => println!("This is prime"),
        13..=19 => println!("13 to 19"),
        _ => println!("unaware"),
    }

    let pair = (0, -2);
    match pair {
        (0, _) => println!("x is 0"),
        (_, 0) => println!("y is 0"),
        _ => println!("no zeroes"),
    }

    let pair2 = (5, -5);
    match pair2 {
        (x, y) if x == y => println!("Equal"),
        (x, y) if x + y == 0 => println!("Sum to zero"),
        (x, _) if x % 2 == 0 => println!("first value is even"),
        _ => println!("no match"),
    }

    let p = 5;
    match p {
        n @ 1..=12 => println!("n: {}", n),
        n @ 13..=22 => println!("n: {}", n),
        _ => println!("no match"),
    }

    let n = match p {
        n @ 1..=12 => n,
        n @ 13..=19 => n,
        _ => 0,
    };

    loop {
        'a: loop {
            println!("loop a");
            'b: loop {
                println!("loop b");
                break 'a;
            }
        }
    }
}
