trait Shape {
    fn area(&self) -> u32;
}

struct Rectangle {
    x: u32,
    y: u32,
}

struct Circle {
    radius: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.x * self.y
    }
}

impl Shape for Circle {
    fn area(&self) -> u32 {
        (3.141 * self.radius * self.radius) as u32
    }
}

#[derive(Debug)]
struct A;

use std::ops;
struct B;

struct AB;

impl ops::Add<B> for A {
    type Output = AB;
    fn add(self, _rhs: B) -> AB {
        AB
    }
}

#[derive(Clone)]
struct Test {
    a: u32,
}

impl Drop for Test {
    fn drop(&mut self) {
        println!("Dropped {}", self.a);
    }
}

impl Iterator for Test {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        Some(self.a)
    }
}

struct Square<T> {
    x: T,
}

use std::fmt;
fn p<T: fmt::Debug>(x: T) {
    println!("{:?}", x);
}
struct Thing<T> {
    x: T,
}

impl<T> Thing<T> {
    fn item(&self) -> &T {
        &self.x
    }
}

struct D<U, V> {
    x: U,
    y: V,
}

trait Shape2<T> {
    fn area(&self) -> T;
}

use std::ops::Mul;
struct Rectangle2<T: Mul> {
    x: T,
    y: T,
}

impl<T: Copy> Shape2<T> for Rectangle2<T>
where
    T: Mul<Output = T>,
{
    fn area(&self) -> T {
        self.x * self.y
    }
}

fn main() {
    let c = Circle { radius: 100.2 };
    let r = Rectangle { x: 30, y: 20 };
    println!("{} {}", c.area(), r.area());

    let s1 = Square { x: 1 };
    let s2 = Square { x: "Hey" };
    let s3 = Square { x: 2.5 };

    let a = Thing { x: "Hello!" };
    a.item();
}
