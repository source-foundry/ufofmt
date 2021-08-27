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
    use pretty_assertions::assert_eq;
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
    fn test_format_default_glif() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false);
        assert!(res_ufo_format.is_ok());

        // glif file
        let test_glyph_string =
            fs::read_to_string(&test_ufo_path.join("glyphs").join("A_.glif")).unwrap();
        let expected_glyph_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<glyph name=\"A\" format=\"2\">
\t<unicode hex=\"0041\"/>
\t<advance width=\"740\"/>
\t<outline>
\t\t<contour>
\t\t\t<point x=\"-10\" y=\"0\" type=\"line\"/>
\t\t\t<point x=\"250\" y=\"0\" type=\"line\"/>
\t\t\t<point x=\"334\" y=\"800\" type=\"line\"/>
\t\t\t<point x=\"104\" y=\"800\" type=\"line\"/>
\t\t</contour>
\t\t<contour>
\t\t\t<point x=\"110\" y=\"120\" type=\"line\"/>
\t\t\t<point x=\"580\" y=\"120\" type=\"line\"/>
\t\t\t<point x=\"580\" y=\"330\" type=\"line\"/>
\t\t\t<point x=\"110\" y=\"330\" type=\"line\"/>
\t\t</contour>
\t\t<contour>
\t\t\t<point x=\"390\" y=\"0\" type=\"line\"/>
\t\t\t<point x=\"730\" y=\"0\" type=\"line\"/>
\t\t\t<point x=\"614\" y=\"800\" type=\"line\"/>
\t\t\t<point x=\"294\" y=\"800\" type=\"line\"/>
\t\t</contour>
\t\t<contour>
\t\t\t<point x=\"204\" y=\"540\" type=\"line\"/>
\t\t\t<point x=\"474\" y=\"540\" type=\"line\"/>
\t\t\t<point x=\"474\" y=\"800\" type=\"line\"/>
\t\t\t<point x=\"204\" y=\"800\" type=\"line\"/>
\t\t</contour>
\t</outline>
\t<lib>
\t\t<dict>
\t\t\t<key>com.typemytype.robofont.Image.Brightness</key>
\t\t\t<integer>0</integer>
\t\t\t<key>com.typemytype.robofont.Image.Contrast</key>
\t\t\t<integer>1</integer>
\t\t\t<key>com.typemytype.robofont.Image.Saturation</key>
\t\t\t<integer>1</integer>
\t\t\t<key>com.typemytype.robofont.Image.Sharpness</key>
\t\t\t<real>0.4</real>
\t\t</dict>
\t</lib>
</glyph>
";

        // observed vs. expected string tests
        assert_eq!(expected_glyph_string, test_glyph_string);
    }

    #[test]
    fn test_format_default_fontinfo_plist() {
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

        let expected_fontinfo_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
\t<key>ascender</key>
\t<integer>800</integer>
\t<key>capHeight</key>
\t<integer>800</integer>
\t<key>copyright</key>
\t<string>License same as MutatorMath. BSD 3-clause. [test-token: A]</string>
\t<key>descender</key>
\t<integer>-200</integer>
\t<key>familyName</key>
\t<string>MutatorMathTest</string>
\t<key>guidelines</key>
\t<array/>
\t<key>italicAngle</key>
\t<integer>0</integer>
\t<key>openTypeNameLicense</key>
\t<string>License same as MutatorMath. BSD 3-clause. [test-token: A]</string>
\t<key>openTypeOS2VendorID</key>
\t<string>LTTR</string>
\t<key>postscriptBlueValues</key>
\t<array>
\t\t<integer>-10</integer>
\t\t<integer>0</integer>
\t\t<integer>800</integer>
\t\t<integer>810</integer>
\t</array>
\t<key>postscriptDefaultWidthX</key>
\t<integer>500</integer>
\t<key>postscriptFamilyBlues</key>
\t<array/>
\t<key>postscriptFamilyOtherBlues</key>
\t<array/>
\t<key>postscriptFontName</key>
\t<string>MutatorMathTest-BoldCondensed</string>
\t<key>postscriptFullName</key>
\t<string>MutatorMathTest BoldCondensed</string>
\t<key>postscriptOtherBlues</key>
\t<array>
\t\t<integer>500</integer>
\t\t<integer>520</integer>
\t</array>
\t<key>postscriptSlantAngle</key>
\t<integer>0</integer>
\t<key>postscriptStemSnapH</key>
\t<array/>
\t<key>postscriptStemSnapV</key>
\t<array/>
\t<key>postscriptWindowsCharacterSet</key>
\t<integer>1</integer>
\t<key>styleMapFamilyName</key>
\t<string></string>
\t<key>styleMapStyleName</key>
\t<string>regular</string>
\t<key>styleName</key>
\t<string>BoldCondensed</string>
\t<key>unitsPerEm</key>
\t<integer>1000</integer>
\t<key>versionMajor</key>
\t<integer>1</integer>
\t<key>versionMinor</key>
\t<integer>2</integer>
\t<key>xHeight</key>
\t<integer>500</integer>
\t<key>year</key>
\t<integer>2004</integer>
</dict>
</plist>";

        // observed vs. expected string tests
        assert_eq!(expected_fontinfo_string, test_fontinfo_string);
    }

    #[test]
    fn test_format_default_groups_plist() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false);
        assert!(res_ufo_format.is_ok());

        // groups.plist
        let test_groups_string = fs::read_to_string(&test_ufo_path.join("groups.plist")).unwrap();
        let expected_groups_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
