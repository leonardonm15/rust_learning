fn main() {
    fn test1() {
            let mut vector = vec![1, 3, 5, 7];
            check(&vector);
            vector.push(15);
            println!("vector {}", vector[4]);

        fn check(val: &[u8]) -> bool {
            if val[0] == 1 {
                println!("TRUE");
                true
            } else {
                println!("FALSE");
                false
            }
        }
    }
    fn test2() {
        fn add_two(num1:i8) -> i8 {
            num1 + 2
        }

        let num2 = 8;
        let num3 = add_two(num2);
        println!("num2 -> {}", num3);
    }
    test2();
}


