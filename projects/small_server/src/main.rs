use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        handle_connection(_stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buff_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buff_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("request: {:#?}", http_request);
}