use std::path::Path;

pub(crate) fn is_invalid_ufo_dir_path(ufopath: &Path) -> bool {
    !ufopath.exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_invalid_ufo_dir_path_invalid_path() {
        let invalid_path = Path::new("totally/bogus/path/test.ufo");
        assert!(is_invalid_ufo_dir_path(invalid_path));
    }

    #[test]
    fn test_is_invalid_ufo_dir_path_valid_path() {
        let valid_path = Path::new("testdata/ufo/MutatorSansBoldWide.ufo");
        assert_eq!(is_invalid_ufo_dir_path(valid_path), false);
    }
}
