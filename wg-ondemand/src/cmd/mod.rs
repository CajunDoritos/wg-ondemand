pub fn parse_arguments(args: &Vec<String>) -> Vec<[String; 2]> {
    let mut parsed_args: Vec<[String; 2]> = Vec::new();
    
    parsed_args.insert(0, [String::from(""), String::from("")]);

    for arg in args {
        println!("{}", arg);
    }

    parsed_args
}