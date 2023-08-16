struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let Point(x,y,z) = Point(5, 4, 1);
    println!("{} {} {}", black.0, black.1, black.2);
    println!("{} {} {}", x, y, z);
}
