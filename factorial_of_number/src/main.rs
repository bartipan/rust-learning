use std::io;

fn main() {
    println!("Enter the number you want the factorial to be calculated for:");

    let number = loop {
        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read the input");

        match number.trim().parse() {
            Err(_) => println!("Please, type a positive number!"),
            Ok(i) => break i,
        };
    };

    let mut result: u64 = 1;

    for factorial in (1..=number).rev() {
        result *= factorial;
    }

    println!("The factorial of the number {} is {}.", number, result)
}
