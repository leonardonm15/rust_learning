fn main() {
    let int= 9; // na stack;
    let s: String = "hello world".to_string(); // na stack
    print(s); 
    let mut s2 = String::from("Hello World");
    let s3 = "ola, mundo".to_string();

}

fn print(string: String) {
    println!("owning -> {}", string);
}
