use colored::Colorize;
use op::deal_filename;
use std::{
    env,
    path::Path,
    process::{exit, Command},
};

fn main() {
    // 0. get env args
    let mut args: Vec<String> = env::args().collect();
    op::deal_pipe(&mut args);

    // 1. convert args[1] to Path
    let path_str = op::deal_kinds_of_path(args[1].clone());
    let path = Path::new(&path_str);
    if !path.exists() {
        eprintln!("{}: the directory does not exist", "[op error]".red());
        exit(1);
    };

    // 2. parse path to parent and file_name
    let parent = path.parent().unwrap();
    let file_name_raw = path.file_name().unwrap().to_str().unwrap();
    let file_name = deal_filename(file_name_raw);

    // 3. cd parent path
    env::set_current_dir(parent).unwrap();

    // 4. execute open (using exeplorer.exe by default) cmd
    let config_path = format!("{}/.config/op/config.toml", env::var("HOME").unwrap());
    let cmd = op::autocmd(path, &config_path);
    let cmd_open = format!("{cmd} {file_name}");

    Command::new("sh").arg("-c").arg(cmd_open).output().unwrap();
}
