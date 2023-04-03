use std::fmt::Debug;

fn main() {

    #[derive(Debug)]
    struct Bola {x: u32}

    fn generic_print<T: Debug>(printable: &T){
        println!("{:?}", printable)
    }

    let bola = Bola{x: 32};
    let blue = String::from("Blue");

    generic_print(&bola);
    generic_print(&blue);

}
// create a oprint funcion that uses generic T type
// gereic T typoe wil need to implement Debug depending on the values u pass in
// Our function takes one parameter of type T out function will then print
// out the value that is passed in 