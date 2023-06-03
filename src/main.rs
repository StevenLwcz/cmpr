// TODO
// add -e ebcdic mode ?
// stdin or maybe just not bother

use clap::{App, Arg, ArgMatches};
use std::cmp::min;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::PathBuf;

const DIFF_OK: i32 = 0;
const DIFF_FAIL: i32 = 1;
const DIFF_FILE_NOT_FOUND: i32 = 2;
const DIFF_FILE_LEN_DIFF: i32 = 3;
const DIFF_INVALID_ARGUMENT: i32 = 4;

type Data = (usize, u8, u8);

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
            skip: match matches.value_of("skip").unwrap_or("0").parse::<usize>() {
               Ok(s) => s,
               Err(err) => {
                   eprintln!("cmpr: invalid option for -i/--ignore - {}", err);
                   std::process::exit(DIFF_INVALID_ARGUMENT);
               }
            },
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
    if !file_name.exists() {
        eprintln!("cmpr: File {} does not exist.", file_name.to_string_lossy());
        std::process::exit(DIFF_FILE_NOT_FOUND);
    }
    let file1 = match File::open(&file_name) {
        Ok(r) => r,
        Err(err) => {
            eprintln!("cmpr: Can't open file {} - {}", file_name.to_string_lossy(), err);
            std::process::exit(DIFF_FILE_NOT_FOUND);
        }
    };
    let file_name2 = options.file2.unwrap();
    let file2 = match File::open(&file_name2) {
        Ok(r) => r,
        Err(err) => {
            eprintln!("cmpr: Can't open file {} - {}", file_name2.to_string_lossy(), err);
            std::process::exit(DIFF_FILE_NOT_FOUND);
        }
    };
    let len1 = fs::metadata(&file_name).unwrap().len();
    let len2 = fs::metadata(&file_name2).unwrap().len();
    let reader1 = BufReader::new(file1);
    let reader2 = BufReader::new(file2);

    let width = (min(len1, len2) as f32).log10().floor() as usize + 1;
    let single = match options.mode {
        Mode::Single => true,
        _ => false,
    };

    if single {
        match compare_files_single(reader1, reader2, skip) {
            Some(d) => {
                status = DIFF_FAIL;
                println!("{} {} differ at byte {}: {:02X} {:02X}", &file_name.to_string_lossy(), &file_name2.to_string_lossy(), d.0, d.1, d.2);
            },
            None => (),
        };
    } else {
        let vec = compare_files(reader1, reader2, skip);
        for d in vec {
            status = DIFF_FAIL;
            match options.mode {
                Mode::Byte => println!("{:width$} {:3} {:3}", d.0, d.1, d.2),
                Mode::Hex => println!("{:width$} {:02X} {:02X}", d.0, d.1, d.2),
                Mode::Char => println!("{:width$} {:1} {:1}", d.0, to_char(d.1), to_char(d.2)),
                Mode::Single => (),
            }
        }
        if len2 < len1 {
            println!("EOF on {} at byte {}", file_name2.to_string_lossy(), len2);
            status = DIFF_FILE_LEN_DIFF;
        } else if len1 < len2 {
            println!("EOF on {} at byte {}", file_name.to_string_lossy(), len1);
            status = DIFF_FILE_LEN_DIFF;
        }
    }
    std::process::exit(status);
}

fn compare_files_single<R>(reader1: R, reader2: R, skip: usize) -> Option<Data> where R: BufRead {
    let mut addr = skip;
    reader1.bytes().skip(skip).zip(reader2.bytes().skip(skip))
    .find_map(|d| {
        addr += 1;
        let x = d.0.unwrap();
        let y = d.1.unwrap();
        if x ==  y {
            None
        } else {
            Some((addr, x, y))
        }
     })
}

fn compare_files<R>(reader1: R, reader2: R, skip: usize) -> Vec<Data> where R: BufRead {
    let mut addr = skip;
    reader1.bytes().skip(skip).zip(reader2.bytes().skip(skip))
    .filter_map(|d| {
        addr += 1;
        let x = d.0.unwrap();
        let y = d.1.unwrap();
        if x == y {
            None
        } else {
            Some((addr, x, y))
        }
    })
    .collect()
}

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
        match b {
            0 => '\u{24ea}',
            1..=20 => char::from_u32(0x245f + b as u32).unwrap(),
            21..=31 => char::from_u32(0x3251 - 21 + b as u32).unwrap(),
            _ => '\u{25AF}'
        }
    }
}
