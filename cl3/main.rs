fn main() {
    println!("Hello, world!");

    let x: i32 = 41;
    // dttypes u & i 8 - 16 - (32) - 64 - 128
    //         f32 - (f64) default

    let floating: f32 = 10.92;
    
    let true_or_false: bool = true;

    let letter: char = 'l';
    println!("{}", letter);


    let tuple: (i32, bool, char)= (1 , true, 's'); // imutavel
    // tuple.2 = "dunno"
    println!("{}", tuple.0);

    let arr: [i32; 4] = [1 , 3, 2, 4];
    // let mut arr to change and add elements
    // arr[3] -> pythonlike 


}
