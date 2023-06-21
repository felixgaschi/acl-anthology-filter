mod bibliography;
mod citations;
mod walkdirectory;
use std::fs;
use std::io::Write;
use std::{collections::HashSet, fs::File};
#[macro_use]
extern crate lazy_static;

use bibliography::{get_anthology_file, parse_bibliography};
use citations::extract_citation;
use walkdirectory::filter_walk_by_extension;

fn main() -> std::io::Result<()> {
    let directory = "/Users/felixgaschi/Documents/phd-thesis";
    let output_file = "/Users/felixgaschi/Documents/phd-thesis/filtered_anthology.bib";

    let anthology_path = get_anthology_file(&directory, None)?;
    let anthology_contents = fs::read_to_string(anthology_path)?;
    let anthology = parse_bibliography(&anthology_contents);

    let mut results: HashSet<String> = HashSet::new();
    for dir_entry in filter_walk_by_extension(directory, "tex") {
        let contents: String = fs::read_to_string(dir_entry)?;
        for captured_group in extract_citation(&contents) {
            if anthology.contains_key(captured_group) {
                results.insert(String::from(captured_group));
            }
        }
    }

    let mut f = File::create(output_file)?;

    for captured_string in results {
        let _ = f.write((anthology.get(&captured_string).unwrap().to_owned() + "\n").as_bytes());
    }

    Ok(())
}
