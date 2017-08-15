extern crate clap;
extern crate file_cat;

use clap::{App, Arg};

fn main() {
  let matches = App::new("FileCat")
    .version("0.0.1")
    .author("Charlie S. <charlieasaunders@gmail.com>")
    .about("Concatenates files meeting criteria.")
    .arg(Arg::with_name("OUTPUT FILE")
      .help("The output file path")
      .index(1)
      .takes_value(true)
      .required(true))
    .arg(Arg::with_name("INPUT PATH")
      .help("The input directory path")
      .index(2)
      .takes_value(true)
      .required(true))
    .arg(Arg::with_name("PATTERN")
      .help("A string containing a regex to filter the input files")
      .short("p")
      .long("pattern")
      .takes_value(true))
    .arg(Arg::with_name("CASE INSENSITIVE")
      .help("Sets filename pattern match to case-insensitive")
      .short("i")
      .long("ignore-case"))
    .arg(Arg::with_name("RECURSIVE")
      .help("Find matching files recursively")
      .short("r")
      .long("recursive"))
    .arg(Arg::with_name("NUM HEADER LINES")
      .help("The number of header lines. This number of lines will be skipped for each file after the first. Defaults to 0.")
      .short("h")
      .long("header-lines")
      .takes_value(true))
    .get_matches();

  file_cat::run(matches);
}