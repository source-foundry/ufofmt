use std::path::{Path, PathBuf};

use walkdir::{DirEntry, WalkDir};

fn is_plist_or_glif_filepath(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".glif") || s.ends_with(".plist"))
        .unwrap_or(false)
}

pub(crate) fn walk_dir_for_plist_and_glif<P>(ufopath: P) -> Vec<PathBuf>
where
    P: AsRef<Path>,
{
    WalkDir::new(ufopath)
        .into_iter()
        .filter_entry(|e| is_plist_or_glif_filepath(e))
        .filter_map(|f| f.ok())
        .map(|g| g.into_path())
        .collect::<Vec<PathBuf>>()
}
