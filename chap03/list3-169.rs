fn type_of<T>(_: T) -> String {
    let a = std::any::type_name::<T>();
    a.to_string()
}
fn main() {
    let borrowed_number: &u8 = &10;
    let owned_borrowed_number = borrowed_number.to_owned();

    let borrowed_array: &[&str; 3] = &["a", "b", "c"];
    let owned_borrowed_array = borrowed_array.to_owned();

    let string: &String = &String::from("Hello Word!");
    let owned_string = string.to_owned();

    println!("borrowed_number: {}, owned: {}",
        type_of(borrowed_number), type_of(owned_borrowed_number));
    println!("borrowed_array: {}, owned: {}",
        type_of(borrowed_array), type_of(owned_borrowed_array));
    println!("borrowed_string: {}, owned: {}",
        type_of(string), type_of(owned_string));

    let array: &[u8] = &[1,2,3];
    let owned_array = array.to_owned();
    let fixed_array = &[1,2,3]; // 明示的にサイズを指定する
    let fixed_owned_array = fixed_array.to_owned();

    println!("array: {}, owned_array: {}",
        type_of(array), type_of(owned_array));
    println!("fixed_array: {}, fixed_owned_array: {}",
        type_of(fixed_array), type_of(fixed_owned_array));
}
