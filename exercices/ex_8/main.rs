macro_rules! op {
    ($num1: expr, $num2: expr, $num3: expr) => {{
        let m = $num1;
        let n = $num2;
        let k = $num3;
        match k {
            1 => m + n,
            2 => m - n,
            3 => m * n,
            4 => m / n,
            5 => m % n,
            _ => panic!("Invalid value of k"),
        }
    }};
}

fn main() {
    println!("Hello, world!, {}", op!(5, 2, 2));
}

#[test]
fn test() {
    assert_eq!(op!(5, 2, 1), 7);
}

#[test]
fn test2() {
    assert_eq!(op!(5, 2, 2), 3);
}

#[test]
fn test3() {
    assert_eq!(op!(5, 2, 3), 10);
}

#[test]
fn test4() {
    assert_eq!(op!(5, 2, 5), 1);
}
