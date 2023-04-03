fn main() {
    println!("Hello, world!");
    testano(5, 25);

    let numero = {
        // como se estivesse chamado uma func, auto return
        let y: i32 = 2;
        y + 1
    }; // nao bota o ; pra poder retornar o valor da ultima linha da expressão
    println!("numero {}", numero);

    let soma = somando_retornando(2, 3);
    println!("soma {}", soma);
}


fn somando_retornando(x: i32, y:i32) -> i32 {
    let retorno =  x + y;
    if x > y {return retorno + 100}
    else {return retorno}
}

fn testano(negoney: i32, y:i32) {
    println!("Teste foi chamado...");
    println!("soma: {}", negoney + y);
}