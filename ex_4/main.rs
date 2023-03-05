#[derive(Debug)]
struct Car {
    mpg: u32,
    color: String,
    t_s: f32
}

#[derive(Debug)]
struct Motorcycle {
    mpg: u32,
    color: String,
    t_s: f32
} 

enum Vehicles {
    Motorcycle,
    Car
}

trait ChangeStuff {
    fn set_mpg(&mut self, _mpg: u32){}
    fn set_color(&mut self, _col: String){}
    fn set_ts(&mut self, _new_ts: f32) {} 
}


impl ChangeStuff for Car {
    fn set_mpg(&mut self, _mpg: u32){
        self.mpg = _mpg
    }
    fn set_color(&mut self, _col: String){
        self.color = _col
    }
    fn set_ts(&mut self, _new_ts: f32) {
        self.t_s = _new_ts
    } 
} 


impl ChangeStuff for Motorcycle {
    fn set_mpg(&mut self, _mpg: u32){
        self.mpg = _mpg
    }
    fn set_color(&mut self, _col: String){
        self.color = _col
    }
    fn set_ts(&mut self, _new_ts: f32) {
        self.t_s = _new_ts
    } 
}


fn main() {
    let bl = "blue".to_string();
    let bl2 = "blue".to_string();

    let mut carro = Car {
        mpg: 1,
        color: bl2,
        t_s: 1.0
    };
    let mut moto = Motorcycle {
        mpg: 1,
        color: bl,
        t_s: 1.0
    };
    println!("{:?}", carro);

    carro.set_color("Leonardo".to_string());
    carro.set_mpg(232);
    carro.set_ts(283.93);

    moto.set_color("Leonardo".to_string());
    moto.set_mpg(232);
    moto.set_ts(283.93);
    

    println!("{:?}", carro);
    println!("{:?}", moto);
}
