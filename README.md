# op

open file in wsl using explorer.exe in windows.

Here is its minimal frame:

```rust
use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // 0. get env args
    let args: Vec<String> = env::args().collect();

    // 1. convert args[1] to Path
    let path = Path::new(&args[1]);

    // 2. parse path to parent and file_name
    let parent = path.parent().unwrap();
    let file_name = path.file_name().unwrap().to_str().unwrap();

    // 3. cd parent path
    env::set_current_dir(parent).unwrap();

    // 4. execute open (using exeplorer.exe) cmd
    let cmd_open = format!("explorer.exe {file_name}");
    Command::new("sh").arg("-c").arg(cmd_open).output().unwrap();
}
```
