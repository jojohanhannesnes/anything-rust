use std::io;
use std::collections::HashMap;

fn main() {
    let mut library = HashMap::new();
    loop {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("cannot read command");
        if command.as_str() == "exit\n" {
            break;
        }
        let command: Vec<&str> = command.split_whitespace().collect();
        let department = command[3].to_string();
        let employee = command[1].to_string();
        library.entry(department)
            .and_modify(|v| *v = employee.clone())
            .or_insert(employee);
            
    }
    println!("{:?}", library);
}
