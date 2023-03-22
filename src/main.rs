use colored::Colorize;
use std::env;
use std::path::Path;
use std::process::{exit, Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    // convert args[1] to Path
    let path_str = {
        let t = &args[1].replace('~', "/home/gleko");
        if t == "." {
            getcwd()
        } else if !t.contains('/') {
            format!("./{t}")
        } else {
            t.to_owned()
        }
    };
    let path = Path::new(&path_str);
    println!("Oops {:?}", path); // This code is for debug.
    if !path.exists() {
        ep("the directory does not exist");
        exit(1);
    };

    // parse path to parent and file_name
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

    // cd parent path
    env::set_current_dir(parent).unwrap_or_else(|_err| {
        ep("set dir");
        exit(1);
    });

    // execute open (using exeplorer.exe) cmd
    let cmd_open = format!("explorer.exe {file_name}");
    Command::new("sh")
        .arg("-c")
        .arg(cmd_open)
        .output()
        .unwrap_or_else(|_err| {
            ep("execute process");
            exit(1)
        });

    // if the file make people happy, print a happy wish.
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
