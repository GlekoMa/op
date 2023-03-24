# op

> **Note**: 
> 
> If you got an error every first time 
> running `op ` + `Tab` to get a completion in a new terminal,
> check if there exists a completion script also named 'op'
> in your shell completion script files.
> 
> For instance, in fish, 
> if there is a /usr/share/fish/completions/op.fish before, 
> you can fixed it by two ways: 
> 
> 1. Rename 'op' (not by alias but by rebuilding).
> 
> 2. Write an empty file to ~/.config/fish/completions/op.fish so that fish will load it instead of /usr/share/fish/completions/op.fish.
> 
> More discussions about this error are in [here](https://stackoverflow.com/questions/75819833/cli-tool-written-in-rust-occurs-error-when-using-completion-of-fish-at-the-first).

open file in wsl using explorer.exe in windows.

Here is its minimal frame (src/main.rs):

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
