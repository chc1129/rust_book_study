fn type_of<T>(_: T) -> String {
    let a = std::any::type_name::<T>();
    a.to_string()
}
fn main() {
    let str = "a";
    let owned_str = str.to_owned();
    let array: &[u8] = &[1,2,3];
    let owned_array = array.to_owned();
    println!("owned_str: {}, owned_array: {}",
        type_of(owned_str), type_of(owned_array));
}
