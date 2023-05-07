use colored::Colorize;
use serde::Deserialize;
use std::{collections::HashMap, env, fs, path::Path, process::exit};

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
        let contents = fs::read_to_string(config_path).unwrap_or_else(|_err| {
            ep("cann't read 'config.toml' path");
            exit(1);
        });
        toml::from_str(&contents).unwrap_or_else(|_err| {
            ep("cann't parse 'config.toml' to str");
            exit(1);
        })
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
                let extension = file_path
                    .extension()
                    .unwrap_or_else(|| {
                        ep("cann't parse `path.extension`");
                        exit(1);
                    })
                    .to_str()
                    .unwrap_or_else(|| {
                        ep("cann't convert extension (Path) to str");
                        exit(1);
                    })
                    .to_owned();
                if value.contains(&extension) {
                    cmd = key.to_string();
                }
            }
        }
    }
    cmd
}

pub fn ep(msg: &str) {
    eprintln!("{}: {}", "[op error]".red(), msg);
}

pub fn deal_kinds_of_path(mut p: String) -> String {
    // when last char is '/', delete it
    // if not, type "op code/" will raise an error
    // because there is no parent path
    let p_last_char = p.chars().last().unwrap_or_else(|| {
        ep("path is empty");
        exit(1);
    });
    if p_last_char == '/' {
        p.pop();
    }

    // detail
    if p.contains('~') {
        // type 'op ~' = type 'op /home/gleko/'
        // because my user name is gleko
        p.replace('~', "/home/gleko").to_owned()
    } else if p == "." {
        // type 'op .', open current dir
        getcwd()
    } else if !p.contains('/') {
        // type 'op code' = type 'op ./code'
        format!("./{p}")
    } else {
        // general situation
        p.to_owned()
    }
}

pub fn getcwd() -> String {
    env::current_dir()
        .unwrap_or_else(|_err| {
            ep("cann't get current_dir");
            exit(1);
        })
        .to_str()
        .unwrap_or_else(|| {
            ep("cann't convert current_dir (Path) to str");
            exit(1);
        })
        .to_owned()
}
