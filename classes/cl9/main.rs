struct Uber{
    active: bool,
    usuario: String,
    sign_in_count: u32
}

struct Coordenadas(i32, i32, i32);

struct UnitStruct;

fn main() {
    println!("Hello, world!");
    let usuario = Uber{active: true, 
                            usuario: String::from("LEONARDO"), 
                            sign_in_count: 9};
    println!("{}", usuario.sign_in_count);
    
    let user2 = build_uber("maça verde".to_string());
    println!("{}", user2.active);

    let cords = Coordenadas(3, 3, 4);    
}

fn build_uber(usuario: String) -> Uber {
    Uber{
        active: true,
        usuario,
        sign_in_count: 3
    }
}
