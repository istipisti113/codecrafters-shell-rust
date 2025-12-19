use std::fmt::format;
use std::io::{self, Write};
use std::collections::HashMap;
use std::{env, fs};
use std::path::Path;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

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
                                match findbin(&prog, &val) {
                                    Ok(dir) => {
                                        println!("{} is {}/{}", prog, dir, prog);
                                        0
                                    },
                                    Err(e)=>{
                                        println!("{}", e);
                                        -1
                                    }
                                }
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
        let path = env::var("PATH").unwrap();
        if builtins.contains_key(command){
            if builtins[command](input, &commands) == 1 {
                return;
            }

        }
        else{
            match findbin(&command.to_string(), &path) {
                Ok(val)=>{
                    let full = val.to_string()+"/"+command;
                    let output = Command::new(command)
                        .args(input.split(" ").collect::<Vec<&str>>().split_off(1)).output().expect(&format!("path: {}\ncmd: {}\nfull: {}", val, command, &full).to_string());
                    if output.status.success(){
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        println!("{}", stdout);
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        println!("{}", stderr);
                    }
                },
                Err(e)=> {
                    println!("{}", e); //command not found
                },
            }
        }
    }
}

fn findbin(bin:&String, path: &String)-> Result<String, String>{
    for dir in path.split(":"){
        match Path::new(dir).read_dir() {
            Ok(val)=>{
                let bins = val.map(|x| x.unwrap().file_name()).map(|x| x.into_string().unwrap()).collect::<Vec<String>>();
                //println!("{}", bins.join(", "));
                if bins.contains(bin){
                    let metadata = fs::metadata(format!("{}/{}", &dir, bin)).unwrap();
                    let permissions = metadata.permissions();
                    if permissions.mode() & 0o111 == 0{continue;}
                    return Ok(dir.to_string());
                }
            },
            Err(_e)=>{} //nix shenanigans, some directories dont actually exist
        }
    }
    return Err(format!("{}: not found", bin));
}
