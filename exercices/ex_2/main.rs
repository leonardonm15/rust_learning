#[derive(Debug)]

struct Car {
    mpg: u32,
    color: String,
    t_s: f32
}

impl Car {
    fn set_mpg(&mut self, _mpg: u32) {
        self.mpg = _mpg
    }
    fn set_color(&mut self, col: String) {
        self.color = col
    }
    fn set_ts(&mut self, new_ts: f32) {
        self.t_s = new_ts
    }
}

fn main() {
    let bl = "blue".to_string();
    let mut carro = Car {
        mpg: 1,
        color: bl,
        t_s: 1.0
    };
    println!("{:?}", carro);

    carro.set_color("Leonardo Nunes Muniz".to_string());
    carro.set_mpg(232);
    carro.set_ts(283.93);

    println!("{:?}", carro);
}
