use std::io;

fn main() {
    loop {
        println!("Select conversion:");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");
        println!("3. Quit");

        let choice: i32 = read_input::<i32>("Enter your choice:");

        match choice {
            1 => {
                let fahrenheit: f32 = read_input::<f32>("Enter the temperature in Fahrenheit:");
                let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
                println!("Temperature in Celsius: {:.2}", celsius);
            }
            2 => {
                let celsius: f32 = read_input::<f32>("Enter the temperature in Celsius:");
                let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
                println!("Temperature in Fahrenheit: {:.2}", fahrenheit);
            }
            3 => break,
            _ => println!("Invalid choice! Please try again."),
        }
    }
}

fn read_input<T: std::str::FromStr>(prompt: &str) -> T {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim().parse::<T>().unwrap_or_else(|_| {
        std::process::exit(1);
    });
    input
}