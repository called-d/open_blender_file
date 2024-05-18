use std::{env, process};
use getopts::Options;
use std::io::BufRead;

mod file_normalizer;
mod version_checker;

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    println!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("p", "print-version", "print version and exit.");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, &opts);
        process::exit(0);
    }

    let input = if matches.free.len() == 1 {
        matches.free[0].clone()
    } else {
        if matches.free.len() > 1 {
            eprintln!("extraneous arguments {:?}", &matches.free[1..]);
        }
        if matches.free.len() == 0 {
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
        println!("{}", version.version);
        process::exit(0);
    }

    process::exit(0);
}