\t<key>public.kern1.@MMK_L_A</key>
\t<array>
\t\t<string>A</string>
\t</array>
\t<key>public.kern2.@MMK_R_A</key>
\t<array>
\t\t<string>A</string>
\t</array>
\t<key>testGroup</key>
\t<array>
\t\t<string>E</string>
\t\t<string>F</string>
\t\t<string>H</string>
\t</array>
</dict>
</plist>";

        // observed vs. expected string tests
        assert_eq!(expected_groups_string, test_groups_string);
    }

    #[test]
    fn test_format_default_kerning_plist() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false);
        assert!(res_ufo_format.is_ok());

        // kerning.plist
        let test_kerning_string = fs::read_to_string(&test_ufo_path.join("kerning.plist")).unwrap();

        let expected_kerning_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
\t<key>A</key>
\t<dict>
\t\t<key>J</key>
\t\t<integer>-20</integer>
\t\t<key>O</key>
\t\t<integer>-30</integer>
\t\t<key>T</key>
\t\t<integer>-70</integer>
\t\t<key>U</key>
\t\t<integer>-30</integer>
\t\t<key>V</key>
\t\t<integer>-50</integer>
\t</dict>
\t<key>B</key>
\t<dict>
\t\t<key>A</key>
\t\t<integer>-20</integer>
\t\t<key>J</key>
\t\t<integer>-50</integer>
\t\t<key>O</key>
\t\t<integer>-20</integer>
\t\t<key>S</key>
\t\t<integer>-10</integer>
\t\t<key>T</key>
\t\t<integer>-10</integer>
\t\t<key>U</key>
\t\t<integer>-20</integer>
\t\t<key>V</key>
\t\t<integer>-30</integer>
\t</dict>
\t<key>C</key>
\t<dict>
\t\t<key>A</key>
\t\t<integer>-20</integer>
\t\t<key>J</key>
\t\t<integer>-50</integer>
\t\t<key>T</key>
\t\t<integer>-20</integer>
\t\t<key>V</key>
\t\t<integer>-20</integer>
\t</dict>
\t<key>E</key>
\t<dict>
\t\t<key>J</key>
\t\t<integer>-20</integer>
\t\t<key>T</key>
\t\t<integer>-10</integer>
\t\t<key>V</key>
\t\t<integer>-10</integer>
\t</dict>
\t<key>F</key>
\t<dict>
\t\t<key>A</key>
\t\t<integer>-40</integer>
\t\t<key>J</key>
\t\t<integer>-80</integer>
\t\t<key>O</key>
\t\t<integer>-10</integer>
\t\t<key>S</key>
\t\t<integer>-20</integer>
\t\t<key>U</key>
\t\t<integer>-10</integer>
\t\t<key>V</key>
\t\t<integer>-10</integer>
\t</dict>
\t<key>G</key>
\t<dict>
\t\t<key>J</key>
\t\t<integer>-20</integer>
\t\t<key>S</key>
\t\t<integer>-10</integer>
\t\t<key>T</key>
\t\t<integer>-40</integer>
\t\t<key>U</key>
\t\t<integer>-10</integer>
\t\t<key>V</key>
\t\t<integer>-30</integer>
\t</dict>
\t<key>H</key>
\t<dict>
\t\t<key>J</key>
\t\t<integer>-30</integer>
\t\t<key>S</key>
\t\t<integer>-10</integer>
\t\t<key>T</key>
\t\t<integer>-10</integer>
\t</dict>
\t<key>J</key>
\t<dict>
\t\t<key>J</key>
\t\t<integer>-70</integer>
\t</dict>
\t<key>L</key>
\t<dict>
\t\t<key>J</key>
\t\t<integer>-20</integer>
\t\t<key>O</key>
\t\t<integer>-20</integer>
\t\t<key>T</key>
\t\t<integer>-110</integer>
\t\t<key>U</key>
\t\t<integer>-20</integer>
\t\t<key>V</key>
\t\t<integer>-60</integer>
\t</dict>
\t<key>O</key>
\t<dict>
\t\t<key>A</key>
\t\t<integer>-30</integer>
\t\t<key>J</key>
\t\t<integer>-60</integer>
\t\t<key>S</key>
\t\t<integer>-10</integer>
\t\t<key>T</key>
\t\t<integer>-30</integer>
\t\t<key>V</key>
\t\t<integer>-30</integer>
\t</dict>
\t<key>P</key>
\t<dict>
\t\t<key>A</key>
\t\t<integer>-50</integer>
\t\t<key>J</key>
\t\t<integer>-100</integer>
\t\t<key>S</key>
\t\t<integer>-10</integer>
\t\t<key>T</key>
\t\t<integer>-10</integer>
\t\t<key>U</key>
\t\t<integer>-10</integer>
\t\t<key>V</key>
\t\t<integer>-20</integer>
\t</dict>
\t<key>R</key>
\t<dict>
\t\t<key>H</key>
\t\t<integer>-10</integer>
\t\t<key>J</key>
\t\t<integer>-20</integer>
\t\t<key>O</key>
\t\t<integer>-30</integer>
\t\t<key>S</key>
\t\t<integer>-20</integer>
\t\t<key>T</key>
\t\t<integer>-30</integer>
\t\t<key>U</key>
\t\t<integer>-30</integer>
\t\t<key>V</key>
\t\t<integer>-40</integer>
\t</dict>
\t<key>S</key>
\t<dict>
\t\t<key>A</key>
\t\t<integer>-20</integer>
\t\t<key>H</key>
\t\t<integer>-20</integer>
\t\t<key>J</key>
\t\t<integer>-40</integer>
\t\t<key>O</key>
\t\t<integer>-10</integer>
\t\t<key>S</key>
\t\t<integer>-10</integer>
\t\t<key>T</key>
\t\t<integer>-30</integer>
\t\t<key>U</key>
\t\t<integer>-10</integer>
\t\t<key>V</key>
\t\t<integer>-30</integer>
\t\t<key>W</key>
\t\t<integer>-10</integer>
\t</dict>
\t<key>T</key>
\t<dict>
\t\t<key>A</key>
\t\t<integer>-65</integer>
\t\t<key>H</key>
\t\t<integer>-10</integer>
\t\t<key>J</key>
\t\t<integer>-130</integer>
\t\t<key>O</key>
\t\t<integer>-20</integer>
\t</dict>
\t<key>U</key>
\t<dict>
\t\t<key>A</key>
\t\t<integer>-30</integer>
\t\t<key>J</key>
\t\t<integer>-60</integer>
\t\t<key>S</key>
\t\t<integer>-10</integer>
\t\t<key>V</key>
\t\t<integer>-10</integer>
\t</dict>
\t<key>V</key>
\t<dict>
\t\t<key>J</key>
\t\t<integer>-100</integer>
\t\t<key>O</key>
\t\t<integer>-30</integer>
\t\t<key>S</key>
\t\t<integer>-20</integer>
\t\t<key>U</key>
\t\t<integer>-10</integer>
\t</dict>
</dict>
</plist>";

        // observed vs. expected string tests
        assert_eq!(expected_kerning_string, test_kerning_string);
    }

    #[test]
    fn test_format_default_layercontents_plist() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false);
        assert!(res_ufo_format.is_ok());

        // layercontents.plist
        let test_lc_string =
            fs::read_to_string(&test_ufo_path.join("layercontents.plist")).unwrap();
        let expected_lc_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<array>
