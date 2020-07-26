fn main() {
    let tup = (1, 'b', true);
    let tup2 = (1, 2, ('b', false));
    println!("{} {}", tup.0, tup.1);
    println!("{:#?}", tup2);

    let array: [i32; 5] = [5, 2, 1, 2, 4];
    println!("{} {}", array[0], array.len());

    let arr_slice = &array[2..4];
    println!("{} {}", arr_slice[0], arr_slice[1]);

    let string_slice = "String?";
    let string = String::from("String!");
    println!("{}", &string[0..2]);

    let a = String::from("Hello, ");
    let b = String::from("World");
    let c = a + &b;
    println!("{}", c);

    let empty = ();
}
