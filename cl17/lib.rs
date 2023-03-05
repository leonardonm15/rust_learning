#[cfg(test)]
mod test {
    use super::*; // can use everythin inside or ouside the scope

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn it_fails() {
        panic!("test failed");
    }
    #[test]
    #[ignore]
    fn maybe() {
        let leo = "leo".to_string();
        assert_ne!(leo, String::from("asd"));
    }

    #[test]
    fn talvez() {
        let leo = "leo".to_string();
        assert_ne!(leo, String::from("leo"));
    }


    #[test]
    fn call_simple_add() {
        assert!(simple_add()); // return True or False
    }

}


fn simple_add() -> bool {
    if 2+2 == 4 {
        true
    } else {
        false
    }
}