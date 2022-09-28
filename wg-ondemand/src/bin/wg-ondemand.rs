extern crate wg_ondemand;

use std::env;
use networking::wifi::*;
use wg_ondemand::cmd::parse_arguments;

fn main() {
    let args: Vec<String> = env::args().collect();
    let _parsed_args: Vec<[String; 2]> = parse_arguments(&args);

    println!("{}", get_ssid());
    println!("Hello, world!");
}