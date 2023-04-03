// thats a declarative macro
macro_rules! gcd {
    ($a: expr, $b: expr) =>  {
        {
            let mut m = $a;
            let mut n = $b;
            while m!=0 {
                if m < n {
                    let t = m;
                    m = n;
                    n = t;
                }
                m = m % n;
            }
            n
        }
    };
}
// this is a procedural macro
// atribute macro add metadata at itens
// funcion-like macro define a res
// derive macro, impl traits for enums and structs
fn main() {
    println!("{}", gcd!(14, 15));
}

