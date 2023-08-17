// Tをstrに置き換えたもの
fn put<str: std::fmt::Debug + Sized>(a: &str) {
    println!("{:?}", a);
}

fn main() {
    put("hoge");
}
