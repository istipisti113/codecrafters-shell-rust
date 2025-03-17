use std::io::{self, Write};
use std::collections::HashMap;

fn main()  {
	let mut builtins: HashMap<String, Box<dyn Fn(String)-> i32>> = HashMap::new();
	builtins.insert(
		"echo".to_string(),
		Box::new(
			move |mut input: String|{
				println!("{}", input.split_off(5));
				return 0;
			}
		)
	);
	builtins.insert(
		"exit".to_string(),
		Box::new(
			move |_input: String|{
				return 1;
			}
		)
	);
	// builtins.insert(
	// 	"type".to_string(),
	// 	Box::new(
	// 		move |input: String|{
	// 		}
	// 	)
	// );
    loop{
        print!("$ ");
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
		input = input.trim().to_string();
		if builtins.contains_key(input.split(" ").nth(0).unwrap()){
			if builtins[&input](input) == 1 {
				return;
			}
		}else if input.split(" ").nth(0).unwrap() == "type" {
			let a = input.split(" ").nth(1).unwrap();
			if builtins.contains_key(a) {
				println!("{} is a shell builtin", a);
			}
		}else{
			println!("{}: command not found", input.trim());
		}
    }
}
