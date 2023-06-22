use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = String::from("."))]
    directory: String,

    #[arg(short, long)]
    output_file: Option<String>,

    #[arg(short, long)]
    anthology_path: Option<String>,
}

#[derive(Debug)]
pub struct ProcessedArgs {
    pub directory: PathBuf,
    pub anthology_path: Option<PathBuf>,
    pub output_file: PathBuf,
}

impl ProcessedArgs {
    pub fn parse() -> ProcessedArgs {
        let args = Args::parse();

        let directory = PathBuf::from(&args.directory);
        let anthology_path = match args.anthology_path {
            Some(path) => Some(PathBuf::from(path)),
            None => None,
        };
        let output_file = match args.output_file {
            Some(path) => PathBuf::from(path),
            None => {
                let mut p = PathBuf::from(directory.clone());
                p.push("filtered_anthology.bib");
                p
            }
        };
        ProcessedArgs {
            directory,
            anthology_path,
            output_file,
        }
    }
}
