use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Filter(FilterArgs),
    Search(SearchArgs),
}

#[derive(Args)]
pub struct FilterArgs {
    #[arg(short, long)]
    anthology_path: Option<String>,

    #[arg(short, long, default_value_t = String::from("."))]
    directory: String,

    #[arg(short, long)]
    output_file: Option<String>,
}

#[derive(Args)]
pub struct SearchArgs {
    #[arg(short, long)]
    anthology_path: Option<String>,

    #[arg(short, long)]
    search_string: String,

    #[arg(short, long)]
    n: Option<usize>,
}

pub trait ProcessedArgs {
    type T: Args;
    fn process(args: &Self::T) -> Self;
}

#[derive(Debug)]
pub struct ProcessedFilterArgs {
    pub directory: PathBuf,
    pub anthology_path: Option<PathBuf>,
    pub output_file: PathBuf,
}

impl ProcessedArgs for ProcessedFilterArgs {
    type T = FilterArgs;

    fn process(args: &Self::T) -> Self {
        let directory = PathBuf::from(&args.directory);
        let anthology_path = match &args.anthology_path {
            Some(path) => Some(PathBuf::from(path)),
            None => None,
        };
        let output_file = match &args.output_file {
            Some(path) => PathBuf::from(path),
            None => {
                let mut p = PathBuf::from(directory.clone());
                p.push("filtered_anthology.bib");
                p
            }
        };
        return Self {
            directory,
            anthology_path,
            output_file,
        };
    }
}

#[derive(Debug)]
pub struct ProcessedSearchArgs {
    pub anthology_path: PathBuf,
    pub search_string: String,
    pub n: Option<usize>,
}

impl ProcessedArgs for ProcessedSearchArgs {
    type T = SearchArgs;

    fn process(args: &Self::T) -> Self {
        let anthology_path = match &args.anthology_path {
            Some(path) => PathBuf::from(path),
            None => PathBuf::from("anthology.bib"),
        };

        return Self {
            anthology_path,
            search_string: args.search_string.clone(),
            n: args.n,
        };
    }
}
