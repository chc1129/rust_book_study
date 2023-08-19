use std::ops::Add;
#[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, othr: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
