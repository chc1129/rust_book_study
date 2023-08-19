fn type_of<T>(_: T) -> String {
    let a = std::any::type_name::<T>();
    a.to_string()
}
