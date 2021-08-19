use std::path::{Path, PathBuf};

use norad::{Font, QuoteChar, WriteOptions};

use crate::lib::errors::{Error, Result};
use crate::lib::utils;

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

    // norad lib read/write round trip formatting
    match Font::load(ufopath) {
        Ok(ufo) => {
            // optional XML declaration quote style customization
            let quote_style = {
                match singlequotes {
                    true => QuoteChar::Single,
                    false => QuoteChar::Double,
                }
            };
            // Norad serialization formatting options
            let options = WriteOptions::default().quote_char(quote_style);
            // Execute serialization with options
            match ufo.save_with_options(&outpath, &options) {
                Ok(_) => Ok(outpath),
                Err(e) => Err(Error::NoradWrite(outpath, e)),
            }
        }
        Err(e) => Err(Error::NoradRead(ufopath.into(), e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{fs, path::Path};

    use fs_extra::dir::{copy, CopyOptions};
    use tempdir;

    // ~~~~~~~~~~~~~~~
    // Path validation
    // ~~~~~~~~~~~~~~~

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

    // ~~~~~~~~~~~~
    // Custom paths
    // ~~~~~~~~~~~~

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

    // ~~~~~~~~~~~~~~~~~~~~
    // Serialization format
    // ~~~~~~~~~~~~~~~~~~~~

    // Defaults
    #[test]
    fn test_format_ufo_default() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false);
        assert!(res_ufo_format.is_ok());
        // fontinfo.plist
        let test_fontinfo_string =
            fs::read_to_string(&test_ufo_path.join("fontinfo.plist")).unwrap();
        let expected_fontinfo_path = Path::new("testdata/expected/fontinfo.default.plist");
        let expected_fontinfo_string = fs::read_to_string(expected_fontinfo_path).unwrap();
        // glif file
        // let test_glyph_string =
        //     fs::read_to_string(&test_ufo_path.join("glyphs").join("A_.glif")).unwrap();
        // let expected_glyph_path = Path::new("testdata/expected/A_.default.glif");
        // let expected_glyph_string = fs::read_to_string(expected_glyph_path).unwrap();
        // groups.plist
        let test_groups_string = fs::read_to_string(&test_ufo_path.join("groups.plist")).unwrap();
        let expected_groups_path = Path::new("testdata/expected/groups.default.plist");
        let expected_groups_string = fs::read_to_string(expected_groups_path).unwrap();
        // kerning.plist
        let test_kerning_string = fs::read_to_string(&test_ufo_path.join("kerning.plist")).unwrap();
        let expected_kerning_path = Path::new("testdata/expected/kerning.default.plist");
        let expected_kerning_string = fs::read_to_string(expected_kerning_path).unwrap();
        // layercontents.plist
        let test_lc_string =
            fs::read_to_string(&test_ufo_path.join("layercontents.plist")).unwrap();
        let expected_lc_path = Path::new("testdata/expected/layercontents.default.plist");
        let expected_lc_string = fs::read_to_string(expected_lc_path).unwrap();
        // lib.plist
        let test_lib_string = fs::read_to_string(&test_ufo_path.join("lib.plist")).unwrap();
        let expected_lib_path = Path::new("testdata/expected/lib.default.plist");
        let expected_lib_string = fs::read_to_string(expected_lib_path).unwrap();
        // metainfo.plist
        let test_mi_string = fs::read_to_string(&test_ufo_path.join("metainfo.plist")).unwrap();
        let expected_mi_path = Path::new("testdata/expected/metainfo.default.plist");
        let expected_mi_string = fs::read_to_string(expected_mi_path).unwrap();
        // glyphs/contents.plist
        let test_contents_string =
            fs::read_to_string(&test_ufo_path.join("glyphs").join("contents.plist")).unwrap();
        let expected_contents_path = Path::new("testdata/expected/contents.default.plist");
        let expected_contents_string = fs::read_to_string(expected_contents_path).unwrap();

        // observed vs. expected string tests
        assert_eq!(expected_glyph_string, test_glyph_string);
        assert_eq!(expected_fontinfo_string, test_fontinfo_string);
        assert_eq!(expected_groups_string, test_groups_string);
        assert_eq!(expected_kerning_string, test_kerning_string);
        assert_eq!(expected_lc_string, test_lc_string);
        assert_eq!(expected_lib_string, test_lib_string);
        assert_eq!(expected_mi_string, test_mi_string);
        assert_eq!(expected_contents_string, test_contents_string);
    }

    // XML declaration optional attribute single quote formatting
    #[test]
    fn test_format_ufo_singlequote() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, true);
        assert!(res_ufo_format.is_ok());
        let test_glyph = fs::read_to_string(&test_ufo_path.join("glyphs").join("A_.glif")).unwrap();
        let expected_path = Path::new("testdata/expected/A_.singlequote.glif");
        let expected_glyph = fs::read_to_string(expected_path).unwrap();

        // observed vs. expected string tests
        assert_eq!(expected_glyph, test_glyph);
    }
}
