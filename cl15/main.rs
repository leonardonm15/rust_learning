use std::ops::Add;

fn main() {
    #[derive(Debug)]
    struct Pontos<T>{
        x: T,
        y: T
    }
    
    impl<T> Add for Pontos<T> 
        where
        T: Add<Output = T> { // essa sintaxe de : significa restringir o tipo T a caras que implementam o Add<> e o output dele e do mesmo tipo que ele proprio
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                    Pontos {
                        x: self.x + rhs.x,
                        y: self.y + rhs.y,
                    }
                }
            }
    
    let ponto1 = Pontos{x: 4, y: 4};
    let ponto2 = Pontos{x: 8, y: 8};
    let ponto3 = ponto1 + ponto2;
    
    println!("{:?}", ponto3)
}