use rand::prelude::*;
use std::env;
use std::process;

struct Config{
    len: u32
}

impl Config {
    fn build()->Result<Config, &'static str>{
        let args = env::args().collect::<Vec<_>>();
        if args.len() != 2{
            return Err("Invalid number of arguements");
        }
        
        let len = match args[1].parse(){
            Ok(len)=>len,
            Err(_)=>18
        };

        return Ok(Config{len: len});
    }
}

fn main() -> std::io::Result<()>{
    let config = Config::build().unwrap_or_else(|err|{
        eprintln!("Error occured: {}", err);
        eprintln!("Usage: passgen <LENGTH>");
        process::exit(1);
    });

    let len = config.len;
    let password = gen_password(len);
    
    println!("This is your password: {}", password);
    Ok(())
}


fn gen_password(len: u32)->String{
    let string = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*");
    let vs = string.chars().collect::<Vec<char>>();
    let pass_len = len;
    let mut password = String::new();
    for _ in 0..pass_len{
        password.push(*vs.choose(&mut rand::thread_rng()).unwrap());
    }
    password
}
