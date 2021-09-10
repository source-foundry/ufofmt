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
    indent_with_space: bool,
    indent_number: u8,
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

    // define the indentation spacing format based on user CL options
    let indentation_str = get_indent_str(indent_with_space, indent_number);

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

            let options =
                WriteOptions::default().whitespace(indentation_str).quote_char(quote_style);
            // Execute serialization with options
            match ufo.save_with_options(&outpath, &options) {
                Ok(_) => Ok(outpath),
                Err(e) => Err(Error::NoradWrite(outpath, e)),
            }
        }
        Err(e) => Err(Error::NoradRead(ufopath.into(), e)),
    }
}

fn get_indent_str(indent_with_space: bool, indent_number: u8) -> &'static str {
    match (indent_with_space, indent_number) {
        (false, 1) => "\t",
        (false, 2) => "\t\t",
        (false, 3) => "\t\t\t",
        (false, 4) => "\t\t\t\t",
        (true, 1) => " ",
        (true, 2) => "  ",
        (true, 3) => "   ",
        (true, 4) => "    ",
        (_, _) => panic!("unsupported indentation definition"),
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
        let res = format_ufo(invalid_path, &None, &None, false, false, 1);
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
        let res = format_ufo(
            invalid_path,
            &Some("_new".to_string()),
            &Some(".test".to_string()),
            false,
            false,
            1,
        );
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
        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, false, 1);
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
        let res_ufo_format = format_ufo(
            &test_ufo_path,
            &Some("_new".to_string()),
            &Some("test".to_string()),
            false,
            false,
            1,
        );
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

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, false, 1);
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

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, false, 1);
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

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, false, 1);
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

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, false, 1);
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

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, false, 1);
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

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, false, 1);
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

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, false, 1);
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

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, false, 1);
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

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, true, false, 1);
        assert!(res_ufo_format.is_ok());
        let test_glyph_string =
            fs::read_to_string(&test_ufo_path.join("glyphs").join("A_.glif")).unwrap();
        // should use single quotes
        assert!(test_glyph_string.starts_with("<?xml version='1.0' encoding='UTF-8'?>"));
    }

    #[test]
    fn test_format_singlequote_fontinfo_plist() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, true, false, 1);
        assert!(res_ufo_format.is_ok());
        let test_fontinfo_string =
            fs::read_to_string(&test_ufo_path.join("fontinfo.plist")).unwrap();
        // should use single quotes
        assert!(test_fontinfo_string.starts_with("<?xml version='1.0' encoding='UTF-8'?>"));
    }

    #[test]
    fn test_format_singlequote_lib_plist() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, true, false, 1);
        assert!(res_ufo_format.is_ok());
        let test_fontinfo_string = fs::read_to_string(&test_ufo_path.join("lib.plist")).unwrap();
        // should use single quotes
        assert!(test_fontinfo_string.starts_with("<?xml version='1.0' encoding='UTF-8'?>"));
    }

    // Indentation spacing format tests
    #[test]
    fn test_get_indent_str() {
        let onetab = get_indent_str(false, 1);
        let twotabs = get_indent_str(false, 2);
        let threetabs = get_indent_str(false, 3);
        let fourtabs = get_indent_str(false, 4);
        let onespace = get_indent_str(true, 1);
        let twospaces = get_indent_str(true, 2);
        let threespaces = get_indent_str(true, 3);
        let fourspaces = get_indent_str(true, 4);

        assert_eq!(onetab, "\t");
        assert_eq!(twotabs, "\t\t");
        assert_eq!(threetabs, "\t\t\t");
        assert_eq!(fourtabs, "\t\t\t\t");
        assert_eq!(onespace, " ");
        assert_eq!(twospaces, "  ");
        assert_eq!(threespaces, "   ");
        assert_eq!(fourspaces, "    ");
    }

    #[test]
    fn test_format_indent_twotab_glif() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, false, 2);
        assert!(res_ufo_format.is_ok());

        // glif file
        let test_glyph_string =
            fs::read_to_string(&test_ufo_path.join("glyphs").join("A_.glif")).unwrap();
        let expected_glyph_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<glyph name=\"A\" format=\"2\">
\t\t<unicode hex=\"0041\"/>
\t\t<advance width=\"740\"/>
\t\t<outline>
\t\t\t\t<contour>
\t\t\t\t\t\t<point x=\"-10\" y=\"0\" type=\"line\"/>
\t\t\t\t\t\t<point x=\"250\" y=\"0\" type=\"line\"/>
\t\t\t\t\t\t<point x=\"334\" y=\"800\" type=\"line\"/>
\t\t\t\t\t\t<point x=\"104\" y=\"800\" type=\"line\"/>
\t\t\t\t</contour>
\t\t\t\t<contour>
\t\t\t\t\t\t<point x=\"110\" y=\"120\" type=\"line\"/>
\t\t\t\t\t\t<point x=\"580\" y=\"120\" type=\"line\"/>
\t\t\t\t\t\t<point x=\"580\" y=\"330\" type=\"line\"/>
\t\t\t\t\t\t<point x=\"110\" y=\"330\" type=\"line\"/>
\t\t\t\t</contour>
\t\t\t\t<contour>
\t\t\t\t\t\t<point x=\"390\" y=\"0\" type=\"line\"/>
\t\t\t\t\t\t<point x=\"730\" y=\"0\" type=\"line\"/>
\t\t\t\t\t\t<point x=\"614\" y=\"800\" type=\"line\"/>
\t\t\t\t\t\t<point x=\"294\" y=\"800\" type=\"line\"/>
\t\t\t\t</contour>
\t\t\t\t<contour>
\t\t\t\t\t\t<point x=\"204\" y=\"540\" type=\"line\"/>
\t\t\t\t\t\t<point x=\"474\" y=\"540\" type=\"line\"/>
\t\t\t\t\t\t<point x=\"474\" y=\"800\" type=\"line\"/>
\t\t\t\t\t\t<point x=\"204\" y=\"800\" type=\"line\"/>
\t\t\t\t</contour>
\t\t</outline>
\t\t<lib>
\t\t\t\t<dict>
\t\t\t\t\t\t<key>com.typemytype.robofont.Image.Brightness</key>
\t\t\t\t\t\t<integer>0</integer>
\t\t\t\t\t\t<key>com.typemytype.robofont.Image.Contrast</key>
\t\t\t\t\t\t<integer>1</integer>
\t\t\t\t\t\t<key>com.typemytype.robofont.Image.Saturation</key>
\t\t\t\t\t\t<integer>1</integer>
\t\t\t\t\t\t<key>com.typemytype.robofont.Image.Sharpness</key>
\t\t\t\t\t\t<real>0.4</real>
\t\t\t\t</dict>
\t\t</lib>
</glyph>
";

        assert_eq!(expected_glyph_string, test_glyph_string);
    }

    #[test]
    fn test_format_indent_singlespace_glif() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, true, 1);
        assert!(res_ufo_format.is_ok());

        // glif file
        let test_glyph_string =
            fs::read_to_string(&test_ufo_path.join("glyphs").join("A_.glif")).unwrap();
        let expected_glyph_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<glyph name=\"A\" format=\"2\">
 <unicode hex=\"0041\"/>
 <advance width=\"740\"/>
 <outline>
  <contour>
   <point x=\"-10\" y=\"0\" type=\"line\"/>
   <point x=\"250\" y=\"0\" type=\"line\"/>
   <point x=\"334\" y=\"800\" type=\"line\"/>
   <point x=\"104\" y=\"800\" type=\"line\"/>
  </contour>
  <contour>
   <point x=\"110\" y=\"120\" type=\"line\"/>
   <point x=\"580\" y=\"120\" type=\"line\"/>
   <point x=\"580\" y=\"330\" type=\"line\"/>
   <point x=\"110\" y=\"330\" type=\"line\"/>
  </contour>
  <contour>
   <point x=\"390\" y=\"0\" type=\"line\"/>
   <point x=\"730\" y=\"0\" type=\"line\"/>
   <point x=\"614\" y=\"800\" type=\"line\"/>
   <point x=\"294\" y=\"800\" type=\"line\"/>
  </contour>
  <contour>
   <point x=\"204\" y=\"540\" type=\"line\"/>
   <point x=\"474\" y=\"540\" type=\"line\"/>
   <point x=\"474\" y=\"800\" type=\"line\"/>
   <point x=\"204\" y=\"800\" type=\"line\"/>
  </contour>
 </outline>
 <lib>
  <dict>
   <key>com.typemytype.robofont.Image.Brightness</key>
   <integer>0</integer>
   <key>com.typemytype.robofont.Image.Contrast</key>
   <integer>1</integer>
   <key>com.typemytype.robofont.Image.Saturation</key>
   <integer>1</integer>
   <key>com.typemytype.robofont.Image.Sharpness</key>
   <real>0.4</real>
  </dict>
 </lib>
