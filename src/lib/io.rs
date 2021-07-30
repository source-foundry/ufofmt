use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use std::fs;

use walkdir::{DirEntry, WalkDir};

use crate::errors::{Error, Result};

fn is_plist_or_glif_filepath(entry: &DirEntry) -> bool {
    entry
        .path()
        .extension()
        .map(|s| s == OsStr::new("glif") || s == OsStr::new("plist"))
        .unwrap_or(false)
}

pub(crate) fn walk_dir_for_plist_and_glif(ufopath: &Path) -> Vec<PathBuf> {
    WalkDir::new(ufopath)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|e| is_plist_or_glif_filepath(e))
        .map(|p| p.path().into())
        .collect::<Vec<PathBuf>>()
}

pub(crate) fn read_file_to_bytes(filepath: &Path) -> Result<Vec<u8>> {
    match fs::read(filepath) {
        Ok(s) => Ok(s),
        Err(e) => Err(Error::Read(filepath.into(), e.to_string())),
    }
}

pub(crate) fn write_bytes_to_file(filepath: &Path, contents: &[u8]) -> Result<()> {
    match fs::write(filepath, contents) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::Write(filepath.into(), e.to_string())),
    }
}
