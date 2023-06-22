use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
    path::PathBuf, 
};

use crate::walkdirectory::filter_walk_by_extension;

pub fn get_anthology_file(
    directory: &PathBuf,
    anthology_path: Option<PathBuf>,
) -> Result<PathBuf, Error> {
    if let Some(real_anthology_path) = anthology_path {
        return Ok(real_anthology_path);
    }

    for file_name in filter_walk_by_extension(directory, "bib") {
        if file_name.file_name().unwrap().to_str().unwrap() == "anthology.bib" {
            return Ok(file_name);
        }
    }
    Err::<PathBuf, std::io::Error>(
        Error::new(
            ErrorKind::NotFound, 
            format!(
                "anthology.bib does not exist in {:?}. If it has another address, you need to specify it in the get_bilbiography function",
                directory.as_os_str()
            )
        )
    )
}

pub fn parse_bibliography(contents: &str) -> HashMap<String,String> {
    let mut bibliography: HashMap<String, String> = HashMap::new();
    let mut bracket_level = 0;
    let mut reading_value = false;
    let mut reading_identifier = false;
    let mut value = String::new();
    let mut identifier = String::new();
    let mut line = 0;
    for c in contents.chars() {
        if "\n".contains(c) {
            line = line + 1;
        }

        if bracket_level == 0 && "@".contains(c) {
            //println!("Starting reading value");
            reading_value = true;
        }

        if reading_identifier && ",".contains(c) {
            reading_identifier = false;
        }

        if "{".contains(c) {
            if bracket_level == 0 {
                reading_identifier = true;
            }
            bracket_level = bracket_level + 1;
        } else if "}".contains(c) {
            bracket_level = bracket_level - 1;
            if bracket_level == 0 {
                reading_value = false;
                value.push(c);
                bibliography.insert(identifier.clone(), value.clone());
                value = String::new();
                identifier = String::new();
            }
        }

        if reading_value {
            value.push(c)
        }
        if reading_identifier && !"{".contains(c) {
            identifier.push(c)
        }
    }
    bibliography
}
