fn main() {
    let a:i32 = 10;
    let b: Box<i32> = Box::new(10);

    let c = a * *b;
    println!("numero -> {:?}", c);
}