</glyph>
";

        // observed vs. expected string tests
        assert_eq!(expected_glyph_string, test_glyph_string);
    }

    #[test]
    fn test_format_indent_fourspace_glif() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, true, 4);
        assert!(res_ufo_format.is_ok());

        // glif file
        let test_glyph_string =
            fs::read_to_string(&test_ufo_path.join("glyphs").join("A_.glif")).unwrap();
        let expected_glyph_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<glyph name=\"A\" format=\"2\">
    <unicode hex=\"0041\"/>
    <advance width=\"740\"/>
    <outline>
        <contour>
            <point x=\"-10\" y=\"0\" type=\"line\"/>
            <point x=\"250\" y=\"0\" type=\"line\"/>
            <point x=\"334\" y=\"800\" type=\"line\"/>
            <point x=\"104\" y=\"800\" type=\"line\"/>
        </contour>
        <contour>
            <point x=\"110\" y=\"120\" type=\"line\"/>
            <point x=\"580\" y=\"120\" type=\"line\"/>
            <point x=\"580\" y=\"330\" type=\"line\"/>
            <point x=\"110\" y=\"330\" type=\"line\"/>
        </contour>
        <contour>
            <point x=\"390\" y=\"0\" type=\"line\"/>
            <point x=\"730\" y=\"0\" type=\"line\"/>
            <point x=\"614\" y=\"800\" type=\"line\"/>
            <point x=\"294\" y=\"800\" type=\"line\"/>
        </contour>
        <contour>
            <point x=\"204\" y=\"540\" type=\"line\"/>
            <point x=\"474\" y=\"540\" type=\"line\"/>
            <point x=\"474\" y=\"800\" type=\"line\"/>
            <point x=\"204\" y=\"800\" type=\"line\"/>
        </contour>
    </outline>
    <lib>
        <dict>
            <key>com.typemytype.robofont.Image.Brightness</key>
            <integer>0</integer>
            <key>com.typemytype.robofont.Image.Contrast</key>
            <integer>1</integer>
            <key>com.typemytype.robofont.Image.Saturation</key>
            <integer>1</integer>
            <key>com.typemytype.robofont.Image.Sharpness</key>
            <real>0.4</real>
        </dict>
    </lib>
</glyph>
";

        // observed vs. expected string tests
        assert_eq!(expected_glyph_string, test_glyph_string);
    }

    #[test]
    fn test_format_indent_threetab_fontinfo_plist() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, false, 3);
        assert!(res_ufo_format.is_ok());

        // fontinfo.plist
        let test_fontinfo_string =
            fs::read_to_string(&test_ufo_path.join("fontinfo.plist")).unwrap();

        let expected_fontinfo_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
\t\t\t<key>ascender</key>
\t\t\t<integer>800</integer>
\t\t\t<key>capHeight</key>
\t\t\t<integer>800</integer>
\t\t\t<key>copyright</key>
\t\t\t<string>License same as MutatorMath. BSD 3-clause. [test-token: A]</string>
\t\t\t<key>descender</key>
\t\t\t<integer>-200</integer>
\t\t\t<key>familyName</key>
\t\t\t<string>MutatorMathTest</string>
\t\t\t<key>guidelines</key>
\t\t\t<array/>
\t\t\t<key>italicAngle</key>
\t\t\t<integer>0</integer>
\t\t\t<key>openTypeNameLicense</key>
\t\t\t<string>License same as MutatorMath. BSD 3-clause. [test-token: A]</string>
\t\t\t<key>openTypeOS2VendorID</key>
\t\t\t<string>LTTR</string>
\t\t\t<key>postscriptBlueValues</key>
\t\t\t<array>
\t\t\t\t\t\t<integer>-10</integer>
\t\t\t\t\t\t<integer>0</integer>
\t\t\t\t\t\t<integer>800</integer>
\t\t\t\t\t\t<integer>810</integer>
\t\t\t</array>
\t\t\t<key>postscriptDefaultWidthX</key>
\t\t\t<integer>500</integer>
\t\t\t<key>postscriptFamilyBlues</key>
\t\t\t<array/>
\t\t\t<key>postscriptFamilyOtherBlues</key>
\t\t\t<array/>
\t\t\t<key>postscriptFontName</key>
\t\t\t<string>MutatorMathTest-BoldCondensed</string>
\t\t\t<key>postscriptFullName</key>
\t\t\t<string>MutatorMathTest BoldCondensed</string>
\t\t\t<key>postscriptOtherBlues</key>
\t\t\t<array>
\t\t\t\t\t\t<integer>500</integer>
\t\t\t\t\t\t<integer>520</integer>
\t\t\t</array>
\t\t\t<key>postscriptSlantAngle</key>
\t\t\t<integer>0</integer>
\t\t\t<key>postscriptStemSnapH</key>
\t\t\t<array/>
\t\t\t<key>postscriptStemSnapV</key>
\t\t\t<array/>
\t\t\t<key>postscriptWindowsCharacterSet</key>
\t\t\t<integer>1</integer>
\t\t\t<key>styleMapFamilyName</key>
\t\t\t<string></string>
\t\t\t<key>styleMapStyleName</key>
\t\t\t<string>regular</string>
\t\t\t<key>styleName</key>
\t\t\t<string>BoldCondensed</string>
\t\t\t<key>unitsPerEm</key>
\t\t\t<integer>1000</integer>
\t\t\t<key>versionMajor</key>
\t\t\t<integer>1</integer>
\t\t\t<key>versionMinor</key>
\t\t\t<integer>2</integer>
\t\t\t<key>xHeight</key>
\t\t\t<integer>500</integer>
\t\t\t<key>year</key>
\t\t\t<integer>2004</integer>
</dict>
</plist>";

        // observed vs. expected string tests
        assert_eq!(expected_fontinfo_string, test_fontinfo_string);
    }

    #[test]
    fn test_format_indent_twospace_fontinfo_plist() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, true, 2);
        assert!(res_ufo_format.is_ok());

        // fontinfo.plist
        let test_fontinfo_string =
            fs::read_to_string(&test_ufo_path.join("fontinfo.plist")).unwrap();

        let expected_fontinfo_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
  <key>ascender</key>
  <integer>800</integer>
  <key>capHeight</key>
  <integer>800</integer>
  <key>copyright</key>
  <string>License same as MutatorMath. BSD 3-clause. [test-token: A]</string>
  <key>descender</key>
  <integer>-200</integer>
  <key>familyName</key>
  <string>MutatorMathTest</string>
  <key>guidelines</key>
  <array/>
  <key>italicAngle</key>
  <integer>0</integer>
  <key>openTypeNameLicense</key>
  <string>License same as MutatorMath. BSD 3-clause. [test-token: A]</string>
  <key>openTypeOS2VendorID</key>
  <string>LTTR</string>
  <key>postscriptBlueValues</key>
  <array>
    <integer>-10</integer>
    <integer>0</integer>
    <integer>800</integer>
    <integer>810</integer>
  </array>
  <key>postscriptDefaultWidthX</key>
  <integer>500</integer>
  <key>postscriptFamilyBlues</key>
  <array/>
  <key>postscriptFamilyOtherBlues</key>
  <array/>
  <key>postscriptFontName</key>
  <string>MutatorMathTest-BoldCondensed</string>
  <key>postscriptFullName</key>
  <string>MutatorMathTest BoldCondensed</string>
  <key>postscriptOtherBlues</key>
  <array>
    <integer>500</integer>
    <integer>520</integer>
  </array>
  <key>postscriptSlantAngle</key>
  <integer>0</integer>
  <key>postscriptStemSnapH</key>
  <array/>
  <key>postscriptStemSnapV</key>
  <array/>
  <key>postscriptWindowsCharacterSet</key>
  <integer>1</integer>
  <key>styleMapFamilyName</key>
  <string></string>
  <key>styleMapStyleName</key>
  <string>regular</string>
  <key>styleName</key>
  <string>BoldCondensed</string>
  <key>unitsPerEm</key>
  <integer>1000</integer>
  <key>versionMajor</key>
  <integer>1</integer>
  <key>versionMinor</key>
  <integer>2</integer>
  <key>xHeight</key>
  <integer>500</integer>
  <key>year</key>
  <integer>2004</integer>
