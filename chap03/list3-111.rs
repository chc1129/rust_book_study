struct Hoge {
    value: i32,
}

fn main() {
    let a = Some(Hoge { value: 10 });
    let b = a.unwrap();
//            --------`a` moved due to this method call
    let c = a.unwrap();
//          ^ value userd here after move
    println!("{}", c.value);
}
