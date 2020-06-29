fn main() {
    println!("Hello, world!");
    let t: (i32, f32, bool) = (4243, 10.2, false);
    let (a, b, c) = t;
    let (_, _, z) = t;

    let array = [1, 2, 3, 4, 5, 6, 7, 8];
    println!("{}", array[0]);
}