</dict>
</plist>";

        // observed vs. expected string tests
        assert_eq!(expected_fontinfo_string, test_fontinfo_string);
    }

    #[test]
    fn test_format_indent_twotabs_groups_plist() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, false, 2);
        assert!(res_ufo_format.is_ok());

        // groups.plist
        let test_groups_string = fs::read_to_string(&test_ufo_path.join("groups.plist")).unwrap();
        let expected_groups_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
\t\t<key>public.kern1.@MMK_L_A</key>
\t\t<array>
\t\t\t\t<string>A</string>
\t\t</array>
\t\t<key>public.kern2.@MMK_R_A</key>
\t\t<array>
\t\t\t\t<string>A</string>
\t\t</array>
\t\t<key>testGroup</key>
\t\t<array>
\t\t\t\t<string>E</string>
\t\t\t\t<string>F</string>
\t\t\t\t<string>H</string>
\t\t</array>
</dict>
</plist>";

        // observed vs. expected string tests
        assert_eq!(expected_groups_string, test_groups_string);
    }

    #[test]
    fn test_format_indent_fourspace_groups_plist() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, true, 4);
        assert!(res_ufo_format.is_ok());

        // groups.plist
        let test_groups_string = fs::read_to_string(&test_ufo_path.join("groups.plist")).unwrap();
        let expected_groups_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
    <key>public.kern1.@MMK_L_A</key>
    <array>
        <string>A</string>
    </array>
    <key>public.kern2.@MMK_R_A</key>
    <array>
        <string>A</string>
    </array>
    <key>testGroup</key>
    <array>
        <string>E</string>
        <string>F</string>
        <string>H</string>
    </array>
</dict>
</plist>";

        // observed vs. expected string tests
        assert_eq!(expected_groups_string, test_groups_string);
    }

    #[test]
    fn test_format_indent_twotab_kerning_plist() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, false, 2);
        assert!(res_ufo_format.is_ok());

        // kerning.plist
        let test_kerning_string = fs::read_to_string(&test_ufo_path.join("kerning.plist")).unwrap();

        let expected_kerning_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