\t<array>
\t\t<string>foreground</string>
\t\t<string>glyphs</string>
\t</array>
\t<array>
\t\t<string>background</string>
\t\t<string>glyphs.background</string>
\t</array>
</array>
</plist>";

        // observed vs. expected string tests
        assert_eq!(expected_lc_string, test_lc_string);
    }

    #[test]
    fn test_format_default_lib_plist() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false);
        assert!(res_ufo_format.is_ok());

        // lib.plist
        let test_lib_string = fs::read_to_string(&test_ufo_path.join("lib.plist")).unwrap();

        let expected_lib_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
\t<key>com.defcon.sortDescriptor</key>
\t<array>
\t\t<dict>
\t\t\t<key>allowPseudoUnicode</key>
\t\t\t<true/>
\t\t\t<key>ascending</key>
\t\t\t<true/>
\t\t\t<key>type</key>
\t\t\t<string>alphabetical</string>
\t\t</dict>
\t\t<dict>
\t\t\t<key>allowPseudoUnicode</key>
\t\t\t<true/>
\t\t\t<key>ascending</key>
\t\t\t<true/>
\t\t\t<key>type</key>
\t\t\t<string>category</string>
\t\t</dict>
\t\t<dict>
\t\t\t<key>allowPseudoUnicode</key>
\t\t\t<true/>
\t\t\t<key>ascending</key>
\t\t\t<true/>
\t\t\t<key>type</key>
\t\t\t<string>unicode</string>
\t\t</dict>
\t\t<dict>
\t\t\t<key>allowPseudoUnicode</key>
\t\t\t<true/>
\t\t\t<key>ascending</key>
\t\t\t<true/>
\t\t\t<key>type</key>
\t\t\t<string>script</string>
\t\t</dict>
\t\t<dict>
\t\t\t<key>allowPseudoUnicode</key>
\t\t\t<true/>
\t\t\t<key>ascending</key>
\t\t\t<true/>
\t\t\t<key>type</key>
\t\t\t<string>suffix</string>
\t\t</dict>
\t\t<dict>
\t\t\t<key>allowPseudoUnicode</key>
\t\t\t<true/>
\t\t\t<key>ascending</key>
\t\t\t<true/>
\t\t\t<key>type</key>
\t\t\t<string>decompositionBase</string>
\t\t</dict>
\t</array>
\t<key>com.letterror.lightMeter.prefs</key>
\t<dict>
\t\t<key>chunkSize</key>
\t\t<integer>5</integer>
\t\t<key>diameter</key>
\t\t<integer>200</integer>
\t\t<key>drawTail</key>
\t\t<false/>
\t\t<key>invert</key>
\t\t<false/>
\t\t<key>toolDiameter</key>
\t\t<integer>30</integer>
\t\t<key>toolStyle</key>
\t\t<string>fluid</string>
\t</dict>
\t<key>com.typemytype.robofont.background.layerStrokeColor</key>
\t<array>
\t\t<real>0</real>
\t\t<real>0.8</real>
\t\t<real>0.2</real>
\t\t<real>0.7</real>
\t</array>
\t<key>com.typemytype.robofont.compileSettings.autohint</key>
\t<true/>
\t<key>com.typemytype.robofont.compileSettings.checkOutlines</key>
\t<false/>
\t<key>com.typemytype.robofont.compileSettings.createDummyDSIG</key>
\t<true/>
\t<key>com.typemytype.robofont.compileSettings.decompose</key>
\t<false/>
\t<key>com.typemytype.robofont.compileSettings.generateFormat</key>
\t<integer>0</integer>
\t<key>com.typemytype.robofont.compileSettings.releaseMode</key>
\t<false/>
\t<key>com.typemytype.robofont.foreground.layerStrokeColor</key>
\t<array>
\t\t<real>0.5</real>
\t\t<real>0</real>
\t\t<real>0.5</real>
\t\t<real>0.7</real>
\t</array>
\t<key>com.typemytype.robofont.italicSlantOffset</key>
\t<integer>0</integer>
\t<key>com.typemytype.robofont.segmentType</key>
\t<string>curve</string>
\t<key>com.typemytype.robofont.shouldAddPointsInSplineConversion</key>
\t<integer>1</integer>
\t<key>com.typesupply.defcon.sortDescriptor</key>
\t<array>
\t\t<dict>
\t\t\t<key>ascending</key>
\t\t\t<array>
\t\t\t\t<string>space</string>
\t\t\t\t<string>A</string>
\t\t\t\t<string>B</string>
\t\t\t\t<string>C</string>
\t\t\t\t<string>D</string>
\t\t\t\t<string>E</string>
\t\t\t\t<string>F</string>
\t\t\t\t<string>G</string>
\t\t\t\t<string>H</string>
\t\t\t\t<string>I</string>
\t\t\t\t<string>J</string>
\t\t\t\t<string>K</string>
\t\t\t\t<string>L</string>
\t\t\t\t<string>M</string>
\t\t\t\t<string>N</string>
\t\t\t\t<string>O</string>
\t\t\t\t<string>P</string>
\t\t\t\t<string>Q</string>
\t\t\t\t<string>R</string>
\t\t\t\t<string>S</string>
\t\t\t\t<string>T</string>
\t\t\t\t<string>U</string>
\t\t\t\t<string>V</string>
\t\t\t\t<string>W</string>
\t\t\t\t<string>X</string>
\t\t\t\t<string>Y</string>
\t\t\t\t<string>Z</string>
\t\t\t\t<string>a</string>
\t\t\t\t<string>b</string>
\t\t\t\t<string>c</string>
\t\t\t\t<string>d</string>
\t\t\t\t<string>e</string>
\t\t\t\t<string>f</string>
\t\t\t\t<string>g</string>
\t\t\t\t<string>h</string>
\t\t\t\t<string>i</string>
\t\t\t\t<string>j</string>
\t\t\t\t<string>k</string>
\t\t\t\t<string>l</string>
\t\t\t\t<string>m</string>
\t\t\t\t<string>n</string>
\t\t\t\t<string>ntilde</string>
\t\t\t\t<string>o</string>
\t\t\t\t<string>p</string>
\t\t\t\t<string>q</string>
\t\t\t\t<string>r</string>
\t\t\t\t<string>s</string>
\t\t\t\t<string>t</string>
\t\t\t\t<string>u</string>
\t\t\t\t<string>v</string>
\t\t\t\t<string>w</string>
\t\t\t\t<string>x</string>
\t\t\t\t<string>y</string>
\t\t\t\t<string>z</string>
\t\t\t\t<string>zcaron</string>
\t\t\t\t<string>zero</string>
\t\t\t\t<string>one</string>
\t\t\t\t<string>two</string>
\t\t\t\t<string>three</string>
\t\t\t\t<string>four</string>
\t\t\t\t<string>five</string>
\t\t\t\t<string>six</string>
\t\t\t\t<string>seven</string>
\t\t\t\t<string>eight</string>
\t\t\t\t<string>nine</string>
\t\t\t\t<string>underscore</string>
\t\t\t\t<string>hyphen</string>
\t\t\t\t<string>endash</string>
\t\t\t\t<string>emdash</string>
\t\t\t\t<string>parenleft</string>
\t\t\t\t<string>parenright</string>
\t\t\t\t<string>bracketleft</string>
\t\t\t\t<string>bracketright</string>
\t\t\t\t<string>braceleft</string>
\t\t\t\t<string>braceright</string>
\t\t\t\t<string>numbersign</string>
\t\t\t\t<string>percent</string>
\t\t\t\t<string>period</string>
\t\t\t\t<string>comma</string>
\t\t\t\t<string>colon</string>
\t\t\t\t<string>semicolon</string>
\t\t\t\t<string>exclam</string>
\t\t\t\t<string>question</string>
\t\t\t\t<string>slash</string>
\t\t\t\t<string>backslash</string>
\t\t\t\t<string>bar</string>
\t\t\t\t<string>at</string>
\t\t\t\t<string>ampersand</string>
\t\t\t\t<string>paragraph</string>
\t\t\t\t<string>bullet</string>
\t\t\t\t<string>dollar</string>
\t\t\t\t<string>trademark</string>
\t\t\t\t<string>fi</string>
\t\t\t\t<string>fl</string>
\t\t\t\t<string>.notdef</string>
\t\t\t\t<string>a_b_c</string>
\t\t\t\t<string>Atilde</string>
\t\t\t\t<string>Adieresis</string>
\t\t\t\t<string>Acircumflex</string>
\t\t\t\t<string>Aring</string>
\t\t\t\t<string>Ccedilla</string>
\t\t\t\t<string>Agrave</string>
\t\t\t\t<string>Aacute</string>
\t\t\t\t<string>quotedblright</string>
\t\t\t\t<string>quotedblleft</string>
\t\t\t</array>
\t\t\t<key>type</key>
\t\t\t<string>glyphList</string>
\t\t</dict>
\t</array>
\t<key>public.glyphOrder</key>
\t<array>
\t\t<string>A</string>
\t\t<string>Aacute</string>
\t\t<string>Adieresis</string>
\t\t<string>B</string>
\t\t<string>C</string>
\t\t<string>D</string>
\t\t<string>E</string>
\t\t<string>F</string>
\t\t<string>G</string>
\t\t<string>H</string>
\t\t<string>I</string>
\t\t<string>J</string>
\t\t<string>K</string>
\t\t<string>L</string>
\t\t<string>M</string>
\t\t<string>N</string>
\t\t<string>O</string>
\t\t<string>P</string>
\t\t<string>Q</string>
\t\t<string>R</string>
\t\t<string>S</string>
\t\t<string>T</string>
\t\t<string>U</string>
\t\t<string>V</string>
\t\t<string>W</string>
\t\t<string>X</string>
\t\t<string>Y</string>
\t\t<string>Z</string>
\t\t<string>IJ</string>
\t\t<string>S.closed</string>
\t\t<string>I.narrow</string>
\t\t<string>J.narrow</string>
\t\t<string>quotesinglbase</string>
\t\t<string>quotedblbase</string>
\t\t<string>quotedblleft</string>
\t\t<string>quotedblright</string>
\t\t<string>comma</string>
\t\t<string>period</string>
\t\t<string>colon</string>
\t\t<string>semicolon</string>
\t\t<string>dot</string>
\t\t<string>dieresis</string>
\t\t<string>acute</string>
\t\t<string>space</string>
\t\t<string>arrowdown</string>
\t\t<string>arrowleft</string>
\t\t<string>arrowright</string>
\t\t<string>arrowup</string>
\t</array>
</dict>
</plist>";

        // observed vs. expected string tests
        assert_eq!(expected_lib_string, test_lib_string);
    }

    #[test]
    fn test_format_default_metainfo_plist() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false);
        assert!(res_ufo_format.is_ok());

        // metainfo.plist
        let test_mi_string = fs::read_to_string(&test_ufo_path.join("metainfo.plist")).unwrap();

        let expected_mi_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
