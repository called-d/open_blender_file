#[cfg(target_os = "windows")]
use lnk::ShellLink;
use std::process::{Child, Command};

#[cfg(target_os = "windows")]
fn normalize_link(executable: &str) -> (String, Option<Vec<String>>) {
    if !executable.ends_with(".lnk") {
        return (executable.to_string(), None);
    }

    let shortcut = ShellLink::open(executable).unwrap();
    let link_info = &shortcut.link_info().as_ref().unwrap();
    let local_base_path = &link_info.local_base_path().as_ref().unwrap();
    let args = &shortcut.arguments().as_ref().map(|x| vec![x.into()]);
    println!("{:#?}", shortcut);

    return (local_base_path.to_string(), args.clone());
}
#[cfg(not(target_os = "windows"))]
fn normalize_link(executable: &str) -> (String, Option<Vec<String>>) {
    (executable.to_string(), None)
}

pub fn open(executable: &str, file: &str, args: Option<Vec<String>>) -> std::io::Result<Child> {
    let (executable, link_args) = normalize_link(executable);
    Command::new(executable)
        .args(link_args.unwrap_or_default())
        .arg(file)
        .args(args.unwrap_or_default())
        .spawn()
}
