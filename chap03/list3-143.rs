fn eturns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}
