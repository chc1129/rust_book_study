fn hoge() -> Option<i32> {
    let a = Some(10);
    let b = a?;
    Some(b)         // return 10
}
