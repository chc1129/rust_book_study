pub trait Geometry {
    fn area(&self) -> f64;
    fn name(&self) -> &str { return "Geometry" }
}
