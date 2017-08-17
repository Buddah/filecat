# FileCat
A command line utility for combining files.

Primarily this is aimed at combining flat files (CSVs, TSVs, etc.). It doesn't do any parsing beyond dealing with lines to skip headers.

Run with `--help` for usage information.

## Getting Started
* [Install Rust](https://www.rust-lang.org/en-US/install.html)
* `git clone https://github.com/Buddah/filecat.git`
* `cd filecat`
* `cargo run -- --help`

### Example:
Combine all of the TSVs in the `test_files/` directory into `test_files/output.tsv`, keeping the first row of the first file we find as the header, and skip the first row in all other files.

`cargo run -- -p '.tsv$' -h 1 test_files/output.tsv test_files/`

The output file is excluded from the files to concatenate, so you can run this again without duplicating the data in `output.tsv`.

## Key features:
* Use a regular expression pattern for filenames to include
* Skip header lines from the beginning of files for files beyond the first
* Search directories recursively

## Possible future features:
* Optional pattern to exclude filenames
* Optional pattern for directory filtering
* Specify a file to use for header line(s)
* Skip lines at beginning of each file without adding them to the output
  * Before/after header line(s) as separate options?
* Skip lines at the end of each file to exclude footers
* Include hidden directories and files
