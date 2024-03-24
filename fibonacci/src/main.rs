fn main() {
    let number = 10;

    let mut fb1: u64 = 1;
    let mut fb2: u64 = 1;

    while fb1 < number {
        print!("{} ", fb1);

        let fbn = fb1 + fb2;
        fb1 = fb2;
        fb2 = fbn;
    }
}