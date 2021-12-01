// use std::net::IpAddr;
use std::env;
use std::io;
mod secret;
use crate::secret::set_info;

fn main() {
    // env::var("name");
    set_info();

    println!("Enter Goldy Password:");
    let mut st_pass = String::new();  
    io::stdin()
        .read_line(&mut st_pass)
        .expect("Invalid value");

    let st_name = st_pass.trim();

    let res = env::var("pass");
    match res {
        Ok(x) => {
            if x == st_name {
                println!("You are LoggedIn Successfully!");
            } 
            else {
                println!("Wrong Password !!");
            }
        },
        Err(error) => println!("Error - {}", error)
    }    
}

