use std::path::Path;

pub(crate) fn is_invalid_ufo_dir_path(ufopath: &Path) -> bool {
    !ufopath.exists()
}
