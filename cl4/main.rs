use std::io;

fn main() {
    println!("Hello, world!");
    let mut inp = String::new();

    io::stdin().read_line(&mut inp).expect("failed to read line");

    println!("{}", inp);
}
