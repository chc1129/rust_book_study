fn hoge() -> Option<i32> {
    let a = None;
    let b = a?;     // return Option<i32>::None
    Some(b)
}
