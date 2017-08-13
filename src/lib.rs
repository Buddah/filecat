extern crate clap;
extern crate filtered_files;

use clap::ArgMatches;
use std::io::{BufWriter, Write};
use std::fs::File;

use filtered_files::file_list;

pub fn run(options: ArgMatches) {
    let output_path = options.value_of("OUTPUT FILE").unwrap();
    let mut output_writer = output_writer(&output_path);

    for item in file_list(&options).iter() {
        write!(output_writer, "{:?}\n", item)
            .expect("Failed to write");
    }
}

fn output_writer(output_path: &str) -> BufWriter<File> {
    let output_file = File::create(output_path).expect("Failed to create output file");
    BufWriter::new(output_file)
}
