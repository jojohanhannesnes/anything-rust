use std::io;

fn main() {
    let mut first_number = 0;
    let mut second_number = 1;
    let mut result = 0;
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("cannot read input");
    let mut number: i32 = number.trim().parse().expect("cannot parse number");
    if number == first_number {
        result = first_number;
    }
    if number == second_number {
        result = second_number;
    }
    while number > 1 {
        result = first_number + second_number;
        first_number = second_number;
        second_number = result;
        number-=1;
    }

    println!("{result}")

}
