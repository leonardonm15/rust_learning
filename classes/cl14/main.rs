struct Point<T> {
    x: T,
    y: T // same type for both of em
}

struct Point2<T, U> {
    x: T,
    y: U
}

trait Overview {
    fn overview(self) -> String;
}

struct Course { 
    headline: String,
    Author: String
}

struct Course2 { 
    headline: String,
    Author: String
}

impl Overview for Course {
    fn overview(self) -> String {
        format!("{}, {}", self.Author, self.headline);
    }
}


fn main() {
    let coordinate = Point{x: 5.0, y: 6.0};
    let str_coordinate = Point{x:"five", y:"six"};
}
