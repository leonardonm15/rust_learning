enum Poligons {
    triangle,
    square,
    pentagon,
    octagon
}

impl Poligons {
fn corners(&self) -> u8{
    match self {
        Poligons::triangle => 3,
        Poligons::square => 4,
        Poligons::pentagon => 5,
        Poligons::octagon => 8
    }
}
}


fn main() {
    let tri = Poligons::triangle;
    let oct = Poligons::octagon;
    println!("{:?}", tri.corners());
    println!("{:?}", oct.corners());
}