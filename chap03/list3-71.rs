struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self { // Selfは実装している型のエイリアス
        Self { x, y }
    }
}

fn main() {
    let a = Point::new(3., 5.);
    print!("x={}, y={}", a.x, a.y);
}