\t<key>creator</key>
\t<string>org.linebender.norad</string>
\t<key>formatVersion</key>
\t<integer>3</integer>
</dict>
</plist>";

        // observed vs. expected string tests
        assert_eq!(expected_mi_string, test_mi_string);
    }

    #[test]
    fn test_format_default_contents_plist() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false);
        assert!(res_ufo_format.is_ok());

        // glyphs/contents.plist
        let test_contents_string =
            fs::read_to_string(&test_ufo_path.join("glyphs").join("contents.plist")).unwrap();

        let expected_contents_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
\t<key>A</key>
\t<string>A_.glif</string>
\t<key>Aacute</key>
\t<string>A_acute.glif</string>
\t<key>Adieresis</key>
\t<string>A_dieresis.glif</string>
\t<key>B</key>
\t<string>B_.glif</string>
\t<key>C</key>
\t<string>C_.glif</string>
\t<key>D</key>
\t<string>D_.glif</string>
\t<key>E</key>
\t<string>E_.glif</string>
\t<key>F</key>
\t<string>F_.glif</string>
\t<key>G</key>
\t<string>G_.glif</string>
\t<key>H</key>
\t<string>H_.glif</string>
\t<key>I</key>
\t<string>I_.glif</string>
\t<key>I.narrow</key>
\t<string>I_.narrow.glif</string>
\t<key>IJ</key>
\t<string>I_J_.glif</string>
\t<key>J</key>
\t<string>J_.glif</string>
\t<key>J.narrow</key>
\t<string>J_.narrow.glif</string>
\t<key>K</key>
\t<string>K_.glif</string>
\t<key>L</key>
\t<string>L_.glif</string>
\t<key>M</key>
\t<string>M_.glif</string>
\t<key>N</key>
\t<string>N_.glif</string>
\t<key>O</key>
\t<string>O_.glif</string>
\t<key>P</key>
\t<string>P_.glif</string>
\t<key>Q</key>
\t<string>Q_.glif</string>
\t<key>R</key>
\t<string>R_.glif</string>
\t<key>S</key>
\t<string>S_.glif</string>
\t<key>S.closed</key>
\t<string>S_.closed.glif</string>
\t<key>T</key>
\t<string>T_.glif</string>
\t<key>U</key>
\t<string>U_.glif</string>
\t<key>V</key>
\t<string>V_.glif</string>
\t<key>W</key>
\t<string>W_.glif</string>
\t<key>X</key>
\t<string>X_.glif</string>
\t<key>Y</key>
\t<string>Y_.glif</string>
\t<key>Z</key>
\t<string>Z_.glif</string>
\t<key>acute</key>
\t<string>acute.glif</string>
\t<key>arrowdown</key>
\t<string>arrowdown.glif</string>
\t<key>arrowleft</key>
\t<string>arrowleft.glif</string>
\t<key>arrowright</key>
\t<string>arrowright.glif</string>
\t<key>arrowup</key>
\t<string>arrowup.glif</string>
\t<key>colon</key>
\t<string>colon.glif</string>
\t<key>comma</key>
\t<string>comma.glif</string>
\t<key>dieresis</key>
\t<string>dieresis.glif</string>
\t<key>dot</key>
\t<string>dot.glif</string>
\t<key>period</key>
\t<string>period.glif</string>
\t<key>quotedblbase</key>
\t<string>quotedblbase.glif</string>
\t<key>quotedblleft</key>
\t<string>quotedblleft.glif</string>
\t<key>quotedblright</key>
\t<string>quotedblright.glif</string>
\t<key>quotesinglbase</key>
\t<string>quotesinglbase.glif</string>
\t<key>semicolon</key>
\t<string>semicolon.glif</string>
\t<key>space</key>
\t<string>space.glif</string>
</dict>
</plist>";

        // observed vs. expected string tests
        assert_eq!(expected_contents_string, test_contents_string);
    }

    // XML declaration optional attribute single quote formatting
    #[test]
    fn test_format_singlequote_glif() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, true);
        assert!(res_ufo_format.is_ok());
        let test_glyph_string =
            fs::read_to_string(&test_ufo_path.join("glyphs").join("A_.glif")).unwrap();

        let expected_glyph_string = "<?xml version='1.0' encoding='UTF-8'?>
