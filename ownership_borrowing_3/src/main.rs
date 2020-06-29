fn take(v: Vec<i32>) {
    println!("Took v: {}", v[10] + v[100]);
}

fn re(v: Vec<i32>) -> Vec<i32> {
    println!("{}", v[10]);
    v
}

fn borrow1(v: &Vec<i32>) {
    println!("{}", (*v)[10]);
}

fn borrow2(v: &Vec<i32>) {
    println!("{}", v[10]);
}

fn main() {
    let x = 1; // x owns 1
    let y = x;
    {
        let a = 10;
    }
    // x + a; fails

    let s = String::from("String");
    let y = &s;
    println!("{}", s);

    let mut vect: Vec<i32> = Vec::new();
    for i in 1..1000 {
        vect.push(i);
    }

    take(vect);
    // println!("{}", vect[0]); This is moved to the function
    println!("Finished.");

    // doing this with primitives would not result in error

    let mut vect2: Vec<i32> = Vec::new();

    for i in 1..1000 {
        vect2.push(i);
    }

    vect2 = re(vect2);
    println!("Still own v: {} {}", vect2[0], vect2[1]);
    borrow1(&vect2);
    println!("Still own v: {} {}", vect2[0], vect2[1]);
    borrow2(&vect2);
    println!("Still own v: {} {}", vect2[0], vect2[1]);

    let vect3 = vec![2, 3, 6, 22, 4, 67, 567, 56, 75, 6, 34, 5, 345, 12];
    for &i in &vect3 {
        let count = count(&vect3, i);
        println!("{} is repeated {} times", i, count);
    }

    fn count(v: &Vec<i32>, val: i32) -> usize {
        v.into_iter().filter(|&&x| x == val).count()
    }
}
