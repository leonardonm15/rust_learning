struct Square {
    w: u32,
    h: u32
}

impl Square {
    fn sqr_sqr(&self) -> u32 {
        self.w * self.h
    }

    fn soma_lados(&self) -> u32 {
        self.w + self.h
    }

    fn change_h(&mut self, new_h: u32) {
        self.h = new_h
    }
}

fn main() {
    println!("Hello, world!");

    let mut sq = Square{w: 5, h: 5};

    println!("sqr -> {}", sq.sqr_sqr());
    println!("somal -> {}", sq.soma_lados());

    sq.change_h(7);
}