<glyph name=\"A\" format=\"2\">
\t<unicode hex=\"0041\"/>
\t<advance width=\"740\"/>
\t<outline>
\t\t<contour>
\t\t\t<point x=\"-10\" y=\"0\" type=\"line\"/>
\t\t\t<point x=\"250\" y=\"0\" type=\"line\"/>
\t\t\t<point x=\"334\" y=\"800\" type=\"line\"/>
\t\t\t<point x=\"104\" y=\"800\" type=\"line\"/>
\t\t</contour>
\t\t<contour>
\t\t\t<point x=\"110\" y=\"120\" type=\"line\"/>
\t\t\t<point x=\"580\" y=\"120\" type=\"line\"/>
\t\t\t<point x=\"580\" y=\"330\" type=\"line\"/>
\t\t\t<point x=\"110\" y=\"330\" type=\"line\"/>
\t\t</contour>
\t\t<contour>
\t\t\t<point x=\"390\" y=\"0\" type=\"line\"/>
\t\t\t<point x=\"730\" y=\"0\" type=\"line\"/>
\t\t\t<point x=\"614\" y=\"800\" type=\"line\"/>
\t\t\t<point x=\"294\" y=\"800\" type=\"line\"/>
\t\t</contour>
\t\t<contour>
\t\t\t<point x=\"204\" y=\"540\" type=\"line\"/>
\t\t\t<point x=\"474\" y=\"540\" type=\"line\"/>
\t\t\t<point x=\"474\" y=\"800\" type=\"line\"/>
\t\t\t<point x=\"204\" y=\"800\" type=\"line\"/>
\t\t</contour>
\t</outline>
\t<lib>
\t\t<dict>
\t\t\t<key>com.typemytype.robofont.Image.Brightness</key>
\t\t\t<integer>0</integer>
\t\t\t<key>com.typemytype.robofont.Image.Contrast</key>
\t\t\t<integer>1</integer>
\t\t\t<key>com.typemytype.robofont.Image.Saturation</key>
\t\t\t<integer>1</integer>
\t\t\t<key>com.typemytype.robofont.Image.Sharpness</key>
\t\t\t<real>0.4</real>
\t\t</dict>
\t</lib>
</glyph>
";

        // observed vs. expected string tests
        assert_eq!(expected_glyph_string, test_glyph_string);
    }
}
