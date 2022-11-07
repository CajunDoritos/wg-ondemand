extern crate wg_ondemand;

use std::env;
use networking::wifi::*;
use wg_ondemand::cmd;

#[derive(Debug)]
enum Flags {
    Help,
    ASSID(String),  //Add SSID
    RSSID(String),  //Remove SSID
    Inclusive,
    Exclusive
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
            }  else if arg[0] == "a" || arg[0] == "add-ssid" {
                flags.push(Flags::ASSID(arg[1].clone()));
            } else if arg[0] == "r" || arg[0] == "remove-ssid" {
                flags.push(Flags::RSSID(arg[1].clone()));
            } else if arg[0] == "e" || arg[0] == "exclusive" {
                flags.push(Flags::Exclusive);
            } else if arg[0] == "i" || arg[0] == "inclusive" {
                flags.push(Flags::Inclusive);
            } else if arg[0] != "" {
                println!("-{} is an invalid option\n", arg[0]);
                println!("{}", cmd::HELP);
                std::process::exit(1);
            }
        }
    }

    if flags.is_empty() {
        flags.push(Flags::Help);
    }

    println!("{:?}", &flags);

    for flag in flags {
        match flag {
            Flags::Help => println!("{}", cmd::HELP),
            Flags::ASSID(ssid) => println!("{}", ssid),
            Flags::RSSID(ssid) => println!("{}", ssid),
            _ => println!("Feature not implemented yet"),
        }
    }

    println!("{}", get_ssid());
}