use is_terminal::IsTerminal as _;
use op::ep;
use std::{
    env,
    io::{stdin, Read},
    path::Path,
    process::{exit, Command},
};

fn main() {
    // 0. get env args
    let mut args: Vec<String> = env::args().collect();
    if stdin().is_terminal() {
        if args.len() == 1 {
            ep("need one path argument");
            exit(1);
        }
    // support pipe
    } else {
        let mut buffer = String::new();
        stdin().read_to_string(&mut buffer).unwrap_or_else(|_err| {
            ep("cann't read stdin to buffer");
            exit(1);
        });
        let arg_path: Vec<&str> = buffer.split("\n").collect();
        // if stdin is not single (not recommend), use first (s.g. ls)
        args.insert(1, arg_path[0].to_owned());
    }

    // 1. convert args[1] to Path
    let path_str = op::deal_kinds_of_path(args[1].clone());
    let path = Path::new(&path_str);
    if !path.exists() {
        ep("the directory does not exist");
        exit(1);
    };

    // 2. parse path to parent and file_name
    let parent = path.parent().unwrap_or_else(|| {
        ep("cann't parse `path.parent`");
        exit(1);
    });
    let file_name = path
        .file_name()
        .unwrap_or_else(|| {
            ep("cann't parse `path.file_name`");
            exit(1);
        })
        .to_str()
        .unwrap_or_else(|| {
            ep("cann't convert file_name (Path) to str");
            exit(1);
        });

    // 3. cd parent path
    env::set_current_dir(parent).unwrap_or_else(|_err| {
        ep("cann't cd parent path");
        exit(1);
    });

    // 4. execute open (using exeplorer.exe by default) cmd
    let config_path = format!(
        "{}/.config/op/config.toml",
        env::var("HOME").unwrap_or_else(|_err| {
            ep("cann't get 'HOME' env var");
            exit(1);
        })
    );
    let cmd = op::autocmd(path, &config_path);
    let cmd_open = format!("{cmd} {file_name}");

    Command::new("sh")
        .arg("-c")
        .arg(cmd_open)
        .output()
        .unwrap_or_else(|_err| {
            ep("cann't execute process");
            exit(1)
        });
}
