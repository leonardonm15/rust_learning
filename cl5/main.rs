use std::io;

fn main() {

    let leo: i64 = 12;
    let mae: i32 = 10;

    let z = leo + (mae as i64);
    // nao da pra somar tipos de dados diff

    // ele cospe erro caso de overflow

    // se dividir dois inteiros sempre vai dar um inteiro, 
    // nao tem virgula

    let leo: f64 = 12.0;

    let z: f64 = leo / (mae as f64);

    println!("{}", z);

    // cast func

    let leo = 90_f32;
    let leo = 90 as f64;

    let macaco: i32 = 128;

    let z = leo / (macaco as f64); // z == f64
    println!("{}", z);

    // i32::MAX -> da pra fazer com todos os tipos
    

    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("expected to read line");

    let input_to_int: i64 = inp
                                .trim() // trim clears \n spaces n shit
                                .parse() // parse takes the string a make it into a number if its possible
                                .unwrap(); // turns it to the datatype u specified and return to us

    println!("{}", input_to_int + 3);
}
