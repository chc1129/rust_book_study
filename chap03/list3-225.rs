macro_rules! S {
    ( $($t:tt),* ) => {
        {
            let mut v = Vec::new();
            $(
                v.push(format!("{}", $t));
            )*
            v.join(" ")
        }
    };
}

fn main() {
    println!("{}", S!("Hello", "World!"));
}
