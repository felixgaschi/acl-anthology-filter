use regex::Regex;

lazy_static! {
    pub static ref CITATION_REGEX_SET: Vec<Regex> = {
        let citation_commands = [
            String::from(r"cite"),
            String::from(r"citep"),
            String::from(r"citet"),
        ];
        let mut regex_set: Vec<Regex> = Vec::new();
        for command in citation_commands {
            let full_command = String::from(r"\\") + &command + &String::from(r"\{([^\}]+)\}");
            regex_set.push(Regex::new(&full_command).unwrap());
        }
        regex_set
    };
}

pub fn extract_citation<'a>(contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = Vec::new();
    for re in CITATION_REGEX_SET.iter() {
        for cap in re.captures_iter(contents) {
            let raw_capture = cap.get(1).unwrap().as_str();
            for part in raw_capture.split(",") {
                result.push(part);
            }
        }
    }
    result
}
