extern crate modules_lifetimes_testlib_11;
use modules_lifetimes_testlib_11::A::B;

fn pr<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() == y.len() {
        x
    } else {
        y
    }
}

struct A<'a, 'b> {
    x: &'a str,
    y: &'b str,
}

impl<'a, 'b> A<'a, 'b> {
    fn slf(&self) -> &str {
        self.x
    }
}

fn ab<'a, 'b>(x: &'a i32, y: &'b i32) {}

// would be inferred
fn a<'a>(s: &'a str) -> &'a str {
    s
}

fn main() {
    let x = 10; //'a
    {
        let y = 10; //'b
                    // x = &y;
    }
    println!("{}", x);

    let a = "a string";
    let b = "a string";
    let c = pr(a, b);
    println!("{}", c);

    let a = A {
        x: "Hello",
        y: "There",
    };

    let s: &'static str = "The String";
}
