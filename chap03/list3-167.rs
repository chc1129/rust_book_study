fn type_of<T>(_: T) -> String {
    let a = std::any::type_name::<T>();
    a.to_string()
}
fn main() {
    let borrowed_number: &u8 = & 10;
    let cloned_borrowed_number = borrowed_number.clone();

    let borrowed_array: &[&str; 3] = &["a", "b", "c"];
    let cloned_borrowed_array = borrowed_array.clone();

    let string: &String = &String::from("Hello World!");
    let cloned_string = string.clone();

    println!("borrowed_number; {}, cloned: {}",
        type_of(borrowed_number), type_of(cloned_borrowed_number));
    println!("borrowed_array; {}, cloned: {}",
        type_of(borrowed_array), type_of(cloned_borrowed_array));
    println!("borrowed_string; {}, cloned: {}",
        type_of(string), type_of(cloned_string));
}