\t\t<key>A</key>
\t\t<dict>
\t\t\t\t<key>J</key>
\t\t\t\t<integer>-20</integer>
\t\t\t\t<key>O</key>
\t\t\t\t<integer>-30</integer>
\t\t\t\t<key>T</key>
\t\t\t\t<integer>-70</integer>
\t\t\t\t<key>U</key>
\t\t\t\t<integer>-30</integer>
\t\t\t\t<key>V</key>
\t\t\t\t<integer>-50</integer>
\t\t</dict>
\t\t<key>B</key>
\t\t<dict>
\t\t\t\t<key>A</key>
\t\t\t\t<integer>-20</integer>
\t\t\t\t<key>J</key>
\t\t\t\t<integer>-50</integer>
\t\t\t\t<key>O</key>
\t\t\t\t<integer>-20</integer>
\t\t\t\t<key>S</key>
\t\t\t\t<integer>-10</integer>
\t\t\t\t<key>T</key>
\t\t\t\t<integer>-10</integer>
\t\t\t\t<key>U</key>
\t\t\t\t<integer>-20</integer>
\t\t\t\t<key>V</key>
\t\t\t\t<integer>-30</integer>
\t\t</dict>
\t\t<key>C</key>
\t\t<dict>
\t\t\t\t<key>A</key>
\t\t\t\t<integer>-20</integer>
\t\t\t\t<key>J</key>
\t\t\t\t<integer>-50</integer>
\t\t\t\t<key>T</key>
\t\t\t\t<integer>-20</integer>
\t\t\t\t<key>V</key>
\t\t\t\t<integer>-20</integer>
\t\t</dict>
\t\t<key>E</key>
\t\t<dict>
\t\t\t\t<key>J</key>
\t\t\t\t<integer>-20</integer>
\t\t\t\t<key>T</key>
\t\t\t\t<integer>-10</integer>
\t\t\t\t<key>V</key>
\t\t\t\t<integer>-10</integer>
\t\t</dict>
\t\t<key>F</key>
\t\t<dict>
\t\t\t\t<key>A</key>
\t\t\t\t<integer>-40</integer>
\t\t\t\t<key>J</key>
\t\t\t\t<integer>-80</integer>
\t\t\t\t<key>O</key>
\t\t\t\t<integer>-10</integer>
\t\t\t\t<key>S</key>
\t\t\t\t<integer>-20</integer>
\t\t\t\t<key>U</key>
\t\t\t\t<integer>-10</integer>
\t\t\t\t<key>V</key>
\t\t\t\t<integer>-10</integer>
\t\t</dict>
\t\t<key>G</key>
\t\t<dict>
\t\t\t\t<key>J</key>
\t\t\t\t<integer>-20</integer>
\t\t\t\t<key>S</key>
\t\t\t\t<integer>-10</integer>
\t\t\t\t<key>T</key>
\t\t\t\t<integer>-40</integer>
\t\t\t\t<key>U</key>
\t\t\t\t<integer>-10</integer>
\t\t\t\t<key>V</key>
\t\t\t\t<integer>-30</integer>
\t\t</dict>
\t\t<key>H</key>
\t\t<dict>
\t\t\t\t<key>J</key>
\t\t\t\t<integer>-30</integer>
\t\t\t\t<key>S</key>
\t\t\t\t<integer>-10</integer>
\t\t\t\t<key>T</key>
\t\t\t\t<integer>-10</integer>
\t\t</dict>
\t\t<key>J</key>
\t\t<dict>
\t\t\t\t<key>J</key>
\t\t\t\t<integer>-70</integer>
\t\t</dict>
\t\t<key>L</key>
\t\t<dict>
\t\t\t\t<key>J</key>
\t\t\t\t<integer>-20</integer>
\t\t\t\t<key>O</key>
\t\t\t\t<integer>-20</integer>
\t\t\t\t<key>T</key>
\t\t\t\t<integer>-110</integer>
\t\t\t\t<key>U</key>
\t\t\t\t<integer>-20</integer>
\t\t\t\t<key>V</key>
\t\t\t\t<integer>-60</integer>
\t\t</dict>
\t\t<key>O</key>
\t\t<dict>
\t\t\t\t<key>A</key>
\t\t\t\t<integer>-30</integer>
\t\t\t\t<key>J</key>
\t\t\t\t<integer>-60</integer>
\t\t\t\t<key>S</key>
\t\t\t\t<integer>-10</integer>
\t\t\t\t<key>T</key>
\t\t\t\t<integer>-30</integer>
\t\t\t\t<key>V</key>
\t\t\t\t<integer>-30</integer>
\t\t</dict>
\t\t<key>P</key>
\t\t<dict>
\t\t\t\t<key>A</key>
\t\t\t\t<integer>-50</integer>
\t\t\t\t<key>J</key>
\t\t\t\t<integer>-100</integer>
\t\t\t\t<key>S</key>
\t\t\t\t<integer>-10</integer>
\t\t\t\t<key>T</key>
\t\t\t\t<integer>-10</integer>
\t\t\t\t<key>U</key>
\t\t\t\t<integer>-10</integer>
\t\t\t\t<key>V</key>
\t\t\t\t<integer>-20</integer>
\t\t</dict>
\t\t<key>R</key>
\t\t<dict>
\t\t\t\t<key>H</key>
\t\t\t\t<integer>-10</integer>
\t\t\t\t<key>J</key>
\t\t\t\t<integer>-20</integer>
\t\t\t\t<key>O</key>
\t\t\t\t<integer>-30</integer>
\t\t\t\t<key>S</key>
\t\t\t\t<integer>-20</integer>
\t\t\t\t<key>T</key>
\t\t\t\t<integer>-30</integer>
\t\t\t\t<key>U</key>
\t\t\t\t<integer>-30</integer>
\t\t\t\t<key>V</key>
\t\t\t\t<integer>-40</integer>
\t\t</dict>
\t\t<key>S</key>
\t\t<dict>
\t\t\t\t<key>A</key>
\t\t\t\t<integer>-20</integer>
\t\t\t\t<key>H</key>
\t\t\t\t<integer>-20</integer>
\t\t\t\t<key>J</key>
\t\t\t\t<integer>-40</integer>
\t\t\t\t<key>O</key>
\t\t\t\t<integer>-10</integer>
\t\t\t\t<key>S</key>
\t\t\t\t<integer>-10</integer>
\t\t\t\t<key>T</key>
\t\t\t\t<integer>-30</integer>
\t\t\t\t<key>U</key>
\t\t\t\t<integer>-10</integer>
\t\t\t\t<key>V</key>
\t\t\t\t<integer>-30</integer>
\t\t\t\t<key>W</key>
\t\t\t\t<integer>-10</integer>
\t\t</dict>
\t\t<key>T</key>
\t\t<dict>
\t\t\t\t<key>A</key>
\t\t\t\t<integer>-65</integer>
\t\t\t\t<key>H</key>
\t\t\t\t<integer>-10</integer>
\t\t\t\t<key>J</key>
\t\t\t\t<integer>-130</integer>
\t\t\t\t<key>O</key>
\t\t\t\t<integer>-20</integer>
\t\t</dict>
\t\t<key>U</key>
\t\t<dict>
\t\t\t\t<key>A</key>
\t\t\t\t<integer>-30</integer>
\t\t\t\t<key>J</key>
\t\t\t\t<integer>-60</integer>
\t\t\t\t<key>S</key>
\t\t\t\t<integer>-10</integer>
\t\t\t\t<key>V</key>
\t\t\t\t<integer>-10</integer>
\t\t</dict>
\t\t<key>V</key>
\t\t<dict>
\t\t\t\t<key>J</key>
\t\t\t\t<integer>-100</integer>
\t\t\t\t<key>O</key>
\t\t\t\t<integer>-30</integer>
\t\t\t\t<key>S</key>
\t\t\t\t<integer>-20</integer>
\t\t\t\t<key>U</key>
\t\t\t\t<integer>-10</integer>
\t\t</dict>
</dict>
</plist>";

        // observed vs. expected string tests
        assert_eq!(expected_kerning_string, test_kerning_string);
    }

    #[test]
    fn test_format_indent_fourspace_kerning_plist() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, true, 4);
        assert!(res_ufo_format.is_ok());

        // kerning.plist
        let test_kerning_string = fs::read_to_string(&test_ufo_path.join("kerning.plist")).unwrap();

        let expected_kerning_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
    <key>A</key>
    <dict>
        <key>J</key>
        <integer>-20</integer>
        <key>O</key>
        <integer>-30</integer>
        <key>T</key>
        <integer>-70</integer>
        <key>U</key>
        <integer>-30</integer>
        <key>V</key>
        <integer>-50</integer>
    </dict>
    <key>B</key>
    <dict>
        <key>A</key>
        <integer>-20</integer>
        <key>J</key>
        <integer>-50</integer>
        <key>O</key>
        <integer>-20</integer>
        <key>S</key>
        <integer>-10</integer>
        <key>T</key>
        <integer>-10</integer>
        <key>U</key>
        <integer>-20</integer>
        <key>V</key>
        <integer>-30</integer>
    </dict>
    <key>C</key>
    <dict>
        <key>A</key>
        <integer>-20</integer>
        <key>J</key>
        <integer>-50</integer>
        <key>T</key>
        <integer>-20</integer>
        <key>V</key>
        <integer>-20</integer>
    </dict>
    <key>E</key>
    <dict>
        <key>J</key>
        <integer>-20</integer>
        <key>T</key>
        <integer>-10</integer>
        <key>V</key>
        <integer>-10</integer>
    </dict>
    <key>F</key>
    <dict>
        <key>A</key>
        <integer>-40</integer>
        <key>J</key>
        <integer>-80</integer>
        <key>O</key>
        <integer>-10</integer>
        <key>S</key>
        <integer>-20</integer>
        <key>U</key>
        <integer>-10</integer>
        <key>V</key>
        <integer>-10</integer>
    </dict>
    <key>G</key>
    <dict>
        <key>J</key>
        <integer>-20</integer>
        <key>S</key>
        <integer>-10</integer>
        <key>T</key>
        <integer>-40</integer>
        <key>U</key>
        <integer>-10</integer>
        <key>V</key>
        <integer>-30</integer>
    </dict>
    <key>H</key>
    <dict>
        <key>J</key>
        <integer>-30</integer>
        <key>S</key>
        <integer>-10</integer>
        <key>T</key>
        <integer>-10</integer>
    </dict>
    <key>J</key>
    <dict>
        <key>J</key>
        <integer>-70</integer>
    </dict>
    <key>L</key>
    <dict>
        <key>J</key>
        <integer>-20</integer>
        <key>O</key>
        <integer>-20</integer>
        <key>T</key>
        <integer>-110</integer>
        <key>U</key>
        <integer>-20</integer>
        <key>V</key>
        <integer>-60</integer>
    </dict>
    <key>O</key>
    <dict>
        <key>A</key>
        <integer>-30</integer>
        <key>J</key>
        <integer>-60</integer>
        <key>S</key>
        <integer>-10</integer>
        <key>T</key>
        <integer>-30</integer>
        <key>V</key>
        <integer>-30</integer>
    </dict>
    <key>P</key>
    <dict>
        <key>A</key>
        <integer>-50</integer>
        <key>J</key>
        <integer>-100</integer>
        <key>S</key>
        <integer>-10</integer>
        <key>T</key>
        <integer>-10</integer>
        <key>U</key>
        <integer>-10</integer>
        <key>V</key>
        <integer>-20</integer>
    </dict>
    <key>R</key>
    <dict>
        <key>H</key>
        <integer>-10</integer>
        <key>J</key>
        <integer>-20</integer>
        <key>O</key>
        <integer>-30</integer>
        <key>S</key>
        <integer>-20</integer>
        <key>T</key>
        <integer>-30</integer>
        <key>U</key>
        <integer>-30</integer>
        <key>V</key>
        <integer>-40</integer>
    </dict>
    <key>S</key>
    <dict>
        <key>A</key>
        <integer>-20</integer>
        <key>H</key>
        <integer>-20</integer>
        <key>J</key>
        <integer>-40</integer>
        <key>O</key>
        <integer>-10</integer>
        <key>S</key>
        <integer>-10</integer>
        <key>T</key>
        <integer>-30</integer>
        <key>U</key>
        <integer>-10</integer>
        <key>V</key>
        <integer>-30</integer>
        <key>W</key>
        <integer>-10</integer>
    </dict>
    <key>T</key>
    <dict>
        <key>A</key>
        <integer>-65</integer>
        <key>H</key>
        <integer>-10</integer>
        <key>J</key>
        <integer>-130</integer>
        <key>O</key>
        <integer>-20</integer>
    </dict>
    <key>U</key>
    <dict>
        <key>A</key>
        <integer>-30</integer>
        <key>J</key>
        <integer>-60</integer>
        <key>S</key>
        <integer>-10</integer>
        <key>V</key>
        <integer>-10</integer>
    </dict>
    <key>V</key>
    <dict>
        <key>J</key>
        <integer>-100</integer>
        <key>O</key>
        <integer>-30</integer>
        <key>S</key>
        <integer>-20</integer>
        <key>U</key>
        <integer>-10</integer>
    </dict>
