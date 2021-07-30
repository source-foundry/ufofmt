use std::path::{Path, PathBuf};

use norad::Font;

use crate::lib::errors::{Error, Result};
use crate::lib::io;
use crate::lib::utils;

use crate::lib::io::{read_file_to_bytes, write_bytes_to_file};

/// Read/write roundtrip through the norad library. Returns Result with successful
/// &PathBuf path write or error
pub(crate) fn format_ufo(
    ufopath: &Path,
    unique_filename: &Option<String>,
    unique_extension: &Option<String>,
    singlequotes: bool,
) -> Result<PathBuf> {
    // validate UFO directory path request
    if !ufopath.exists() {
        return Err(Error::InvalidPath(ufopath.into()));
    }
    // define out directory path based on optional user-specified command line options
    let outpath;
    if unique_filename.is_some() || unique_extension.is_some() {
        outpath = utils::get_ufo_outpath(ufopath, unique_filename, unique_extension);
    } else {
        // if the user did not specify options for custom file name or custom
        // extension, then write in place over the in path
        outpath = ufopath.to_path_buf();
    }

    // norad lib read/write formatting
    let norad_rw_res = match Font::load(ufopath) {
        Ok(ufo) => match ufo.save(&outpath) {
            Ok(_) => Ok(outpath),
            Err(e) => Err(Error::NoradWrite(outpath, e)),
        },
        Err(e) => Err(Error::NoradRead(ufopath.into(), e)),
    };

    // single quote formatting
    if singlequotes {
        match norad_rw_res {
            Ok(p) => {
                let filepaths = io::walk_dir_for_plist_and_glif(&p);
                for filepath in filepaths {
                    let singlequote_res = write_bytes_to_file(
                        &filepath,
                        format_single_quotes(&mut read_file_to_bytes(&filepath)?),
                    );
                    match singlequote_res {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }
                }
                // return the UFO path wrapped in a Result
                Ok(p)
            }
            Err(e) => Err(e),
        }
    } else {
        norad_rw_res
    }
}

fn format_single_quotes(bytes: &mut Vec<u8>) -> &Vec<u8> {
    // format the string:
    // <?xml version="1.0" encoding="UTF-8"?> ...
    // to:
    // <?xml version='1.0' encoding='UTF-8'?> ...
    bytes[14] = 0x0027;
    bytes[18] = 0x0027;
    bytes[29] = 0x0027;
    bytes[35] = 0x0027;
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::Path;

    use fs_extra::dir::{copy, CopyOptions};
    use tempdir;

    #[test]
    fn test_format_ufo_invalid_dir_path_default() {
        let invalid_path = Path::new("totally/bogus/path/test.ufo");
        let res = format_ufo(invalid_path, &None, &None, false);
        match res {
            Ok(x) => panic!("failed with unexpected test result: {:?}", x),
            Err(err) => {
                assert!(matches!(err, Error::InvalidPath(_)));
            }
        }
        assert!(!invalid_path.exists());
    }

    #[test]
    fn test_format_ufo_invalid_dir_path_with_custom_names() {
        let invalid_path = Path::new("totally/bogus/path/test.ufo");
        let res =
            format_ufo(invalid_path, &Some("_new".to_string()), &Some(".test".to_string()), false);
        match res {
            Ok(x) => panic!("failed with unexpected test result: {:?}", x),
            Err(err) => {
                assert!(matches!(err, Error::InvalidPath(_)));
            }
        }
        let new_path = Path::new("totally/bogus/path/test_new.test");
        assert!(!new_path.exists());
    }

    #[test]
    fn test_format_ufo_valid_dir_path_default() {
        // setup
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        assert!(&src_ufo_path.exists());
        assert!(&tmp_dir.path().exists());
        let options = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &options);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        // test run of formatter across valid UFO sources
        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false);
        assert!(res_ufo_format.is_ok());
        assert_eq!(format!("{:?}", res_ufo_format.unwrap()), format!("{:?}", &test_ufo_path));
        assert!(&test_ufo_path.exists());
    }

    #[test]
    fn test_format_ufo_valid_dir_path_with_custom_names() {
        // setup
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        assert!(&src_ufo_path.exists());
        assert!(&tmp_dir.path().exists());
        let options = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &options);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        // test run of formatter across valid UFO sources
        let res_ufo_format =
            format_ufo(&test_ufo_path, &Some("_new".to_string()), &Some("test".to_string()), false);
        assert!(res_ufo_format.is_ok());
        let expected_path = tmp_dir.path().join("MutatorSansBoldCondensed_new.test");
        assert_eq!(format!("{:?}", res_ufo_format.unwrap()), format!("{:?}", expected_path));
        assert!(expected_path.exists() && expected_path.is_dir());
    }
}
