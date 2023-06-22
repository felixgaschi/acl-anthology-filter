mod bibliography;
mod citations;
mod environment;
mod walkdirectory;
use clap::Parser;
use std::fs;
use std::io::Write;
use std::{collections::HashSet, fs::File};
#[macro_use]
extern crate lazy_static;

use bibliography::{get_anthology_file, parse_bibliography};
use citations::extract_citation;
use environment::{Cli, Commands, ProcessedArgs, ProcessedFilterArgs, ProcessedSearchArgs};
use walkdirectory::filter_walk_by_extension;

fn run_filter(args: ProcessedFilterArgs) -> std::io::Result<()> {
    let anthology_path = get_anthology_file(&args.directory, args.anthology_path)?;
    let anthology_contents = fs::read_to_string(anthology_path)?;
    let anthology = parse_bibliography(&anthology_contents);

    let mut results: HashSet<String> = HashSet::new();
    for dir_entry in filter_walk_by_extension(&args.directory, "tex") {
        let contents: String = fs::read_to_string(dir_entry)?;
        for captured_group in extract_citation(&contents) {
            if anthology.contains_key(captured_group) {
                results.insert(String::from(captured_group));
            }
        }
    }

    let mut f = File::create(args.output_file)?;

    for captured_string in results {
        let _ = f.write((anthology.get(&captured_string).unwrap().to_owned() + "\n").as_bytes());
    }

    Ok(())
}

fn run_search(args: ProcessedSearchArgs) -> std::io::Result<()> {
    let anthology_contents = fs::read_to_string(args.anthology_path)?;
    let anthology = parse_bibliography(&anthology_contents);

    let mut count_correct = 0;
    let lower_search_string = args.search_string.to_lowercase();

    for (_key, val) in anthology.iter() {
        if val.to_lowercase().contains(&lower_search_string) {
            println!("{}", val);
            count_correct = count_correct + 1;
        }
        if let Some(_a) = args.n {
            if args.n.unwrap() <= count_correct {
                break;
            }
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Filter(args) => run_filter(ProcessedFilterArgs::process(&args)),
        Commands::Search(args) => run_search(ProcessedSearchArgs::process(&args)),
    }
}
