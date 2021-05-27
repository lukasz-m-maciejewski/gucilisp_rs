#![allow(dead_code)]
mod parser;

fn make_adder(n: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| {x + n} )
}

fn main() {
    let cl = | x | { x * x };
    println!("{:?}", cl(7));
    println!("{}", make_adder(2)(4));
}
