extern crate wg_ondemand;

use std::env;
use networking::wifi::*;
use wg_ondemand::cmd::parse_arguments;

fn main() {
    let args: Vec<String> = env::args().collect();
    let parsed_args: Vec<[String; 2]> = parse_arguments(&args);

    println!("{:?}", parsed_args);
    println!("{}", get_ssid());
}