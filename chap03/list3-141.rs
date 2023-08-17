fn put<T: std::fmt::Debug + ?Sized>(a: &T) {
    println!("{:?}", a);
}

fn main() {
    put("hoge");
}
