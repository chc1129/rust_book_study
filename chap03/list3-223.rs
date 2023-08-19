macro_rules! S {
    ($l:literal) => { format!("literal:{}", $l) };
    ($e:expr) => { format!("expr:{}", $e) };
    ($t:tt) => { format!("token:{}", $t) };
}

fn main() {
    println!("{}", S!("Hello, World!"));
}
