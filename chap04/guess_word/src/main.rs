use guess_word::Dictionary;

fn main() {
    println!("{}", Dictionary::new().get_random_word());
}
