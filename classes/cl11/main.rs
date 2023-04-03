fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("{}", r);
}

fn example<'a, 'b>(x: &'a str, y:&'b str) -> &'a str {
    x
}

fn example2<'a, 'b>(x: &'a str, y: &'b str) -> &'b str {
    y
}