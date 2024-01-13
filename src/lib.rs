use std::fs;

mod line_matcher;

pub struct Config {
    file_name: String,
    query: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Missing arguments");
        }

        let query = args[1].clone();
        let file_name = args[2].clone();

        Ok(Config { file_name, query })
    }
}

pub fn run(config: Config) {
    let content = read_file(&config.file_name);

    let matching_lines = line_matcher::find_matching_lines(&content, &config.query);
    if matching_lines.len() == 0 {
        println!("Nothing found");
    }
    print_lines(&matching_lines);
}

fn print_lines(lines: &[&str]) -> () {
    for line in lines {
        println!("{}", line);
    }
}

fn read_file(file_name: &str) -> String {
    return fs::read_to_string(file_name).expect("Couldn't read file");
}
