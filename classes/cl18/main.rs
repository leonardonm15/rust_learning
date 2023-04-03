#[derive(Debug)]
struct City {
    city: String,
    population: u64
}

#[derive(Debug)]
struct Item {
    name: String
}

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32
}

impl Iterator for Range {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }
        let result = Some(self.start);
        self.start += 1;
        result
    }
}

// fn sort_poulation(city: &mut Vec<City>) {
//     city.sort_by_key();
// }

// fn sort_helper(pop: &City) -> u64 {
//     pop.population
// }

fn sort_pop_closure(pop: &mut Vec<City>) {
    pop.sort_by_key(|p| p.population)
}

fn check_inventory(items: Vec<Item>, product: String) -> Vec<Item> {
    items.into_iter().filter(|p| p.name == product).collect()
}


fn main() {
    let a = City{city: "Leonardo".to_string(), population: 1000};
    let b = City{city: "Leonardo".to_string(), population: 10};
    let c = City{city: "Leonardo".to_string(), population: 10000};
    let d = City{city: "Leonardo".to_string(), population: 10000000};

    let mut vec: Vec<City> = Vec::new();

    vec.push(a);
    vec.push(b);
    vec.push(d);
    vec.push(c);

    sort_pop_closure(&mut vec);
    println!("{:?}", vec);

    let add = |x: i32| -> i32 {x + 1};
    let add_v2 = |x| x + 1;
    add_v2(1);


    let vec = vec!{1, 2, 3};
    for val in vec.iter() {
        println!("{}", val)
    }

    let vec2 = vec![1, 2, 3];
    let mut iter = (&vec2).into_iter();
    while let Some(v) = iter.next() {
        println!("{}", v);
    }

    let mut vec: Vec<Item> = Vec::new();

    vec.push(Item{name: "Leonardo".to_string()});
    vec.push(Item{name: "Lucas".to_string()});
    vec.push(Item{name: "Mãe".to_string()});

    let checado = check_inventory(vec, "Leonardo".to_string());
    println!("{:?} <- ", checado);


    let mut range = Range{start: 0, end: 10};
    // for r in range {
    //     println!("{}", r);
    // }

    let vec: Vec<u32> = range.filter(|x| x % 2 == 0).collect();
    println!("{:?}", vec);

}


