use std::process::{Child, Command};

pub fn open(executable: &str, file: &str, args: Option<Vec<String>>) -> std::io::Result<Child> {
    Command::new(executable)
        .arg(file)
        .args(args.unwrap_or_default())
        .spawn()
}
