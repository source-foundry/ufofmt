use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub(crate) fn get_ufo_outpath(
    user_ufo_path: &Path,
    user_unique_filename: &Option<String>,
    user_unique_extension: &Option<String>,
) -> PathBuf {
    let original_basepath = match user_ufo_path.parent() {
        Some(opar) => opar,
        None => Path::new("."),
    };

    let new_basepath = PathBuf::from(original_basepath);

    let original_dir_rootpath = match user_ufo_path.file_stem() {
        Some(oroot) => oroot,
        None => panic!("missing directory root!"),
    };

    let mut new_outpath = match user_unique_filename {
        Some(unique_name) => {
            new_basepath.join(format!("{}{}", original_dir_rootpath.to_string_lossy(), unique_name))
        }
        None => new_basepath.join(original_dir_rootpath),
    };

    let original_extension = match user_ufo_path.extension() {
        Some(oext) => oext,
        None => OsStr::new(""),
    };

    match user_unique_extension {
        Some(unique_ext) => {
            // create the new extension string
            if unique_ext.starts_with('.') {
                new_outpath.set_extension(unique_ext.strip_prefix('.').unwrap())
            } else {
                new_outpath.set_extension(unique_ext)
            }
        }
        None => new_outpath.set_extension(original_extension),
    };

    new_outpath
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ufo_outpath_default() {
        let op = get_ufo_outpath(&Path::new("one/two/three.ufo"), &None, &None);
        assert_eq!(op, PathBuf::from("one/two/three.ufo"));
    }

    #[test]
    fn test_get_ufo_path_unique_filename() {
        let op = get_ufo_outpath(&Path::new("one/two/three.ufo"), &Some("-new".to_string()), &None);
        assert_eq!(op, PathBuf::from("one/two/three-new.ufo"));
    }

    #[test]
    fn test_get_ufo_path_unique_extension_two_extensions_withperiod() {
        let op =
            get_ufo_outpath(&Path::new("one/two/three.ufo"), &None, &Some(".fmt.ufo".to_string()));
        assert_eq!(op, PathBuf::from("one/two/three.fmt.ufo"));
    }

    #[test]
    fn test_get_ufo_path_unique_extension_two_extensions_without_period() {
        let op =
            get_ufo_outpath(&Path::new("one/two/three.ufo"), &None, &Some("fmt.ufo".to_string()));
        assert_eq!(op, PathBuf::from("one/two/three.fmt.ufo"));
    }

    #[test]
    fn test_get_ufo_path_unique_extension_single_extension_with_period() {
        let op = get_ufo_outpath(&Path::new("one/two/three.ufo"), &None, &Some(".fmt".to_string()));
        assert_eq!(op, PathBuf::from("one/two/three.fmt"));
    }

    #[test]
    fn test_get_ufo_path_unique_extension_single_extension_without_period() {
        let op = get_ufo_outpath(&Path::new("one/two/three.ufo"), &None, &Some("fmt".to_string()));
        assert_eq!(op, PathBuf::from("one/two/three.fmt"));
    }

    #[test]
    fn test_get_ufo_path_unique_extension_empty_extension() {
        let op = get_ufo_outpath(&Path::new("one/two/three.ufo"), &None, &Some("".to_string()));
        assert_eq!(op, PathBuf::from("one/two/three"));
    }

    #[test]
    fn test_get_ufo_path_unique_extension_unique_dirname() {
        let op = get_ufo_outpath(
            &Path::new("one/two/three.ufo"),
            &Some("-new".to_string()),
            &Some("fmt".to_string()),
        );
        assert_eq!(op, PathBuf::from("one/two/three-new.fmt"));
    }
}
