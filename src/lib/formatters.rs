use std::path::{Path, PathBuf};

use norad::Font;

use crate::lib::errors;
use crate::lib::utils;
use crate::lib::validators;

/// Read/write roundtrip through the norad library. Returns Result with successful
/// &PathBuf path write or error
pub(crate) fn format_ufo(
    ufopath: &Path,
    unique_filename: &Option<String>,
    unique_extension: &Option<String>,
) -> errors::Result<PathBuf> {
    // validate UFO directory path request
    if validators::is_invalid_ufo_dir_path(ufopath) {
        let err_msg = format!("{}: not a valid UFO directory path", ufopath.display());
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, err_msg).into());
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

    match Font::load(ufopath) {
        Ok(ufo) => match ufo.save(&outpath) {
            Ok(_) => Ok(outpath),
            Err(e) => {
                let err_msg = format!("{}: norad library write error: {}", &outpath.display(), e);
                Err(std::io::Error::new(std::io::ErrorKind::Other, err_msg).into())
            }
        },
        Err(e) => {
            let err_msg = format!("{}: norad library read error: {}", ufopath.display(), e);
            Err(std::io::Error::new(std::io::ErrorKind::Other, err_msg).into())
        }
    }
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
        let res = format_ufo(invalid_path, &None, &None);
        assert!(res.is_err());
        match res {
            Ok(x) => panic!("failed with unexpected test result: {:?}", x),
            Err(err) => {
                assert_eq!(
                    err.to_string(),
                    "totally/bogus/path/test.ufo: not a valid UFO directory path"
                );
            }
        }
        assert!(!invalid_path.exists());
    }

    #[test]
    fn test_format_ufo_invalid_dir_path_with_custom_names() {
        let invalid_path = Path::new("totally/bogus/path/test.ufo");
        let res = format_ufo(invalid_path, &Some("_new".to_string()), &Some(".test".to_string()));
        assert!(res.is_err());
        match res {
            Ok(x) => panic!("failed with unexpected test result: {:?}", x),
            Err(err) => {
                assert_eq!(
                    err.to_string(),
                    "totally/bogus/path/test.ufo: not a valid UFO directory path"
                );
            }
        }
        let new_path = Path::new("totally/bogus/path/test_new.test");
        assert!(!new_path.exists());
    }

    #[test]
    fn test_format_ufo_valid_dir_path_default() -> Result<(), std::io::Error> {
        // setup
        let tmp_dir = tempdir::TempDir::new("test")?;
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        assert!(&src_ufo_path.exists());
        assert!(&tmp_dir.path().exists());
        let options = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &options);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        // test run of formatter across valid UFO sources
        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None);
        assert!(res_ufo_format.is_ok());
        assert_eq!(format!("{:?}", res_ufo_format.unwrap()), format!("{:?}", &test_ufo_path));
        assert!(&test_ufo_path.exists());

        Ok(())
    }

    #[test]
    fn test_format_ufo_valid_dir_path_with_custom_names() -> Result<(), std::io::Error> {
        // setup
        let tmp_dir = tempdir::TempDir::new("test")?;
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        assert!(&src_ufo_path.exists());
        assert!(&tmp_dir.path().exists());
        let options = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &options);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        // test run of formatter across valid UFO sources
        let res_ufo_format =
            format_ufo(&test_ufo_path, &Some("_new".to_string()), &Some("test".to_string()));
        assert!(res_ufo_format.is_ok());
        let expected_path = tmp_dir.path().join("MutatorSansBoldCondensed_new.test");
        assert_eq!(format!("{:?}", res_ufo_format.unwrap()), format!("{:?}", expected_path));
        assert!(expected_path.exists() && expected_path.is_dir());

        Ok(())
    }
}
