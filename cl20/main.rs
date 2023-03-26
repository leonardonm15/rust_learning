use std::sync::mpsc;
use std::thread;
fn main() {
    println!("Hello, world!");
	let (transmitter, reciever) = mpsc::sync_channel(1000);
	let tx = transmitter.clone();	

	// let val = String::from("Transmitting!");
	// std::thread::spawn(move || {
		// transmitter.send(val).unwrap();
	// });

	// let msg = reciever().recv().unwrap(); // recv() takes the onwnership of the value passed thru the transmitter
	// println!("{}", msg);

	std::thread::spawn(move || {
		let vec = vec![String::from("Transmitting"), String::from("Original")];
		for val in vec {
			transmitter.send(val).unwrap();
		}
	});		
	std::thread::spawn(move || {
		let vec = vec![String::from("Leonardo"), String::from("Muniz")];
		for val in vec {
			tx.send(val).unwrap();
		}
	});		
	
	for rec in reciever {
		println!("{}", rec);
	}
}
