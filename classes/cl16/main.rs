use std::fs::File;
use std::io::ErrorKind;
use std::fs::rename;
use std::io::Error;

fn main() {
    // panic!("panicked here")

    let vec = vec![1];
    vec[10];

    let file = File::open("error.txt");


    // example 1
//    let file = match file {
//       Ok(file) => file,
//        Err(error) => panic!("Err: {:?}", error)
//    };

    // example 2
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("error.txt") {
                Ok(file_created) => file_created,
                Err(err) => panic!("Cannot create a file")
            }
            _ => panic!("dunno")
        }
        _ => panic!("fodase")
    };

    let file = File::open("error.txt").expect("Error opening the file");


    // error progagation
    let test = open_file();
    test.unwrap();


}

fn open_file() -> Result<File, Error> {
    let file = File::open("error.txt")?;
    Ok(file)
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}