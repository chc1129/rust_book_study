macro_rules! S {
    ($e:expr) => { format!("expr:{}", $e) };
    ($t:tt) => { format!("token:{}", $t) };
    ($l:literal) => { format!("literal:{}", $l) };
}

fn main() {
    println!("{}", S!("Hello, World!"));
}
