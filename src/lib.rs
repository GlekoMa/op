use colored::Colorize;
use is_terminal::IsTerminal as _;
use serde::Deserialize;
use std::{
    collections::HashMap,
    env, fs,
    io::{stdin, Read},
    path::Path,
};

#[derive(Deserialize)]
struct Config {
    custom: Vec<Custom>,
}

#[derive(Deserialize)]
struct Custom {
    command: String,
    subcommands: Vec<String>,
    filetype: Vec<String>,
}

impl Config {
    fn new(config_path: &str) -> Config {
        let contents = fs::read_to_string(config_path).unwrap();
        toml::from_str(&contents).unwrap()
    }
}

fn parse(config_path: &str) -> HashMap<String, Vec<String>> {
    let config = Config::new(config_path);
    let mut custom_commands = HashMap::new();
    for custom in &config.custom {
        let command = &custom.command;
        let subcommands = custom.subcommands.join(" --");
        let complete_command = if subcommands == "" {
            command.to_string()
        } else {
            format!("{command} --{subcommands}")
        };
        custom_commands.insert(complete_command, custom.filetype.clone());
    }
    custom_commands
}

pub fn autocmd(file_path: &Path, config_path: &str) -> String {
    let mut cmd = String::from("explorer.exe");
    if Path::new(config_path).exists() {
        let config = parse(config_path);
        if file_path.is_file() {
            for (key, value) in &config {
                let extension = file_path.extension().unwrap().to_str().unwrap().to_owned();
                if value.contains(&extension) {
                    cmd = key.to_string();
                }
            }
        }
    }
    cmd
}

pub fn deal_pipe(args: &mut Vec<String>) {
    if stdin().is_terminal() {
        if args.len() == 1 {
            eprintln!("{}: need path argument", "[op error]".red());
            std::process::exit(1);
        }
    // support pipe
    } else {
        let mut buffer = String::new();
        stdin().read_to_string(&mut buffer).unwrap();
        let arg_path: Vec<&str> = buffer.split("\n").collect();
        // if stdin is not single (not recommend), use first (s.g. ls)
        args.insert(1, arg_path[0].to_owned());
    }
}

pub fn deal_kinds_of_path(mut p: String) -> String {
    // s.g. type "op code/" equals "op code"
    let p_last_char = p.chars().last().unwrap();
    if p_last_char == '/' {
        p.pop();
    }

    if p.contains('~') {
        p.replace('~', &env::var("HOME").unwrap()).to_owned()
    } else if p == "." {
        env::current_dir().unwrap().to_str().unwrap().to_owned()
    } else if !p.contains('/') {
        // s.g. type 'op code' equals 'op ./code'
        format!("./{p}")
    } else {
        p.to_owned()
    }
}
