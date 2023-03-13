use std::rc::Rc;
fn main() {
    let str: String = "Leonardo".to_string();
    let point_str: Rc<String> = Rc::new(str);
    println!("first point_str count -> {:?}", Rc::strong_count(&point_str));
    let point_point_str: Rc<Rc<String>> = Rc::new(point_str.clone());
    println!("second point_str count -> {:?}", Rc::strong_count(&point_str));
    println!("point_point_str count -> {:?}", Rc::strong_count(&point_point_str));
    
}