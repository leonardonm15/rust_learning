use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Flagger {
    is_true: Rc<RefCell<bool>>
}

fn main() {
    // smart pointers original in c++
    //BOX allows to alocate on the heap rather than the stack
    let t = (12, "EGGS"); // created on the stack
    let b = Box::new(t); // created on the heap but stored on the stack
    println!("{:?}", b);

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    // assert_eq!(5, y); // y is a int* of x thats why i throws an error
    assert_eq!(5, *y); // this one works, just like in C

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, *y);

    println!("{:?}", y);


    // Rc counts the and lets multiple pointers to a single address in memory
    // so, cloning a RC mens that the where creating a pointer to the same location 
    let s1 = Rc::new(String::from("Pointer"));
    let s2 = s1.clone();
    let s3 = s2.clone();

    let flag = Flagger{is_true: Rc::new(RefCell::new(true))};
    // borrow returns Ref<T>
    // borrow_mut return RefMut<T>

    // Rc + RefCell is helps with multiple "objects" changing the status
    // of one variable
    let reference = Rc::new(flag.is_true.clone());
    println!("{:?}", *reference); // -> true

    let mut mut_ref = flag.is_true.borrow_mut();
    *mut_ref = false; 

}
