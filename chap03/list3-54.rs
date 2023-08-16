fn copy_trait_check<T: Copy>(_: T) {}   // trait bound

fn main() {
    let s = String::from("hello");      // String
    copy_trait_check(s);
                //   ^ the trait `Copy` is not implemented for `String`
                // error[E0277]: the trait boudn `String: Copy` is not satisfied
    let a = 10;          // i32
    copy_trait_check(a); // OK
}
