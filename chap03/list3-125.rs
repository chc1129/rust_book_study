static mut MAXCOUNT: u32 = 6;

fn sum() -> u32 {
    let mut result = 0;
    unsafe {
        for i in 1..=MAXCOUNT {
            result += i as u32;
        }
    }
    result
}

fn main() {
    println!("{}", sum());
    unsafe{ MAXCOUNT = 12; }
    println!("{}", sum());
}
