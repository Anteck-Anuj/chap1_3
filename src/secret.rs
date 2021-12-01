use std::env;

pub fn set_info() {
    println!("****************Welcome Goldy *********************");
    env::set_var("pass", "123");
}