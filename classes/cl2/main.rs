fn main() {
    // let x = 10;
    // x = "hello"; // nao da pq x ja é int
    let mut x = 10;
    println!("x is : {}", x);
    x = 5;
    println!("x is : {}", x);

    let y = 4;
    println!("y is : {}", y);

    {
        let y = y + 10;
        println!("y is : {}", y);
    }

    let y = y + 1;
    println!("y is : {}", y);

    const SEGUNDOS_EM_MINUTOS: u32 = 60;
    // const SEGUNDOS_EM_MINUTOS: u32 = 100;
    println!("{}: segundo em minutos", SEGUNDOS_EM_MINUTOS)
}
