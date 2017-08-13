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
      .short("p")
      .long("pattern")
      .help("A string containing a regex to filter the input files")
      .takes_value(true))
    .arg(Arg::with_name("RECURSIVE")
      .short("r")
      .long("recursive")
      .help("Find matching files recursively"))
    .get_matches();

  file_cat::run(matches);
}