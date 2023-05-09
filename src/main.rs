use colored::Colorize;
use std::{
    env,
    path::Path,
    process::{exit, Command},
};

use op::Config;

fn run() -> Option<()> {
    // 0. get env args
    let mut args: Vec<String> = env::args().collect();
    op::deal_args(&mut args);

    // 1. convert args[1] to Path
    let file_path_str = op::deal_kinds_of_path(args[1].clone())?;
    let file_path = Path::new(&file_path_str);
    if !file_path.exists() {
        eprintln!("{}: the directory does not exist", "[op error]".red());
        exit(1);
    };

    // 2. parse path to parent and file_name
    let parent = file_path.parent().unwrap_or_else(|| {
        eprintln!("{}: cann't parse parent directory", "[op error]".red());
        exit(1);
    });
    let file_name_raw = file_path
        .file_name()
        .unwrap_or_else(|| {
            eprintln!("{}: cann't parse file name", "[op error]".red());
            exit(1);
        })
        .to_str()?;
    let file_name = op::deal_filename(file_name_raw);

    // 3. cd parent path
    env::set_current_dir(parent).unwrap_or_else(|err| {
        eprintln!("{}: {}", "[op error]".red(), err);
        exit(1);
    });

    // 4. execute open (using explorer.exe by default) cmd
    let config_path = format!(
        "{}/.config/op/config.toml",
        env::var("HOME").unwrap_or_else(|err| {
            eprintln!("{}: {}", "[op error]".red(), err);
            exit(1);
        })
    );
    let cmd = if Path::new(&config_path).exists() & Path::new(file_name_raw).is_file() {
        let file_type = file_path.extension()?.to_str()?;
        Config::new(&config_path).auto_select_cmd(&file_type)
    } else {
        "explorer.exe".to_owned()
    };
    let cmd_final = format!("{cmd} {file_name}");

    Command::new("sh")
        .arg("-c")
        .arg(cmd_final)
        .output()
        .unwrap_or_else(|err| {
            eprintln!("{}: {}", "[op error]".red(), err);
            exit(1);
        });
    Some(())
}

fn main() {
    let option = run();
    if let None = option {
        eprintln!(
            "{}: have not handle this",
            "[op error]".red()
        );
        exit(1);
    }
}
