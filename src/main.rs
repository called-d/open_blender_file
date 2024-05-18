use std::{env, process};
use getopts::Options;
use std::io::BufRead;

mod file_normalizer;
mod version_checker;
mod config;
mod exec;

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    println!("{}", opts.usage(&brief));
}

fn extra_args(free: &Vec<String>) -> (Vec<String>, Option<Vec<String>>) {
    let mut iter = free.splitn(2, |arg| arg == "--");
    (
        iter.next()
            .map_or(vec![], |free| free.to_vec()),
        iter.next()
            .map_or(None, |extra| Some(extra.to_vec()))
    )
}
#[test]
fn split_args() {
    let (a, b) = extra_args(&Vec::<String>::new());
    assert!(a.len() == 0 && b.is_none());
    let (a, b) = extra_args(
        &["1"].iter().map(|s| s.to_string()).collect()
    );
    assert!(a.len() == 1 && b.is_none());
    let (a, b) = extra_args(
        &["--"].iter().map(|s| s.to_string()).collect()
    );
    assert!(a.len() == 0 && b.is_some_and(|x| x.len() == 0));
    let (a, b) = extra_args(
        &["a", "--", "b", "c"].iter().map(|s| s.to_string()).collect()
    );
    assert!(a.len() == 1 && b.is_some_and(|x| x.len() == 2));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("p", "print-version", "print version and exit.");
    opts.optflag("", "dry-run", "print found blender executable and exit.");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, &opts);
        process::exit(0);
    }

    let (matches_free, extra_args) = extra_args(&matches.free);
    let input = if matches_free.len() == 1 {
        matches_free[0].clone()
    } else {
        if matches_free.len() > 1 {
            eprintln!("extraneous arguments {:?}", &matches_free[1..]);
        }
        if matches_free.len() == 0 {
            print_usage(&program, &opts);
        }
        process::exit(-1);
    };
    let version = {
        let mut reader = file_normalizer::open_buf_reader(&input);
        let mut magic = vec![0u8; 4];
        reader.read_exact(&mut magic).unwrap();
        let mut reader = file_normalizer::normalize_compressed(&magic, reader).unwrap();

        let mut buffer = vec![0u8; 32];
        reader.read_until(0, &mut buffer).unwrap();
        let version_str= std::str::from_utf8(&buffer)
            .unwrap()
            .trim_matches('\0');
        version_checker::get_version(version_str).unwrap()
    };

    dbg!(&version);
    if matches.opt_present("print-version") {
        println!("{}", &version.version);
        process::exit(0);
    }

    let settings = config::load().unwrap();

    let executable = settings.get_executable(&version.version);
    if matches.opt_present("dry-run") {
        println!("Versoin: {}", &version.version);
        println!("Blender executable: {}", &executable.unwrap_or("missing".to_string()));
        process::exit(0);
    } else if let Some(executable) = executable {
        exec::open(&executable, &input, extra_args).unwrap().wait().unwrap();
    }

    process::exit(0);
}
