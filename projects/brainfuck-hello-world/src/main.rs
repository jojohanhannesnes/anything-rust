fn brainfuck(bf_code: String) -> String {
    let mut ip: usize = 0;
    let mut mem: [u8; 13] = [0; 13];
    let mut return_str: String = String::new();

    let mut i: usize = 0;
    while i < bf_code.len() {
        match &bf_code[i..i + 1] {
            "+" => {
                mem[ip] += 1;
            }
            "-" => {
                mem[ip] -= 1;
            }
            ">" => {
                if ip != 12 {
                    ip += 1;
                }
            }
            "<" => {
                if ip != 0 {
                    ip -= 1;
                }
            }
            "." => {
                return_str.push(mem[ip] as char);
            }
            "[" => {
                // begin loop
                if mem[ip] == 0 {
                    while &bf_code[i..i + 1] != "]" {
                        i += 1;
                    }
                }
            }
            "]" => {
                if mem[ip] != 0 {
                    while &bf_code[i..i + 1] != "[" {
                        i -= 1;
                    }
                }
            }
            _ => {}
        }

        i += 1;
    }

    return_str
}

fn main() {
    let result = brainfuck(
        ">++++++++++[<++++++++++>-]<++++.".into(), // >>++++++++++[<++++++++++>-]<+.>>>+++++++++++[<++++++++++<++++++++++>>-]<<--.>--.>>+++++++++++[<++++++++++>-]<+.>>++++[<++++++++>-]<.>>++++++++++++[<++++++++++>-]<-.>>+++++++++++[<++++++++++>-]<+.>>+++++++++++[<++++++++++>-]<++++.>>+++++++++++[<++++++++++>-]<--.>>++++++++++[<++++++++++>-]<.>>++++[<++++++++>-]<+.
    );
    println!("{}", result);
}