</dict>
</plist>";

        // observed vs. expected string tests
        assert_eq!(expected_kerning_string, test_kerning_string);
    }

    #[test]
    fn test_format_indent_twotabs_layercontents_plist() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, false, 2);
        assert!(res_ufo_format.is_ok());

        // layercontents.plist
        let test_lc_string =
            fs::read_to_string(&test_ufo_path.join("layercontents.plist")).unwrap();
        let expected_lc_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<array>
\t\t<array>
\t\t\t\t<string>foreground</string>
\t\t\t\t<string>glyphs</string>
\t\t</array>
\t\t<array>
\t\t\t\t<string>background</string>
\t\t\t\t<string>glyphs.background</string>
\t\t</array>
</array>
</plist>";

        // observed vs. expected string tests
        assert_eq!(expected_lc_string, test_lc_string);
    }

    #[test]
    fn test_format_indent_fourspace_layercontents_plist() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, true, 4);
        assert!(res_ufo_format.is_ok());

        // layercontents.plist
        let test_lc_string =
            fs::read_to_string(&test_ufo_path.join("layercontents.plist")).unwrap();
        let expected_lc_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<array>
    <array>
        <string>foreground</string>
        <string>glyphs</string>
    </array>
    <array>
        <string>background</string>
        <string>glyphs.background</string>
    </array>
</array>
</plist>";

        // observed vs. expected string tests
        assert_eq!(expected_lc_string, test_lc_string);
    }

    #[test]
    fn test_format_indent_twotabs_lib_plist() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, false, 2);
        assert!(res_ufo_format.is_ok());

        // lib.plist
        let test_lib_string = fs::read_to_string(&test_ufo_path.join("lib.plist")).unwrap();

        let expected_lib_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
