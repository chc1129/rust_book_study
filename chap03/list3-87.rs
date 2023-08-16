// 関数
fn add<T: std::ops::Add<Output=T> >(a: T, b: T) -> T {
    a+b
}

// 構造体
struct Point<T> { x: T, y: T }

// 列挙型
enum Reuslt<T,E> {
    Ok(T),
    Err(E),
}
