use std::io::{self, Write};

fn main()  {
    loop{
        print!("$ ");
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
		match input.trim() {
			"exit 0" => {return ;},
			_ => {
				println!("{}: command not found", input.trim());
			}
		}
    }
}