\t\t<key>com.defcon.sortDescriptor</key>
\t\t<array>
\t\t\t\t<dict>
\t\t\t\t\t\t<key>allowPseudoUnicode</key>
\t\t\t\t\t\t<true/>
\t\t\t\t\t\t<key>ascending</key>
\t\t\t\t\t\t<true/>
\t\t\t\t\t\t<key>type</key>
\t\t\t\t\t\t<string>alphabetical</string>
\t\t\t\t</dict>
\t\t\t\t<dict>
\t\t\t\t\t\t<key>allowPseudoUnicode</key>
\t\t\t\t\t\t<true/>
\t\t\t\t\t\t<key>ascending</key>
\t\t\t\t\t\t<true/>
\t\t\t\t\t\t<key>type</key>
\t\t\t\t\t\t<string>category</string>
\t\t\t\t</dict>
\t\t\t\t<dict>
\t\t\t\t\t\t<key>allowPseudoUnicode</key>
\t\t\t\t\t\t<true/>
\t\t\t\t\t\t<key>ascending</key>
\t\t\t\t\t\t<true/>
\t\t\t\t\t\t<key>type</key>
\t\t\t\t\t\t<string>unicode</string>
\t\t\t\t</dict>
\t\t\t\t<dict>
\t\t\t\t\t\t<key>allowPseudoUnicode</key>
\t\t\t\t\t\t<true/>
\t\t\t\t\t\t<key>ascending</key>
\t\t\t\t\t\t<true/>
\t\t\t\t\t\t<key>type</key>
\t\t\t\t\t\t<string>script</string>
\t\t\t\t</dict>
\t\t\t\t<dict>
\t\t\t\t\t\t<key>allowPseudoUnicode</key>
\t\t\t\t\t\t<true/>
\t\t\t\t\t\t<key>ascending</key>
\t\t\t\t\t\t<true/>
\t\t\t\t\t\t<key>type</key>
\t\t\t\t\t\t<string>suffix</string>
\t\t\t\t</dict>
\t\t\t\t<dict>
\t\t\t\t\t\t<key>allowPseudoUnicode</key>
\t\t\t\t\t\t<true/>
\t\t\t\t\t\t<key>ascending</key>
\t\t\t\t\t\t<true/>
\t\t\t\t\t\t<key>type</key>
\t\t\t\t\t\t<string>decompositionBase</string>
\t\t\t\t</dict>
\t\t</array>
\t\t<key>com.letterror.lightMeter.prefs</key>
\t\t<dict>
\t\t\t\t<key>chunkSize</key>
\t\t\t\t<integer>5</integer>
\t\t\t\t<key>diameter</key>
\t\t\t\t<integer>200</integer>
\t\t\t\t<key>drawTail</key>
\t\t\t\t<false/>
\t\t\t\t<key>invert</key>
\t\t\t\t<false/>
\t\t\t\t<key>toolDiameter</key>
\t\t\t\t<integer>30</integer>
\t\t\t\t<key>toolStyle</key>
\t\t\t\t<string>fluid</string>
\t\t</dict>
\t\t<key>com.typemytype.robofont.background.layerStrokeColor</key>
\t\t<array>
\t\t\t\t<real>0</real>
\t\t\t\t<real>0.8</real>
\t\t\t\t<real>0.2</real>
\t\t\t\t<real>0.7</real>
\t\t</array>
\t\t<key>com.typemytype.robofont.compileSettings.autohint</key>
\t\t<true/>
\t\t<key>com.typemytype.robofont.compileSettings.checkOutlines</key>
\t\t<false/>
\t\t<key>com.typemytype.robofont.compileSettings.createDummyDSIG</key>
\t\t<true/>
\t\t<key>com.typemytype.robofont.compileSettings.decompose</key>
\t\t<false/>
\t\t<key>com.typemytype.robofont.compileSettings.generateFormat</key>
\t\t<integer>0</integer>
\t\t<key>com.typemytype.robofont.compileSettings.releaseMode</key>
\t\t<false/>
\t\t<key>com.typemytype.robofont.foreground.layerStrokeColor</key>
\t\t<array>
\t\t\t\t<real>0.5</real>
\t\t\t\t<real>0</real>
\t\t\t\t<real>0.5</real>
\t\t\t\t<real>0.7</real>
\t\t</array>
\t\t<key>com.typemytype.robofont.italicSlantOffset</key>
\t\t<integer>0</integer>
\t\t<key>com.typemytype.robofont.segmentType</key>
\t\t<string>curve</string>
\t\t<key>com.typemytype.robofont.shouldAddPointsInSplineConversion</key>
\t\t<integer>1</integer>
\t\t<key>com.typesupply.defcon.sortDescriptor</key>
\t\t<array>
\t\t\t\t<dict>
\t\t\t\t\t\t<key>ascending</key>
\t\t\t\t\t\t<array>
\t\t\t\t\t\t\t\t<string>space</string>
\t\t\t\t\t\t\t\t<string>A</string>
\t\t\t\t\t\t\t\t<string>B</string>
\t\t\t\t\t\t\t\t<string>C</string>
\t\t\t\t\t\t\t\t<string>D</string>
\t\t\t\t\t\t\t\t<string>E</string>
\t\t\t\t\t\t\t\t<string>F</string>
\t\t\t\t\t\t\t\t<string>G</string>
\t\t\t\t\t\t\t\t<string>H</string>
\t\t\t\t\t\t\t\t<string>I</string>
\t\t\t\t\t\t\t\t<string>J</string>
\t\t\t\t\t\t\t\t<string>K</string>
\t\t\t\t\t\t\t\t<string>L</string>
\t\t\t\t\t\t\t\t<string>M</string>
\t\t\t\t\t\t\t\t<string>N</string>
\t\t\t\t\t\t\t\t<string>O</string>
\t\t\t\t\t\t\t\t<string>P</string>
\t\t\t\t\t\t\t\t<string>Q</string>
\t\t\t\t\t\t\t\t<string>R</string>
\t\t\t\t\t\t\t\t<string>S</string>
\t\t\t\t\t\t\t\t<string>T</string>
\t\t\t\t\t\t\t\t<string>U</string>
\t\t\t\t\t\t\t\t<string>V</string>
\t\t\t\t\t\t\t\t<string>W</string>
\t\t\t\t\t\t\t\t<string>X</string>
\t\t\t\t\t\t\t\t<string>Y</string>
\t\t\t\t\t\t\t\t<string>Z</string>
\t\t\t\t\t\t\t\t<string>a</string>
\t\t\t\t\t\t\t\t<string>b</string>
\t\t\t\t\t\t\t\t<string>c</string>
\t\t\t\t\t\t\t\t<string>d</string>
\t\t\t\t\t\t\t\t<string>e</string>
\t\t\t\t\t\t\t\t<string>f</string>
\t\t\t\t\t\t\t\t<string>g</string>
\t\t\t\t\t\t\t\t<string>h</string>
\t\t\t\t\t\t\t\t<string>i</string>
\t\t\t\t\t\t\t\t<string>j</string>
\t\t\t\t\t\t\t\t<string>k</string>
\t\t\t\t\t\t\t\t<string>l</string>
\t\t\t\t\t\t\t\t<string>m</string>
\t\t\t\t\t\t\t\t<string>n</string>
\t\t\t\t\t\t\t\t<string>ntilde</string>
\t\t\t\t\t\t\t\t<string>o</string>
\t\t\t\t\t\t\t\t<string>p</string>
\t\t\t\t\t\t\t\t<string>q</string>
\t\t\t\t\t\t\t\t<string>r</string>
\t\t\t\t\t\t\t\t<string>s</string>
\t\t\t\t\t\t\t\t<string>t</string>
\t\t\t\t\t\t\t\t<string>u</string>
\t\t\t\t\t\t\t\t<string>v</string>
\t\t\t\t\t\t\t\t<string>w</string>
\t\t\t\t\t\t\t\t<string>x</string>
\t\t\t\t\t\t\t\t<string>y</string>
\t\t\t\t\t\t\t\t<string>z</string>
\t\t\t\t\t\t\t\t<string>zcaron</string>
\t\t\t\t\t\t\t\t<string>zero</string>
\t\t\t\t\t\t\t\t<string>one</string>
\t\t\t\t\t\t\t\t<string>two</string>
\t\t\t\t\t\t\t\t<string>three</string>
\t\t\t\t\t\t\t\t<string>four</string>
\t\t\t\t\t\t\t\t<string>five</string>
\t\t\t\t\t\t\t\t<string>six</string>
\t\t\t\t\t\t\t\t<string>seven</string>
\t\t\t\t\t\t\t\t<string>eight</string>
\t\t\t\t\t\t\t\t<string>nine</string>
\t\t\t\t\t\t\t\t<string>underscore</string>
\t\t\t\t\t\t\t\t<string>hyphen</string>
\t\t\t\t\t\t\t\t<string>endash</string>
\t\t\t\t\t\t\t\t<string>emdash</string>
\t\t\t\t\t\t\t\t<string>parenleft</string>
\t\t\t\t\t\t\t\t<string>parenright</string>
\t\t\t\t\t\t\t\t<string>bracketleft</string>
\t\t\t\t\t\t\t\t<string>bracketright</string>
\t\t\t\t\t\t\t\t<string>braceleft</string>
\t\t\t\t\t\t\t\t<string>braceright</string>
\t\t\t\t\t\t\t\t<string>numbersign</string>
\t\t\t\t\t\t\t\t<string>percent</string>
\t\t\t\t\t\t\t\t<string>period</string>
\t\t\t\t\t\t\t\t<string>comma</string>
\t\t\t\t\t\t\t\t<string>colon</string>
\t\t\t\t\t\t\t\t<string>semicolon</string>
\t\t\t\t\t\t\t\t<string>exclam</string>
\t\t\t\t\t\t\t\t<string>question</string>
\t\t\t\t\t\t\t\t<string>slash</string>
\t\t\t\t\t\t\t\t<string>backslash</string>
\t\t\t\t\t\t\t\t<string>bar</string>
\t\t\t\t\t\t\t\t<string>at</string>
\t\t\t\t\t\t\t\t<string>ampersand</string>
\t\t\t\t\t\t\t\t<string>paragraph</string>
\t\t\t\t\t\t\t\t<string>bullet</string>
\t\t\t\t\t\t\t\t<string>dollar</string>
\t\t\t\t\t\t\t\t<string>trademark</string>
\t\t\t\t\t\t\t\t<string>fi</string>
\t\t\t\t\t\t\t\t<string>fl</string>
\t\t\t\t\t\t\t\t<string>.notdef</string>
\t\t\t\t\t\t\t\t<string>a_b_c</string>
\t\t\t\t\t\t\t\t<string>Atilde</string>
\t\t\t\t\t\t\t\t<string>Adieresis</string>
\t\t\t\t\t\t\t\t<string>Acircumflex</string>
\t\t\t\t\t\t\t\t<string>Aring</string>
\t\t\t\t\t\t\t\t<string>Ccedilla</string>
\t\t\t\t\t\t\t\t<string>Agrave</string>
\t\t\t\t\t\t\t\t<string>Aacute</string>
\t\t\t\t\t\t\t\t<string>quotedblright</string>
\t\t\t\t\t\t\t\t<string>quotedblleft</string>
\t\t\t\t\t\t</array>
\t\t\t\t\t\t<key>type</key>
\t\t\t\t\t\t<string>glyphList</string>
\t\t\t\t</dict>
\t\t</array>
\t\t<key>public.glyphOrder</key>
\t\t<array>
\t\t\t\t<string>A</string>
\t\t\t\t<string>Aacute</string>
\t\t\t\t<string>Adieresis</string>
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
\t\t\t\t<string>IJ</string>
\t\t\t\t<string>S.closed</string>
\t\t\t\t<string>I.narrow</string>
\t\t\t\t<string>J.narrow</string>
\t\t\t\t<string>quotesinglbase</string>
\t\t\t\t<string>quotedblbase</string>
\t\t\t\t<string>quotedblleft</string>
\t\t\t\t<string>quotedblright</string>
\t\t\t\t<string>comma</string>
\t\t\t\t<string>period</string>
\t\t\t\t<string>colon</string>
\t\t\t\t<string>semicolon</string>
\t\t\t\t<string>dot</string>
\t\t\t\t<string>dieresis</string>
\t\t\t\t<string>acute</string>
\t\t\t\t<string>space</string>
\t\t\t\t<string>arrowdown</string>
\t\t\t\t<string>arrowleft</string>
\t\t\t\t<string>arrowright</string>
\t\t\t\t<string>arrowup</string>
\t\t</array>
</dict>
</plist>";

        // observed vs. expected string tests
        assert_eq!(expected_lib_string, test_lib_string);
    }

    #[test]
    fn test_format_indent_fourspaces_lib_plist() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, true, 4);
        assert!(res_ufo_format.is_ok());

        // lib.plist
        let test_lib_string = fs::read_to_string(&test_ufo_path.join("lib.plist")).unwrap();

        let expected_lib_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
    <key>com.defcon.sortDescriptor</key>
    <array>
        <dict>
            <key>allowPseudoUnicode</key>
            <true/>
            <key>ascending</key>
            <true/>
            <key>type</key>
            <string>alphabetical</string>
        </dict>
        <dict>
            <key>allowPseudoUnicode</key>
            <true/>
            <key>ascending</key>
            <true/>
            <key>type</key>
            <string>category</string>
        </dict>
        <dict>
            <key>allowPseudoUnicode</key>
            <true/>
            <key>ascending</key>
            <true/>
            <key>type</key>
            <string>unicode</string>
        </dict>
        <dict>
            <key>allowPseudoUnicode</key>
            <true/>
            <key>ascending</key>
            <true/>
            <key>type</key>
            <string>script</string>
        </dict>
        <dict>
            <key>allowPseudoUnicode</key>
            <true/>
            <key>ascending</key>
            <true/>
            <key>type</key>
            <string>suffix</string>
        </dict>
        <dict>
            <key>allowPseudoUnicode</key>
            <true/>
            <key>ascending</key>
            <true/>
            <key>type</key>
            <string>decompositionBase</string>
        </dict>
    </array>
    <key>com.letterror.lightMeter.prefs</key>
    <dict>
        <key>chunkSize</key>
        <integer>5</integer>
        <key>diameter</key>
        <integer>200</integer>
        <key>drawTail</key>
        <false/>
        <key>invert</key>
        <false/>
        <key>toolDiameter</key>
        <integer>30</integer>
        <key>toolStyle</key>
        <string>fluid</string>
    </dict>
    <key>com.typemytype.robofont.background.layerStrokeColor</key>
    <array>
        <real>0</real>
        <real>0.8</real>
        <real>0.2</real>
        <real>0.7</real>
    </array>
    <key>com.typemytype.robofont.compileSettings.autohint</key>
    <true/>
    <key>com.typemytype.robofont.compileSettings.checkOutlines</key>
    <false/>
    <key>com.typemytype.robofont.compileSettings.createDummyDSIG</key>
    <true/>
    <key>com.typemytype.robofont.compileSettings.decompose</key>
    <false/>
    <key>com.typemytype.robofont.compileSettings.generateFormat</key>
    <integer>0</integer>
    <key>com.typemytype.robofont.compileSettings.releaseMode</key>
    <false/>
    <key>com.typemytype.robofont.foreground.layerStrokeColor</key>
    <array>
        <real>0.5</real>
        <real>0</real>
        <real>0.5</real>
        <real>0.7</real>
    </array>
    <key>com.typemytype.robofont.italicSlantOffset</key>
    <integer>0</integer>
    <key>com.typemytype.robofont.segmentType</key>
    <string>curve</string>
    <key>com.typemytype.robofont.shouldAddPointsInSplineConversion</key>
    <integer>1</integer>
    <key>com.typesupply.defcon.sortDescriptor</key>
    <array>
        <dict>
            <key>ascending</key>
            <array>
                <string>space</string>
                <string>A</string>
                <string>B</string>
                <string>C</string>
                <string>D</string>
                <string>E</string>
                <string>F</string>
                <string>G</string>
                <string>H</string>
                <string>I</string>
                <string>J</string>
                <string>K</string>
                <string>L</string>
                <string>M</string>
                <string>N</string>
                <string>O</string>
                <string>P</string>
                <string>Q</string>
                <string>R</string>
                <string>S</string>
                <string>T</string>
                <string>U</string>
                <string>V</string>
                <string>W</string>
                <string>X</string>
                <string>Y</string>
                <string>Z</string>
                <string>a</string>
                <string>b</string>
                <string>c</string>
                <string>d</string>
                <string>e</string>
                <string>f</string>
                <string>g</string>
                <string>h</string>
                <string>i</string>
                <string>j</string>
                <string>k</string>
                <string>l</string>
                <string>m</string>
                <string>n</string>
                <string>ntilde</string>
                <string>o</string>
                <string>p</string>
                <string>q</string>
                <string>r</string>
                <string>s</string>
                <string>t</string>
                <string>u</string>
                <string>v</string>
                <string>w</string>
                <string>x</string>
                <string>y</string>
                <string>z</string>
                <string>zcaron</string>
                <string>zero</string>
                <string>one</string>
                <string>two</string>
                <string>three</string>
                <string>four</string>
                <string>five</string>
                <string>six</string>
                <string>seven</string>
                <string>eight</string>
                <string>nine</string>
                <string>underscore</string>
                <string>hyphen</string>
                <string>endash</string>
                <string>emdash</string>
                <string>parenleft</string>
                <string>parenright</string>
                <string>bracketleft</string>
                <string>bracketright</string>
                <string>braceleft</string>
                <string>braceright</string>
                <string>numbersign</string>
                <string>percent</string>
                <string>period</string>
                <string>comma</string>
                <string>colon</string>
                <string>semicolon</string>
                <string>exclam</string>
                <string>question</string>
                <string>slash</string>
                <string>backslash</string>
                <string>bar</string>
                <string>at</string>
                <string>ampersand</string>
                <string>paragraph</string>
                <string>bullet</string>
                <string>dollar</string>
                <string>trademark</string>
                <string>fi</string>
                <string>fl</string>
                <string>.notdef</string>
                <string>a_b_c</string>
                <string>Atilde</string>
                <string>Adieresis</string>
                <string>Acircumflex</string>
                <string>Aring</string>
                <string>Ccedilla</string>
                <string>Agrave</string>
                <string>Aacute</string>
                <string>quotedblright</string>
                <string>quotedblleft</string>
            </array>
            <key>type</key>
            <string>glyphList</string>
        </dict>
    </array>
    <key>public.glyphOrder</key>
    <array>
        <string>A</string>
        <string>Aacute</string>
        <string>Adieresis</string>
        <string>B</string>
        <string>C</string>
        <string>D</string>
        <string>E</string>
        <string>F</string>
        <string>G</string>
        <string>H</string>
        <string>I</string>
        <string>J</string>
        <string>K</string>
        <string>L</string>
        <string>M</string>
        <string>N</string>
        <string>O</string>
        <string>P</string>
        <string>Q</string>
        <string>R</string>
        <string>S</string>
        <string>T</string>
        <string>U</string>
        <string>V</string>
        <string>W</string>
        <string>X</string>
        <string>Y</string>
        <string>Z</string>
        <string>IJ</string>
        <string>S.closed</string>
        <string>I.narrow</string>
        <string>J.narrow</string>
        <string>quotesinglbase</string>
        <string>quotedblbase</string>
        <string>quotedblleft</string>
        <string>quotedblright</string>
        <string>comma</string>
        <string>period</string>
        <string>colon</string>
        <string>semicolon</string>
        <string>dot</string>
        <string>dieresis</string>
        <string>acute</string>
        <string>space</string>
        <string>arrowdown</string>
        <string>arrowleft</string>
        <string>arrowright</string>
        <string>arrowup</string>
    </array>
