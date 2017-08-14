extern crate clap;
extern crate walkdir;
extern crate filtered_files;

use clap::ArgMatches;
use std::io::{BufWriter, Write, BufReader, BufRead};
use std::fs::File;
use walkdir::DirEntry;

use filtered_files::file_list;

pub fn run(options: ArgMatches) {
    let output_path = options.value_of("OUTPUT FILE").unwrap();
    let mut output_writer = output_writer(&output_path);

    for item in file_list(&options).iter() {
        match file_reader(&item) {
            Err(e) => println!("{}", e),
            Ok(reader) => {
                for line in reader.lines().map(|l| l.unwrap()) {
                    write!(output_writer, "{}\n", line)
                        .expect("Failed to write");
                }
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