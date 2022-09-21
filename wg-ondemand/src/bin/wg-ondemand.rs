extern crate wg_ondemand;

use networking::wifi::*;

fn main() {
    println!("{}", get_ssid(get_interface()));
    println!("Hello, world!");
}