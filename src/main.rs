//use std::fmt::format;
use std::io::{self, Write};
use std::collections::HashMap;
use std::{env, fs};
use std::path::Path;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

fn main()  {
  //let mut builtins: HashMap<String, Box<dyn Fn(String)-> i32>> = HashMap::from([

  let builtins: HashMap<String, Box<dyn Fn(String, &Vec<String>, &mut String, &Vec<String>)-> i32>> = HashMap::from([
    (
      "echo".to_string(),
      Box::new(
        move |mut input: String, _commands: &Vec<String>, _path: &mut String, params: &Vec<String>|{
          println!("{}", params.join(" "));
          return 0;
        }
      ) as Box<dyn Fn(String, &Vec<String>, &mut String, &Vec<String>)->i32>
    ),
    (
      "exit".to_string(),
      Box::new(
        move |_input: String, _commands: &Vec<String>, _path: &mut String, _params: &Vec<String>|{
          return 1;
        }
      )
    ),

    (
      "type".to_string(),
      Box::new(
        move |input: String, commands: &Vec<String>, _path: &mut String, _params: &Vec<String>|{
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
    ),

    (
      "pwd".to_string(),
      Box::new(
        move |_input: String, _commands: &Vec<String>, path: &mut String, _params: &Vec<String>| {
          println!("{}", path);
          0
        }
      )
    ),

    (
      "ls".to_string(),
      Box::new(
        move |_input:String,  _commands: &Vec<String>, path: &mut String, params: &Vec<String>| {
          let newpath = path.to_owned()+"/"+&params.join("/");
          let entries = Path::new(&newpath).read_dir().expect(&format!("bad path: ---{}---", &newpath))
            .map(|x| x.unwrap().file_name().into_string().unwrap()).collect::<Vec<String>>();
          println!("{}", entries.join("\t"));
          0
        } 
      )
    ),

    (
      "cd".to_string(),
      Box::new(
        move |input: String, _commands: &Vec<String>, path: &mut String, params: &Vec<String>|{
          let dir = input.split(" ").nth(1).unwrap();
          if dir.chars().into_iter().nth(0).unwrap() == '/'{ // absolute path
            if Path::new(dir).exists(){
              *path = dir.to_owned();
              return 0;
            } else {
              println!("cd: {}: No such file or directory", dir);
              return -1;
            }
          } else { // relative path
            let ownedpath = path.to_owned();
            let mut vectorised = ownedpath.split("/").collect::<Vec<&str>>();
            //let cdpath = input.split(" ").nth(1).unwrap().split("/").into_iter();

            for directory in params{
              if directory == ".." {
                if let Some(_last) = vectorised.pop(){
                  *path = vectorised.join("/");
                } else {
                  println!("gebasz van helo");
                  return -1;
                }
              } else if directory == "."{//do nothing, this is the current one
              } else if directory == "~" { //home 
                match env::var("HOME") {
                  Ok(val)=>{
                    *path = val.to_string();
                  },
                  Err(_e)=>{todo!("no home in env vars")},
                }
              } else if directory != "" { // change to a local directory
                *path = path.to_owned()+"/"+directory;
              } else {}//do nothing
            }
            return 0;

          } 
        }
      )
    )
  ]);

  let commands: Vec<String> = builtins.keys().map(|x| x.to_string()).collect();
  let mut pwd: String = String::from_utf8_lossy(&Command::new("pwd").output().unwrap().stdout).to_string().trim().to_string();
  loop{
    print!("$ ");
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    input = input.trim().to_string();

    #[allow(unused)]
    let mut params: Vec<String> = vec![];

    let mut counter = 0;
    params = input.split("\'").map(|x|
      {
        counter+=1;
        if counter%2==1{
          if x==""{
            return vec![""];
          }
          return x.trim().split(" ").collect();
        }
        return vec![x.trim()];
      }
    ).flatten().filter(|x|x!=&"").map(|x| x.trim().to_owned()).collect();

    let command: String = params[0].to_owned();
    params.remove(0);
    let path = env::var("PATH").unwrap();
    if builtins.contains_key(&command){ // builtins
      if builtins[&command](input, &commands, &mut pwd, &params) == 1 {
        return;
      }
    }

    else{ // not builtin command
      match findbin(&command.to_string(), &path) {
        Ok(val)=>{
          let full = val.to_string()+"/"+&command;
          let output = Command::new(&command)
            .args(params).output().expect(&format!("path: {}\ncmd: {}\nfull: {}", val, &command, &full).to_string());
          if output.status.success(){
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.len()>0{
              println!("{}", stdout.trim());
            }
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
