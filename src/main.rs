trait HasArea {
    fn area(&self) ->f64;
}

struct Circle {
    radius: f64,
}
impl HasArea for Circle {
    fn area(&self) ->f64 {
        println!("Circle has an area:");
        std::f64::consts::PI* (self.radius *self.radius)
    }
}

struct Rect {
    x: f64,
    y: f64,
}
impl HasArea for Rect {
    fn area(&self) ->f64 {
        println!("Rect has an area:");
        self.x * self.y
    }
}

fn print_area<T: HasArea>(obj: T) {
    let area = obj.area();
    println!("{}", area);
}
fn main() {
    let c =Circle {
        radius: 3.27,
    };
    print_area(c);

    println!(" ");

    let r =Rect {
        x: 8.1,
        y: 9.3,
    };
    print_area(r);
}