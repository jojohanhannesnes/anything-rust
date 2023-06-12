use std::io;

fn main() {
    let mut result = String::new();
    loop {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("cannot read command");
        let command = command.trim().parse().expect("cannot parse message");
        match command {
            1 => {
                println!("input the words pig latin");
            }
            _ => {
                std::process::exit(1);
            }
        }
        let mut words = String::new();
        io::stdin()
            .read_line(&mut words)
            .expect("cannot read command");
        let words = words.trim();
        if let Some(word) = words.chars().next() {
                result = match word.to_ascii_lowercase() {
                    'a' | 'i' | 'u' | 'e' | 'o' => {
                       format!("{}-hay",words)
                    }
                    _ => {
                        format!("{}-{}ay",words[1..words.len()].to_string(), word)
                    }
                }
            }
            println!("{}", result);
        }

}
