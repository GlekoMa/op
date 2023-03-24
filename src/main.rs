use colored::Colorize;
use std::env;
use std::path::Path;
use std::process::{exit, Command};

fn main() {
    // 0. get env args
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        ep("need one path argument");
        exit(1);
    }

    // 1. convert args[1] to Path
    let path_str = deal_kinds_of_path(args[1].clone());
    let path = Path::new(&path_str);
    // eprintln!("Oops {:?}", path); // This code is for debug.
    if !path.exists() {
        ep("the directory does not exist");
        exit(1);
    };

    // 2. parse path to parent and file_name
    let parent = path.parent().unwrap_or_else(|| {
        ep("parse `path.parent`");
        exit(1);
    });
    let file_name = path
        .file_name()
        .unwrap_or_else(|| {
            ep("parse `path.file_name`");
            exit(1);
        })
        .to_str()
        .unwrap_or_else(|| {
            ep("convert file_name (Path) to str");
            exit(1);
        });

    // 3. cd parent path
    env::set_current_dir(parent).unwrap_or_else(|_err| {
        ep("set dir");
        exit(1);
    });

    // 4. execute open (using exeplorer.exe) cmd
    let cmd_open = format!("explorer.exe {file_name}");
    Command::new("sh")
        .arg("-c")
        .arg(cmd_open)
        .output()
        .unwrap_or_else(|_err| {
            ep("execute process");
            exit(1)
        });

    // *. if the file make people happy, print a happy wish.
    if path.extension().is_none() {
    } else {
        let ext = path
            .extension()
            .unwrap_or_else(|| {
                ep("parse `path.extension`");
                exit(1);
            })
            .to_str()
            .unwrap_or_else(|| {
                ep("convert ext (Path) to str");
                exit(1);
            });
        let happies = vec!["mp4", "mkv", "mp3", "flac", "wav"];
        if happies.iter().any(|e| &ext == e) {
            println!("{}", "Happy day! 😊".blue());
        };
    };
}

fn ep(msg: &str) {
    eprintln!("{}: {}", "error".red(), msg);
}

fn deal_kinds_of_path(mut p: String) -> String {
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

fn getcwd() -> String {
    env::current_dir()
        .unwrap_or_else(|_err| {
            ep("cann't get current_dir");
            exit(1);
        })
        .to_str()
        .unwrap_or_else(|| {
            ep("convert current_dir (Path) to str");
            exit(1);
        })
        .to_owned()
}