</dict>
</plist>";

        // observed vs. expected string tests
        assert_eq!(expected_lib_string, test_lib_string);
    }

    #[test]
    fn test_format_indent_twotabs_metainfo_plist() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, false, 2);
        assert!(res_ufo_format.is_ok());

        // metainfo.plist
        let test_mi_string = fs::read_to_string(&test_ufo_path.join("metainfo.plist")).unwrap();

        let expected_mi_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
\t\t<key>creator</key>
\t\t<string>org.linebender.norad</string>
\t\t<key>formatVersion</key>
\t\t<integer>3</integer>
</dict>
</plist>";

        // observed vs. expected string tests
        assert_eq!(expected_mi_string, test_mi_string);
    }

    #[test]
    fn test_format_indent_fourspaces_metainfo_plist() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, true, 4);
        assert!(res_ufo_format.is_ok());

        // metainfo.plist
        let test_mi_string = fs::read_to_string(&test_ufo_path.join("metainfo.plist")).unwrap();

        let expected_mi_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
    <key>creator</key>
    <string>org.linebender.norad</string>
    <key>formatVersion</key>
    <integer>3</integer>
</dict>
</plist>";

        // observed vs. expected string tests
        assert_eq!(expected_mi_string, test_mi_string);
    }

    #[test]
    fn test_format_indent_twotabs_contents_plist() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, false, 2);
        assert!(res_ufo_format.is_ok());

        // glyphs/contents.plist
        let test_contents_string =
            fs::read_to_string(&test_ufo_path.join("glyphs").join("contents.plist")).unwrap();

        let expected_contents_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
