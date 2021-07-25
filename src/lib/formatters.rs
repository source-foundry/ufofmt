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
