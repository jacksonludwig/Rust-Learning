use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn exit(x: Option<i32>) {
    match x {
        Some(0) => panic!("0"),
        Some(x) => println!("good"),
        None => println!("none"),
    }
}

fn read_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("text.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let v = vec![1, 2];
    exit(None);

    let f = File::open("text.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("text.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Couldnt make file"),
        },
        Err(error) => panic!("file couldnt be opened"),
    };

    let f2 = File::open("text.txt").unwrap();

    if let Ok(f) = read_file() {
        println!("read the file!");
    } else {
        println!("Errored");
    }
}
