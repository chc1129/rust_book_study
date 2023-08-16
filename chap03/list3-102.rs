struct Account { name: String, pass: String }

fn main() {
    let a = Account {
        name: String::from("name"), pass: String::from("pass")
    };
    let Account { name, pass } = a;    // move ownership
    println!("{} {}", name, pass);     // borrow check!! - OK
    println!("{} {}", a.name, a.pass); // borrow check!! - Error
// |                  ^^^^^^ value borrowed here after move
// |
// = note: move occurs because `a.name` has type `String`,
//         which does not implement the `Copy` trait
}
