use std::io::{self, Write};

fn main()  {
    loop{
        print!("$ ");
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
		input = input.trim().to_string();
		match input.trim().split(" ").nth(0).unwrap() {
			"exit" => {return ;},
			"echo" => {println!("{}", input.split_off(5).trim())}
			_ => {
				println!("{}: command not found", input.trim());
			}
		}
    }
}
