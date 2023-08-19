fn type_of<T>(_: T) -> String {
    let a = std::any::type_name::<T>();
    a.to_string()
}
fn main() {
    let array: &[u8] = &[1,2,3];
    let cloned_array = array.clone();
    let fixed_array = &[1,2,3]; // 明示的にサイズを指定する
    let fixed_cloned_array = fixed_array.clone();
    println!("array: {}, cloned_array: {}",
        type_of(array), type_of(cloned_array));
    println!("fixed_array: {}, fixed_cloned_array: {}",
        type_of(fixed_array), type_of(fixed_cloned_array));
}
