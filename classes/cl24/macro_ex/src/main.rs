use macros::debug_print;

// declarative macros matches pattern and replaces it with new patterns 
macro_rules! average {
    ($(,)*) => {{
        0.0
    }};
    ($($val:expr), + $(,)*) => {{
        let count = 0usize  $(+ {let _ = stringfy!($val); 1})*; //number of elements in the list how the fuck
        let sum = 0.0 $(+ $val as f64)*;
        sum / count as f64
    }};
}

#[debug_print]
fn main() {
    println!("Hello, world!");
    println!("{}", average!(1.0, 2.0, 3.0));
    println!("{}", average!(1, 2, 4, 5, 6));
}


