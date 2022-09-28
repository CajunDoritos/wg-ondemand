pub fn parse_arguments(args: &Vec<String>) -> Vec<[String; 2]> {
    let mut parsed_args: Vec<[String; 2]> = Vec::new();
    let mut append_at_end: Vec<String> = Vec::new();

    let mut prev_flag: bool = false;
    for arg in args {
        if arg.get(..1).unwrap() == "-" && arg.get(..2).unwrap() != "--" {
            prev_flag = true;
            for i in 1..arg.len() {
                let char = arg.get(i..i+1).unwrap();
                parsed_args.push([String::from(char), String::from("")]);
            }
        } else if arg.get(..2).unwrap() == "--" {
            prev_flag = true;
            parsed_args.push([String::from(arg.get(2..).unwrap()), String::from("")]);
        } else {
            if prev_flag == true {
                let flag: &str = parsed_args[parsed_args.len() - 1][0].as_str();
                let new_arg: [String; 2] = [String::from(flag), arg.clone()];
                parsed_args.insert(parsed_args.len() - 1, new_arg);
                parsed_args.remove(parsed_args.len() - 1);
            } else {
                append_at_end.push(String::from(arg));
            }
            prev_flag = false;
        }
    }

    for a in append_at_end {
        let arg: [String; 2] = [String::from(""), a];
        parsed_args.push(arg);
    }

    parsed_args
}

pub const HELP: &str = 
"Run WireGuard on demand

USAGE: wg-ondemand [OPTIONS] <WIREGUARD_CONFIG>

OPTIONS:
    -h, --help     Display this help menu";