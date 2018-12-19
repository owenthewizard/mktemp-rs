use std::{env, fs};
use std::ffi::CString;
use std::io::{Error, ErrorKind, Result};

#[cfg(unix)]
use libc::mkdtemp;

/// A temporary directory
pub struct TempDir {
    path: String,
}

impl TempDir {
    /// Creates a new temporary directory with the given prefix
    ///
    /// # Errors
    ///
    /// Any of the following will produce errors:
    /// * An OS temporary directory that contain non-UTF-8 characters
    /// * Failure to parse a CString from the given data
    /// * `mkdtemp` returning NULL
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs;
    /// use mktemp::TempDir;
    ///
    /// let td = TempDir::new("my-groovy-tempdir-").unwrap();
    /// assert!(fs::metadata(td.path()).is_ok());
    /// ```
    pub fn new(prefix: &str) -> Result<TempDir> {
        debug!("init new TempDir");
        // validate temp dir
        let tmp_dir = match env::temp_dir().to_str() {
            Some(s) => s.to_string(),
            None => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "Temporary directory path must be valid UTF-8",
                ))
            }
        };
        debug!("found temp dir: {}", &tmp_dir);

        // CString --> &c_char
        let ptr = match CString::new(format!("{}/{}XXXXXX", tmp_dir, prefix)) {
            Ok(p) => p.into_raw(),
            Err(e) => return Err(Error::new(ErrorKind::Other, e)),
        };
        debug!("CString to raw done");

        // mkdir and null check
        let ptr = unsafe { mkdtemp(ptr) };
        if ptr.is_null() {
            debug!("mkdtemp returned NULL pointer");
            return Err(Error::last_os_error());
        }
        debug!("directory created");

        // &c_char --> CString --> String
        let path = match unsafe { CString::from_raw(ptr) }.into_string() {
            Ok(s) => s,
            Err(e) => return Err(Error::new(ErrorKind::Other, e)),
        };
        debug!("raw to CString to String done");
        debug!("got file path: {}", &path);

        // yay!
        Ok(TempDir { path: path })
    }

    /// Return the path to the temporary directory
    pub fn path(&self) -> &str {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        debug!("Dropping TempDir: {}", &self.path);
        let _ = fs::remove_dir_all(&self.path);
    }
}
