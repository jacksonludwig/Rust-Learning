use std::collections::HashMap;

fn main() {
    let mut x: Vec<i32> = vec![1, 2, 3, 4];
    x.push(5);
    println!("{:?} -- {} -- {}", &x, x.len(), x.capacity());

    let r = vec![
        Example::Float(143.2),
        Example::Int(9348),
        Example::Text(String::from("Hey")),
    ];
    println!("{:?}", &r);

    let mut map = HashMap::new();
    map.insert(String::from("random"), 24);
    map.insert(String::from("strings"), 50);

    for (k, v) in &map {
        println!("{}: {}", k, v);
    }

    match map.get(&String::from("random")) {
        Some(&n) => println!("{}", n),
        _ => println!("no match"),
    }

    let s = Some('c');
    match s {
        Some(i) => println!("{}", i),
        _ => {}
    }
    if let Some(i) = s {
        println!("{}", i);
    }

    let mut s2 = Some(2);
    while let Some(i) = s2 {
        if i > 19 {
            println!("Quit");
            s2 = None;
        } else {
            println!("{}", i);
            s2 = Some(i + 2);
        }
    }

    use std::fs::File;
    let f = File::open("test.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("File not found: {:?}", error),
    };
}

#[derive(Debug)]
enum Example {
    Float(f64),
    Int(i32),
    Text(String),
}
