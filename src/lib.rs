extern crate clap;
extern crate walkdir;
extern crate filtered_files;

use std::io::{BufWriter, Write, BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;
use std::process;
use clap::ArgMatches;
use walkdir::DirEntry;

use filtered_files::file_list;

pub fn run(options: ArgMatches) {
    let output_path = options.value_of("OUTPUT FILE").unwrap();
    let mut output_writer = output_writer(&output_path);

    for (index, item) in file_list(&options).iter().enumerate() {
        if options.is_present("VERBOSE") {
            print_file_info(&index, &item);
        }
        match file_reader(&item) {
            Err(e) => eprintln!("{}", e),
            Ok(reader) => {
                copy_to_output_file(reader, &mut output_writer, lines_to_skip(index, &options));
            },
        }
    }
}

fn output_writer(output_path: &str) -> BufWriter<File> {
    let output_file = File::create(output_path).expect("Failed to create output file");
    BufWriter::new(output_file)
}

fn file_reader(entry: &DirEntry) -> Result<BufReader<File>, String> {
    if let Ok(f) = File::open(entry.path()) {
        Ok(BufReader::new(f))
    } else { // TODO: this is ugly
        let mut msg = String::from("Failed to open file: ");
        msg.push_str(entry.file_name().to_str().unwrap());
        Err(msg)
    }
}

fn copy_to_output_file(reader: BufReader<File>, writer: &mut BufWriter<File>, lines_to_skip: usize) /*-> Result<(), String>*/ {
    let line_iter = reader
        .lines()
        .map(|l| l.unwrap())
        .skip(lines_to_skip);

    for line in line_iter {
        write!(writer, "{}\n", line).expect("Failed to write");
    }
}

fn lines_to_skip(file_index: usize, options: &ArgMatches) -> usize {
    let input = options.value_of("NUM HEADER LINES").unwrap_or("0");
    let input_lines_to_skip = usize::from_str(input).unwrap_or_else(|err|{
        eprintln!("Invalid number of header lines input: {}", err);
        process::exit(1);
    });

    match file_index {
        0 => 0,
        _ => input_lines_to_skip
    }
}

fn print_file_info(item_num: &usize, file: &DirEntry) {
    println!("{}: {}", item_num, file.path().to_str().unwrap());
}