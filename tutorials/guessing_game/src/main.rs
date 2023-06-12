use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("your secret is {secret_number}");

    loop {
            let mut guess = String::new();
        println!("Guess the number!");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("{guess}");
        let guess: u32 = guess.trim().parse().expect("Not a number");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Less"),
            Ordering::Equal => {
                println!("Correct {guess}");
                break;
            },
            Ordering::Greater => println!("Greater"),
        }
    }
}
