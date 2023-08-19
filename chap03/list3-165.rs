fn type_of<T>(_: T) -> String {
    let a = std::any::type_name::<T>();
    a.to_string()
}
fn main() {
    let str = "a";
    let cloned_str = str.clone();
    let array: &[u8] = &[1,2,3];
    let cloned_array = array.clone();
    println!("str: {}, cloned_str: {}, array: {}, cloned_array: {}",
        type_of(str), type_of(cloned_str),
        type_of(array), type_of(cloned_array));
}