\t\t<key>A</key>
\t\t<string>A_.glif</string>
\t\t<key>Aacute</key>
\t\t<string>A_acute.glif</string>
\t\t<key>Adieresis</key>
\t\t<string>A_dieresis.glif</string>
\t\t<key>B</key>
\t\t<string>B_.glif</string>
\t\t<key>C</key>
\t\t<string>C_.glif</string>
\t\t<key>D</key>
\t\t<string>D_.glif</string>
\t\t<key>E</key>
\t\t<string>E_.glif</string>
\t\t<key>F</key>
\t\t<string>F_.glif</string>
\t\t<key>G</key>
\t\t<string>G_.glif</string>
\t\t<key>H</key>
\t\t<string>H_.glif</string>
\t\t<key>I</key>
\t\t<string>I_.glif</string>
\t\t<key>I.narrow</key>
\t\t<string>I_.narrow.glif</string>
\t\t<key>IJ</key>
\t\t<string>I_J_.glif</string>
\t\t<key>J</key>
\t\t<string>J_.glif</string>
\t\t<key>J.narrow</key>
\t\t<string>J_.narrow.glif</string>
\t\t<key>K</key>
\t\t<string>K_.glif</string>
\t\t<key>L</key>
\t\t<string>L_.glif</string>
\t\t<key>M</key>
\t\t<string>M_.glif</string>
\t\t<key>N</key>
\t\t<string>N_.glif</string>
\t\t<key>O</key>
\t\t<string>O_.glif</string>
\t\t<key>P</key>
\t\t<string>P_.glif</string>
\t\t<key>Q</key>
\t\t<string>Q_.glif</string>
\t\t<key>R</key>
\t\t<string>R_.glif</string>
\t\t<key>S</key>
\t\t<string>S_.glif</string>
\t\t<key>S.closed</key>
\t\t<string>S_.closed.glif</string>
\t\t<key>T</key>
\t\t<string>T_.glif</string>
\t\t<key>U</key>
\t\t<string>U_.glif</string>
\t\t<key>V</key>
\t\t<string>V_.glif</string>
\t\t<key>W</key>
\t\t<string>W_.glif</string>
\t\t<key>X</key>
\t\t<string>X_.glif</string>
\t\t<key>Y</key>
\t\t<string>Y_.glif</string>
\t\t<key>Z</key>
\t\t<string>Z_.glif</string>
\t\t<key>acute</key>
\t\t<string>acute.glif</string>
\t\t<key>arrowdown</key>
\t\t<string>arrowdown.glif</string>
\t\t<key>arrowleft</key>
\t\t<string>arrowleft.glif</string>
\t\t<key>arrowright</key>
\t\t<string>arrowright.glif</string>
\t\t<key>arrowup</key>
\t\t<string>arrowup.glif</string>
\t\t<key>colon</key>
\t\t<string>colon.glif</string>
\t\t<key>comma</key>
\t\t<string>comma.glif</string>
\t\t<key>dieresis</key>
\t\t<string>dieresis.glif</string>
\t\t<key>dot</key>
\t\t<string>dot.glif</string>
\t\t<key>period</key>
\t\t<string>period.glif</string>
\t\t<key>quotedblbase</key>
\t\t<string>quotedblbase.glif</string>
\t\t<key>quotedblleft</key>
\t\t<string>quotedblleft.glif</string>
\t\t<key>quotedblright</key>
\t\t<string>quotedblright.glif</string>
\t\t<key>quotesinglbase</key>
\t\t<string>quotesinglbase.glif</string>
\t\t<key>semicolon</key>
\t\t<string>semicolon.glif</string>
\t\t<key>space</key>
\t\t<string>space.glif</string>
</dict>
</plist>";

        // observed vs. expected string tests
        assert_eq!(expected_contents_string, test_contents_string);
    }

    #[test]
    fn test_format_indent_fourspaces_contents_plist() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, false, true, 4);
        assert!(res_ufo_format.is_ok());

        // glyphs/contents.plist
        let test_contents_string =
            fs::read_to_string(&test_ufo_path.join("glyphs").join("contents.plist")).unwrap();

        let expected_contents_string = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
    <key>A</key>
    <string>A_.glif</string>
    <key>Aacute</key>
    <string>A_acute.glif</string>
    <key>Adieresis</key>
    <string>A_dieresis.glif</string>
    <key>B</key>
    <string>B_.glif</string>
    <key>C</key>
    <string>C_.glif</string>
    <key>D</key>
    <string>D_.glif</string>
    <key>E</key>
    <string>E_.glif</string>
    <key>F</key>
    <string>F_.glif</string>
    <key>G</key>
    <string>G_.glif</string>
    <key>H</key>
    <string>H_.glif</string>
    <key>I</key>
    <string>I_.glif</string>
    <key>I.narrow</key>
    <string>I_.narrow.glif</string>
    <key>IJ</key>
    <string>I_J_.glif</string>
    <key>J</key>
    <string>J_.glif</string>
    <key>J.narrow</key>
    <string>J_.narrow.glif</string>
    <key>K</key>
    <string>K_.glif</string>
    <key>L</key>
    <string>L_.glif</string>
    <key>M</key>
    <string>M_.glif</string>
    <key>N</key>
    <string>N_.glif</string>
    <key>O</key>
    <string>O_.glif</string>
    <key>P</key>
    <string>P_.glif</string>
    <key>Q</key>
    <string>Q_.glif</string>
    <key>R</key>
    <string>R_.glif</string>
    <key>S</key>
    <string>S_.glif</string>
    <key>S.closed</key>
    <string>S_.closed.glif</string>
    <key>T</key>
    <string>T_.glif</string>
    <key>U</key>
    <string>U_.glif</string>
    <key>V</key>
    <string>V_.glif</string>
    <key>W</key>
    <string>W_.glif</string>
    <key>X</key>
    <string>X_.glif</string>
    <key>Y</key>
    <string>Y_.glif</string>
    <key>Z</key>
    <string>Z_.glif</string>
    <key>acute</key>
    <string>acute.glif</string>
    <key>arrowdown</key>
    <string>arrowdown.glif</string>
    <key>arrowleft</key>
    <string>arrowleft.glif</string>
    <key>arrowright</key>
    <string>arrowright.glif</string>
    <key>arrowup</key>
    <string>arrowup.glif</string>
    <key>colon</key>
    <string>colon.glif</string>
    <key>comma</key>
    <string>comma.glif</string>
    <key>dieresis</key>
    <string>dieresis.glif</string>
    <key>dot</key>
    <string>dot.glif</string>
    <key>period</key>
    <string>period.glif</string>
    <key>quotedblbase</key>
    <string>quotedblbase.glif</string>
    <key>quotedblleft</key>
    <string>quotedblleft.glif</string>
    <key>quotedblright</key>
    <string>quotedblright.glif</string>
    <key>quotesinglbase</key>
    <string>quotesinglbase.glif</string>
    <key>semicolon</key>
    <string>semicolon.glif</string>
    <key>space</key>
    <string>space.glif</string>
</dict>
</plist>";

        // observed vs. expected string tests
        assert_eq!(expected_contents_string, test_contents_string);
    }

    #[test]
    fn test_format_indent_spaces_singlequotes_metainfo_plist() {
        let tmp_dir = tempdir::TempDir::new("test").unwrap();
        let src_ufo_path = Path::new("testdata/ufo/MutatorSansBoldCondensed.ufo");
        let copy_opt = CopyOptions::new();
        let res_ufo_copy = copy(&src_ufo_path, &tmp_dir.path(), &copy_opt);
        assert!(res_ufo_copy.is_ok());
        let test_ufo_path = tmp_dir.path().join("MutatorSansBoldCondensed.ufo");

        let res_ufo_format = format_ufo(&test_ufo_path, &None, &None, true, true, 4);
        assert!(res_ufo_format.is_ok());

        // metainfo.plist
        let test_mi_string = fs::read_to_string(&test_ufo_path.join("metainfo.plist")).unwrap();

        let expected_mi_string = "<?xml version='1.0' encoding='UTF-8'?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
    <key>creator</key>
    <string>org.linebender.norad</string>
    <key>formatVersion</key>
    <integer>3</integer>
</dict>
</plist>";

        // observed vs. expected string tests
        assert_eq!(expected_mi_string, test_mi_string);
    }
}
