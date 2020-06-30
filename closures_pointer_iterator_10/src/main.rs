enum List {
    Cons(i32, Box<List>),
    End,
}

fn run<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn add3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

struct A<F: Fn(i32) -> i32> {
    f: F,
}

use List::Cons;
use List::End;
fn main() {
    let b = Box::new(10);
    println!("b = {}", b);

    let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(End))))));

    let y = 4;
    let x = &y;
    let z = Box::new(y);

    if *x == *z {
        println!("true");
    }

    let f = |i| i + 1;
    let x = 10;
    f(x);
    let p = || println!("This is a closure");
    p();

    let mut c = 0;
    let mut inc = || {
        c += 1;
        println!("incremented by 1: {}", c);
    };

    inc();
    inc();
    inc();

    let d = |i| i * 10;
    run(p);
    println!("3 * 10 = {}", add3(d));

    let v = vec![1, 2, 3];
    println!("v: {}", v.iter().any(|&x| x == 4));

    let s: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n| n < 10000)
        .filter(|&n| n % 2 == 0)
        .fold(0, |s, i| s + i);
    println!("{}", s);
}
