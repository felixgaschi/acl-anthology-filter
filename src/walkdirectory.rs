use std::{fs::read_link, path::PathBuf};

use walkdir::{DirEntry, WalkDir};

fn convert_to_pathbuf(dir_entry: DirEntry) -> PathBuf {
    if dir_entry.path_is_symlink() {
        read_link(dir_entry.path()).unwrap()
    } else {
        dir_entry.path().to_path_buf()
    }
}

/// Performs recursive walks in directory
/// and returns an iterator over files
/// with given extension
pub fn filter_walk_by_extension<'a>(
    directory: &PathBuf,
    extension: &'a str,
) -> Box<dyn 'a + Iterator<Item = PathBuf>> {
    Box::new(
        WalkDir::new(directory)
            .into_iter()
            .map(|x| x.unwrap())
            .filter(move |x| match x.path().extension() {
                Some(ext) => ext == extension,
                None => false,
            })
            .map(convert_to_pathbuf),
    )
}
