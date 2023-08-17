fn put<T: std::fmt::Debug>(a: &T) {
    println!("{:?}", a);
}

fn main() {
    put("hoge");
}
