use std::thread;
fn main() {
	let handle = std::thread::spawn(move || {
		println!("hello from a thread");
	});
	handle.join().unwrap(); // blocks the main thread until the thread using the join ends executing
	println!("hello from main"); //this will always go after the handle thread ends




	// part2

	let v = vec![1, 2, 3];
	let handle = std::thread::spawn(move || { // move forces the thread to own the values that it uses from the main thread
	//	println!("{:?}", v);
	});		
	// println!("{:?}" v); <- this wont work because the value is got out of scope when the handle thread ended

	let mut thread_handles = Vec::new();
	for e in v {
		thread_handles.push(thread::spawn(move || println!("Thread {}", e)));
	}
	
	println!("Main thread!");
	for handle in thread_handles {
		handle.join().unwrap();
	}
}
