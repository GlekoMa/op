use colored::Colorize;
use is_terminal::IsTerminal as _;
use serde::Deserialize;
use std::{
    collections::HashMap,
    env, fs,
    io::{stdin, Read},
    process,
};

#[derive(Deserialize)]
pub struct Config {
    custom: Vec<Custom>,
}

#[derive(Deserialize)]
struct Custom {
    command: String,
    subcommands: Vec<String>,
    filetypes: Vec<String>,
}

impl Config {
    pub fn new(config_path: &str) -> Config {
        let contents = fs::read_to_string(config_path).unwrap_or_else(|err| {
            eprintln!("{}: {}", "[op error]".red(), err);
            process::exit(1);
        });
        toml::from_str(&contents).unwrap_or_else(|err| {
            eprintln!("{}: {}", "[op error]".red(), err);
            process::exit(1);
        })
    }
    pub fn parse(self) -> HashMap<String, Vec<String>> {
        let mut custom_cmds = HashMap::new();
        for entry in &self.custom {
            let cmd = &entry.command;
            let subcmds = entry.subcommands.join(" --");
            let cmd_complete = if subcmds == "" {
                cmd.to_string()
            } else {
                format!("{cmd} --{subcmds}")
            };
            custom_cmds.insert(cmd_complete, entry.filetypes.clone());
        }
        custom_cmds
    }
    pub fn auto_select_cmd(self, file_type: &str) -> String {
        let mut cmd = String::from("explorer.exe");
        let custom_cmds = self.parse();

        let mut flag = 0;
        for (cmd_complete, file_types) in &custom_cmds {
            if file_types.contains(&file_type.to_owned()) {
                if flag == 1 {
                    eprintln!(
                        "{}: filetype matches multiple commands in config.toml",
                        "[op error]".red()
                    );
                    process::exit(1);
                } else {
                    cmd = cmd_complete.to_owned();
                    flag = 1;
                }
            }
        }
        cmd
    }
}

pub fn deal_args(args: &mut Vec<String>) {
    if stdin().is_terminal() {
        if args.len() == 1 {
            eprintln!("{}: need path argument", "[op error]".red());
            process::exit(1);
        }
    } else {
        // if the args are passed through the pipe, args.len() will be 1
        // thus need to read from stdin and insert it to original args as path
        let mut buffer = String::new();
        stdin().read_to_string(&mut buffer).unwrap_or_else(|err| {
            eprintln!("{}: {}", "[op error]".red(), err);
            process::exit(1);
        });
        let args_real: Vec<&str> = buffer.split("\n").collect();
        if args_real.len() > 2 {
            eprintln!("{}: don't support multiple path", "[op error]".red());
            process::exit(1);
        } else {
            args.insert(1, args_real[0].to_owned());
        }
    }
}

pub fn deal_kinds_of_path(mut path: String) -> Option<String> {
    // s.g. type "op code/" equals "op code"
    let path_last_char = path.chars().last().unwrap_or_else(|| {
        eprintln!("{}: need path argument", "[op error]".red());
        process::exit(1);
    });
    if path_last_char == '/' {
        path.pop();
    }

    let path_done = if path.contains('~') {
        path.replace(
            '~',
            &env::var("HOME").unwrap_or_else(|err| {
                eprintln!("{}: {}", "[op error]".red(), err);
                process::exit(1);
            }),
        )
        .to_owned()
    } else if path == "." {
        env::current_dir()
            .unwrap_or_else(|err| {
                eprintln!("{}: {}", "[op error]".red(), err);
                process::exit(1);
            })
            .to_str()?
            .to_owned()
    } else if !path.contains('/') {
        // s.g. type 'op code' equals 'op ./code'
        format!("./{path}")
    } else {
        path.to_owned()
    };
    Some(path_done)
}

// deal with special char (s.g. Donna Donna.mp3)
pub fn deal_filename(file_name_raw: &str) -> String {
    file_name_raw
        .replace("(", r"\(")
        .replace(")", r"\)")
        .split(" ")
        .collect::<Vec<&str>>()
        .join(r"\ ")
}
