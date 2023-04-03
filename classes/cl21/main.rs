use std::thread;
use std::sync::{Mutex, Arc};
fn main() {
	let rc1 = Arc::new(String::from("Test"));
	let rc2 = rc1.clone();
	std::thread::spawn(move || {
		rc2;
	});

	// PART 2 MUTEX, ONE THREAD ACCES TO A DATA AT ANY GIVEN TIME
	// to use the data u must lock the data, and afeter use u must unlock it to let other threads use it

	let counter = Arc::new(Mutex::new(0));
	let mut handles = vec![];
	for _ in 0..8 {
		let counter = Arc::clone(&counter);
		let handle = std::thread::spawn(move || {
			let mut num  = counter.lock().unwrap();
			// let mut num2  = counter.lock().unwrap(); DEADLOCK ERROR
			*num += 1;
		});
		handles.push(handle);
	}

	for handle in handles {
		handle.join().unwrap();
	}

	println!("{}", counter.lock().unwrap());

	// THREAD POISONING - HAPPENS WHENEVER A THREAD PANICS WHILE HOLDING THE LOCK
	let lock = Arc::new(Mutex::new(0));
	let lock2 = Arc::clone(&lock);

	let _ = std::thread::spawn(move || -> () {
		let _guard = lock2.lock().unwrap();
		panic!();
	}).join();

	// recover from the poisoning with match statement
	let mut guard = match lock.lock() {
		Ok(guard) => guard,
		Err(poisoned) => poisoned.into_inner(),
	};

	*guard += 1;
	println!("{:?}", guard);
}
