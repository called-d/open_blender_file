#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use getopts::Options;
use std::io::{BufRead, Read};
use std::{env, process};

use crate::gui::open_ui;

mod config;
mod exec;
mod file_normalizer;
mod gui;
#[cfg(target_os = "windows")]
mod registry;
mod version_checker;

fn print_usage(program: &str, opts: &Options) {
    let program = std::path::Path::new(program);
    let program = program.file_name().unwrap().to_str().unwrap();
    let brief = format!(
        "Usage: {} <FILE> [options] [\"--\" [extra args for blender.exe]]",
        program
    );
    println!("{}", opts.usage(&brief));
}

/// Split free arguments by "--" to pass extra args to Blender.
/// # Example
/// ```no-run
/// extra_args(["a", "--", "b", "c"])
/// // -> (["a"], Some(["b", "c"]))
/// ```
fn extra_args(free: &Vec<String>) -> (Vec<String>, Option<Vec<String>>) {
    let mut iter = free.splitn(2, |arg| arg == "--");
    (
        iter.next().map_or(vec![], |free| free.to_vec()),
        iter.next().map_or(None, |extra| Some(extra.to_vec())),
    )
}
#[test]
fn split_args() {
    macro_rules! args { ($($x:expr),*) => (vec![$($x.to_string()),*]); }
    assert_eq!(extra_args(&Vec::<String>::new()), (args![], None));
    assert_eq!(extra_args(&args!["1"]), (args!["1"], None));
    assert_eq!(extra_args(&args!["--"]), (args![], Some(args![])));
    assert_eq!(
        extra_args(&args!["a", "--", "b", "c"]),
        (args!["a"], Some(args!["b", "c"]))
    );
}
fn find_executable(version: &str) -> Option<String> {
    let path = match &version[0..1] {
        "2" => format!(
            "C:\\Program Files\\Blender Foundation\\Blender {}\\blender.exe",
            version
        ),
        _ => format!(
            "C:\\Program Files\\Blender Foundation\\Blender {}\\blender-launcher.exe",
            version
        ),
    };
    if std::path::Path::new(&path).exists() {
        Some(path)
    } else {
        None
    }
}

#[cfg(target_os = "windows")]
#[link(name = "kernel32")]
extern "system" {
    fn GetConsoleOutputCP() -> u32;
    fn AllocConsole() -> i32;
    fn GetConsoleProcessList(list: *mut u32, count: u32) -> u32;
}
fn enable_console() {
    #[cfg(target_os = "windows")]
    {
        let has_console = unsafe { GetConsoleOutputCP() != 0 };
        if !has_console {
            // Try to allocate a console.
            unsafe { AllocConsole() };
        }
    }
}
fn wait_enter() {
    #[cfg(target_os = "windows")]
    {
        let mut list = [0_u32];
        let count = unsafe { GetConsoleProcessList(list.as_mut_ptr(), 1) };
        if count != 1 {
            // no console
            return;
        }
        println!("Press Enter to exit");
        std::io::stdin().bytes().next();
    }
}

fn get_version_from_file(input: &str) -> version_checker::BlenderVersion {
    let mut reader = file_normalizer::open_buf_reader(&input);
    let mut magic = vec![0u8; 4];
    reader.read_exact(&mut magic).unwrap();
    let mut reader = file_normalizer::normalize_compressed(&magic, reader).unwrap();

    let mut buffer = vec![0u8; 32];
    reader.read_until(0, &mut buffer).unwrap();
    let version_str = std::str::from_utf8(&buffer).unwrap().trim_matches('\0');
    version_checker::get_version(version_str).unwrap()
}

fn is_background_mode(args: &Vec<String>) -> bool {
    let (args, _extra) = extra_args(args);
    args.iter().any(|x| x == "-b" || x == "--background")
}
fn pass_throuh_to_blender(args: Vec<String>) -> std::process::ExitStatus {
    let input = args
        .iter()
        .find(|x| x.ends_with(".blend"))
        .expect("args must have .blend file");
    let version = get_version_from_file(&input);
    let settings = config::load().unwrap();

    let executable = settings
        .get_executable(&version.version)
        .or_else(|| find_executable(&version.version))
        .expect("blender executable should be found.");

    // skip args[0] is open_blender_file.exe
    exec::exec(&executable, args[1..].to_vec())
        .unwrap()
        .wait()
        .unwrap()
}
fn main() {
    let args: Vec<String> = env::args().collect();

    // When arguments has '-b' / '--background',
    // it probably means that Unity is trying to import a .blend file
    if is_background_mode(&args) {
        let status = pass_throuh_to_blender(args);
        process::exit(if status.success() { 0 } else { -1 });
    }
    let program = args[0].clone();

    let mut opts = Options::new();
    if cfg!(target_os = "windows") {
        opts.optflag(
            "",
            "set-icon",
            "set icon (requires Administrator to edit registry).",
        );
    }
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("p", "print-version", "print version and exit.");
    opts.optflag("", "dry-run", "print found blender executable and exit.");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    if matches.opt_present("h") {
        enable_console();
        print_usage(&program, &opts);
        wait_enter();
        process::exit(0);
    }

    #[cfg(target_os = "windows")]
    if matches.opt_present("set-icon") {
        registry::set_icon().unwrap();
        process::exit(0);
    }

    let (matches_free, extra_args) = extra_args(&matches.free);
    let input = if matches_free.len() == 1 {
        matches_free[0].clone()
    } else {
        enable_console();
        if matches_free.len() > 1 {
            eprintln!("extraneous arguments {:?}", &matches_free[1..]);
        }
        if matches_free.len() == 0 {
            print_usage(&program, &opts);
        }
        wait_enter();
        process::exit(-1);
    };
    dbg!(&input);
    let version = get_version_from_file(&input);

    dbg!(&version);
    if matches.opt_present("print-version") {
        enable_console();
        println!("{}", &version.version);
        wait_enter();
        process::exit(0);
    }

    let settings = config::load().unwrap();

    let executable = settings.get_executable(&version.version);
    if matches.opt_present("dry-run") {
        enable_console();
        println!("Versoin: {}", &version.version);
        println!(
            "Blender executable: {}",
            &executable.unwrap_or("missing".to_string())
        );
        wait_enter();
        process::exit(0);
    }
    let executable = executable.or(find_executable(&version.version));
    if let Some(executable) = executable {
        exec::open(&executable, &input, extra_args)
            .unwrap()
            .wait()
            .unwrap();
        process::exit(0);
    }
    open_ui(&version, &input).unwrap();
    process::exit(-1);
}
