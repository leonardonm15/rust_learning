
fn main() {
    let mut vec = vec![1, 2, 5, 7, 9];
    vec = vec.into_iter().map(|x| x * 10).collect();

    println!("{:?}", vec);
}