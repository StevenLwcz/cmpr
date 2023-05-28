// TODO
// automated testing
// improve char output - use unicode char for control characters
// add -e ebcdic mode
// todo stdin or maybe just not bother with
// handle invalid skip parameter

use clap::{App, Arg, ArgMatches};
use std::cmp::min;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;

const DIFF_OK: i32 = 0;
const DIFF_FAIL: i32 = 1;
const DIFF_FILE_NOT_FOUND: i32 = 2;
const DIFF_FILE_LEN_DIFF: i32 = 3;

enum Mode {
    Hex,
    Char,
    Byte,
    Single,
}

struct CmpOptions {
    file1: PathBuf,
    file2: Option<PathBuf>,
    mode: Mode,
    skip: usize,
}

// todo catch non numeric value for skip
impl CmpOptions {
    fn new(matches: ArgMatches) -> Self {
        Self {
            skip: matches
                .value_of("skip")
                .unwrap_or("0")
                .parse::<usize>()
                .unwrap_or(0),
            mode: if matches.is_present("hex") {
                Mode::Hex
            } else if matches.is_present("char") {
                Mode::Char
            } else if matches.is_present("list") {
                Mode::Byte
            } else {
                Mode::Single
            },
            file1: matches.value_of("file1").unwrap().into(),
            file2: matches.value_of("file2").map(PathBuf::from),
        }
    }
}

fn main() {
    let mut status = DIFF_OK;
    let options = parse_command_line();
    let skip = options.skip;
    let file_name = options.file1;
    let file1 = match File::open(&file_name) {
        Ok(r) => r,
        Err(err) => {
            eprintln!("Can't open file {} - {}", file_name.to_string_lossy(), err);
            std::process::exit(DIFF_FILE_NOT_FOUND);
        }
    };
    let file_name2 = options.file2.unwrap();
    let file2 = match File::open(&file_name2) {
        Ok(r) => r,
        Err(err) => {
            eprintln!("Can't open file {} - {}", file_name2.to_string_lossy(), err);
            std::process::exit(DIFF_FILE_NOT_FOUND);
        }
    };
    let len1 = fs::metadata(&file_name).unwrap().len();
    let len2 = fs::metadata(&file_name2).unwrap().len();
    let reader1 = BufReader::new(file1);
    let reader2 = BufReader::new(file2);

    let width = (min(len1, len2) as f32).log10().floor() as usize + 1;
    let mut addr = skip;
    for c in reader1.bytes().skip(skip).zip(reader2.bytes().skip(skip)) {
        addr += 1;
        let (a, b) = c;
        let x = a.unwrap();
        let y = b.unwrap();
        if x != y {
            status = DIFF_FAIL;
            match options.mode {
                Mode::Single => {
                    println!(
                        "{} {} differ at byte {}: {:02X} {:02X}",
                        &file_name.to_string_lossy(),
                        &file_name2.to_string_lossy(),
                        addr,
                        x,
                        y
                    );
                    break;
                }
                Mode::Byte => println!("{:width$} {:3} {:3}", addr, x, y),
                Mode::Hex => println!("{:width$} {:02X} {:02X}", addr, x, y),
                Mode::Char => println!("{:width$} {:1} {:1}", addr, to_char(x), to_char(y)),
            }
        }
    }
    match options.mode {
        Mode::Single => (),
        _ => {
            if addr < len1 as usize {
                println!("EOF on {} at byte {}", file_name2.to_string_lossy(), len2);
                status = DIFF_FILE_LEN_DIFF;
            } else if addr < len2 as usize {
                println!("EOF on {} at byte {}", file_name.to_string_lossy(), len1);
                status = DIFF_FILE_LEN_DIFF;
            }
        }
    }
    std::process::exit(status);
}

// todo add -i ignore
// normal cmp can read from std with ctrl-d
fn parse_command_line() -> CmpOptions {
    let matches = App::new("cmpr")
        .version("0.1.0")
        .author("Steven Lalewicz 05-2023")
        .about("cmpr with ascii, byte or hex output")
        .arg(
            Arg::with_name("list")
                .help("list all differences shown in bytes")
                .short("l")
                .long("list"),
        )
        .arg(
            Arg::with_name("hex")
                .help("list all differences shown in hex")
                .short("h")
                .long("hex"),
        )
        .arg(
            Arg::with_name("char")
                .help("list all differences shown as characters")
                .short("c")
                .long("char"),
        )
        .arg(
            Arg::with_name("skip")
                .help("skip first n bytes")
                .short("i")
                .long("ignore")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file1")
                .help("first file to compare")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("file2")
                .help("second file to compare")
                .index(2),
        )
        .get_matches();

    CmpOptions::new(matches)
}

fn to_char(b: u8) -> char {
    if b.is_ascii_graphic() {
        b as char
    } else {
        '\u{25AF}'
    }
}
