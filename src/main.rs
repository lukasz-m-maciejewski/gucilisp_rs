mod parser;

fn main() {
    println!("{:?}", parser::parse("nil").1);
}
