use std::path::Path;

use norad::Font;

use crate::lib::errors;
use crate::lib::validators;

/// Read/write roundtrip through the norad library. Returns Result with successful
/// &PathBuf path write or error
pub(crate) fn format_ufo(ufopath: &Path) -> errors::Result<&Path> {
    // validate UFO directory path request
    if validators::is_invalid_ufo_dir_path(ufopath) {
        let err_msg = format!("{:?}: not a valid UFO directory path", ufopath);
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, err_msg).into());
    }
    match Font::load(ufopath) {
        Ok(ufo) => match ufo.save(ufopath) {
            Ok(_) => Ok(ufopath),
            Err(e) => {
                let err_msg = format!("{:?}: norad library write error: {}", ufopath, e);
                Err(std::io::Error::new(std::io::ErrorKind::Other, err_msg).into())
            }
        },
        Err(e) => {
            let err_msg = format!("{:?}: norad library read error: {}", ufopath, e);
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
    fn test_format_ufo_invalid_dir_path() {
        let invalid_path = Path::new("totally/bogus/path/test.ufo");
        let res = format_ufo(invalid_path);
        assert!(res.is_err());
        match res {
            Ok(x) => panic!("failed with unexpected test result: {:?}", x),
            Err(err) => {
                assert_eq!(
                    err.to_string(),
                    "\"totally/bogus/path/test.ufo\": not a valid UFO directory path"
                );
            }
        }
    }

    #[test]
    fn test_format_ufo_valid_dir_path() -> Result<(), std::io::Error> {
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
        let res_ufo_format = format_ufo(&test_ufo_path);
        assert!(res_ufo_format.is_ok());
        assert_eq!(format!("{:?}", res_ufo_format.unwrap()), format!("{:?}", &test_ufo_path));

        Ok(())
    }
}