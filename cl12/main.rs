enum Pets { 
    dog,
    cat,
    fish
}

// usamos o static lifetime ali pq como o &str é
// emprestado de uma string, ao retornamos esse tipo
// precisamos especificar que esse cara nao tem dono

impl Pets {
    fn waht_am_i(self) -> &'static str {
        match self {
            Pets::dog => "i am a dog",
            Pets::fish => "i am a fish",
            Pets::cat => "i am a cat",
        }
    }
} 

enum IpAdrrKind {
    V4(String),
    V6
}

struct EnderecoIP {
    tipo: IpAdrrKind,
    endereco: String
}


fn main() {
    let dog = Pets::dog;
    println!("{}", dog.waht_am_i());

    let casa = IpAdrrKind::V4("x".to_string());


    let home = EnderecoIP {
        tipo: IpAdrrKind::V6,
        endereco: "12.124.12".to_string()
    };

    let looppack = EnderecoIP {
        tipo: IpAdrrKind::V4("zaza".to_string()),
        endereco: String::from("value"),
    };

    let some_number = Some(5);
    let some_string = Some("STRING");
    let nothing: Option<i32> = None;

    let x: i32 = 5;
    let y: Option<i32> = Some(5);

    let dog2 = Some(Pets::dog);
    
    if let Some(Pets::dog) = dog2 {
        println!("o animal é um cachorro");
    } else {
        println!("não é um cachorro");
    }
    
    let mut vet = vec![1, 2, 3, 4];

    while let Some(top) = vet.pop() {
        println!("{:?}", top)
    }

    // a gente usa o option quando nao sabe oq vai ser retornado
    // dps se usa um match expression pra ver oq tem dentro daquele option
    // bom pra tratamento de erro
    //let sum = x + y;

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn waht_pet(input: &str) -> Option<Pets> {
    match input {
        "Dog" => Some(Pets::dog),
        "fish" => Some(Pets::fish),
        "cat" => Some(Pets::cat),
        _ => None
    }
}