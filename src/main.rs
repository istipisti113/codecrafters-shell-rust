use std::io::{self, Write};
use std::collections::HashMap;
use std::env;
use std::path::Path;

fn main()  {
    //let mut builtins: HashMap<String, Box<dyn Fn(String)-> i32>> = HashMap::from([
    let mut commands: Vec<String> = Vec::new();
    let builtins: HashMap<String, Box<dyn Fn(String, &Vec<String>)-> i32>> = HashMap::from([
        (
            "echo".to_string(),
            Box::new(
                move |mut input: String, _commands: &Vec<String>|{
                    println!("{}", input.split_off(5));
                    return 0;
                }
            ) as Box<dyn Fn(String, &Vec<String>)->i32>
        ),
        (
            "exit".to_string(),
            Box::new(
                move |_input: String, _commands: &Vec<String>|{
                    return 1;
                }
            )
        ),

        (
            "type".to_string(),
            Box::new(
                move |input: String, commands: &Vec<String>|{
                    let prog: String = input.split(" ").nth(1).unwrap().to_string();
                    if commands.contains(&prog){
                        println!("{} is a shell builtin", &prog);
                        -1
                    } else {
                        match env::var("PATH") {
                            Ok(val) => {
                                for dir in val.split(":"){
                                    match Path::new(dir).read_dir() {
                                        Ok(val)=>{
                                            let bins = val.map(|x| x.unwrap().file_name()).map(|x| x.into_string().unwrap()).collect::<Vec<String>>();
                                            //println!("{}", bins.join(", "));
                                            if bins.contains(&prog){
                                                println!("{} is {}/{}", &prog, dir, &prog);
                                                return 0;
                                            }
                                        },
                                        Err(_e)=>{} //nix shenanigans, some directories dont actually exist
                                    }
                                }
                                println!("{}: not found", prog);
                                return -1;
                            },
                            Err(_e) => {
                                println!("path is not found or readable or idk");
                                -1
                            },
                        }
                    }
                }
            )
        )
    ]);
    commands = builtins.keys().map(|x| x.to_string()).collect();
    loop{
        print!("$ ");
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        input = input.trim().to_string();
        let command = input.split(" ").nth(0).unwrap();
        if builtins.contains_key(command){
            if builtins[command](input, &commands) == 1 {
                return;
            }
        }
        /*
        else if command == "type" {
            let a = input.split(" ").nth(1).unwrap();
            if builtins.contains_key(a) || a == "type" {
                println!("{} is a shell builtin", a);
            } else {
                println!("{}: not found", a);
            }
        }*/
        else{
            println!("{}: command not found", input.trim());
        }
    }
}
