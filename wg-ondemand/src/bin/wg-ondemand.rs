extern crate wg_ondemand;

use std::env;
use networking::wifi::*;
use wg_ondemand::cmd;

#[derive(Debug)]
enum Flags {
    Help,
    SSID(String),
}

struct OneTimeFlags {
    help: bool,
}

fn main() {
    let mut flags: Vec<Flags> = Vec::new();

    {
        let mut parsed_args: Vec<[String; 2]>;
        {
            let args: Vec<String> = env::args().collect(); //args are not needed in memory after parsing
            parsed_args = cmd::parse_arguments(&args);
        }
    
        parsed_args.sort();
        parsed_args.dedup();

        let mut otf: OneTimeFlags = OneTimeFlags { help: false };
        for arg in &parsed_args {
            if (arg[0] == "h" || arg[0] == "help") && !otf.help {
                otf.help = true;
                flags.push(Flags::Help);
            }
            if arg[0] == "s" || arg[0] == "ssid" {
                flags.push(Flags::SSID(arg[1].clone()));
            }
        }
    }

    if flags.is_empty() {
        flags.push(Flags::Help);
    }

    println!("{:?}", flags);
    println!("{}", get_ssid());
}