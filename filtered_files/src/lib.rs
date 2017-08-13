extern crate walkdir;
extern crate clap;
extern crate regex;

use std::path::Path;
use regex::Regex;
use walkdir::{DirEntry, WalkDir, WalkDirIterator};
use clap::ArgMatches;

pub fn file_list(options: &ArgMatches) -> Vec<DirEntry> {
    let output_path = prefixed_path(options.value_of("OUTPUT FILE").unwrap());

    dir_walker(&options)
        .into_iter()
        .filter_entry(|e| {
            !is_hidden(e) &&
            !is_output_file(e, &output_path) &&
            filename_matches_pattern(e, &options)
        })
        .filter_map(|e| e.ok())
        .filter(|e| !is_directory(e))
        .collect()
}

// it doesn't seem like this should be needed
// A Path struct seems like it should be able to give the
// full relative reference (with the "./")
// TODO: use OS separator instead
fn prefixed_path(original_path: &str) -> String {
    if original_path.contains("/") {
        String::from(original_path)
    } else {
        let mut new_str = String::from("./");
        new_str.push_str(original_path);
        new_str
    }
}

fn dir_walker(options: &ArgMatches) -> WalkDir {
    let input_path = options.value_of("INPUT PATH").unwrap();

    if options.is_present("RECURSIVE") { 
        WalkDir::new(input_path)
    } else {
        WalkDir::new(input_path).max_depth(1)
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    filename_meets_condition(entry, |name| name.starts_with(".")) && !is_internal_link(entry)
}

fn is_internal_link(entry: &DirEntry) -> bool {
    filename_meets_condition(entry, |name| name == "." || name == "..")
}

fn is_output_file(entry: &DirEntry, output_path: &str) -> bool {
    entry.path() == Path::new(output_path)
}

fn is_directory(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
}

fn filename_matches_pattern(entry: &DirEntry, options: &ArgMatches) -> bool {
    if !options.is_present("PATTERN") {
        true
    } else {
        let regex = get_regex(options);
        filename_meets_condition(entry, |name| regex.is_match(name)) || is_directory(entry)
    }
}

fn get_regex(options: &ArgMatches) -> Regex {
    let input = options.value_of("PATTERN").unwrap();
    let mut pattern_str = String::from(input);

    if options.is_present("CASE INSENSITIVE") {
        pattern_str = String::from("(?i)") + input;
    }
    
    Regex::new(&pattern_str).expect("Failed to parse pattern as regex")
}

fn filename_meets_condition<F>(entry: &DirEntry, predicate: F) -> bool
    where F: Fn(&str) -> bool {

    entry.file_name()
         .to_str()
         .map(|name| predicate(name))
         .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::prefixed_path;
    #[test]
    fn prefixed_path_full() {
        let s = "./hey/what.txt";
        assert_eq!(s, prefixed_path(s));
    }

    #[test]
    fn prefixed_path_name_only() {
        let original = "what.txt";
        let expected = "./what.txt";
        assert_eq!(expected, prefixed_path(original));
    }
}