use std::env;
use std::ffi::CString;
use std::fmt::Arguments;
use std::fs::{self, File};
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};

#[cfg(unix)]
use std::os::unix::io::FromRawFd;
#[cfg(unix)]
use libc::{c_int, mkstemps};

/// A temporary file
pub struct TempFile {
    file: Option<File>,
    path: String,
}

impl TempFile {
    /// Creates a new temporary file with the given prefix and suffix.
    ///
    /// # Errors
    ///
    /// Any of the following will produce errors:
    /// * A suffix with > `std::i32::MAX` chars
    /// * Failure to parse a CString from the given data
    /// * `mkstemps` returning an invalid file descriptor
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Write;
    /// use mktemp::TempFile;
    ///
    /// let mut tf = TempFile::new("my-groovy-prefix-", ".txt").unwrap();
    /// tf.write(b"Hello world!");
    /// ```
    pub fn new(prefix: &str, suffix: &str) -> Result<TempFile> {
        debug!("init new TempFile");
        // validate suffix len
        let len = suffix.chars().count();
        if len > std::i32::MAX as usize {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Suffix length must be less than std::i32::MAX",
            ));
        }
        debug!("validated suffix len");

        // get temporary directory
        let tmp_dir = env::temp_dir();
        debug!("found temp dir: {:?}", tmp_dir);

        // CString --> &c_char
        let ptr = match CString::new(format!("{}/{}XXXXXX{}", tmp_dir.display(), prefix, suffix)) {
            Ok(p) => p.into_raw(),
            Err(e) => return Err(Error::new(ErrorKind::Other, e)),
        };
        debug!("CString to raw done");

        // get fd
        let fd = unsafe { mkstemps(ptr, len as c_int) };
        debug!("got fd: {}", fd);

        // &c_char --> CString --> String
        let path = match unsafe { CString::from_raw(ptr) }.into_string() {
            Ok(s) => s,
            Err(e) => return Err(Error::new(ErrorKind::Other, e)),
        };
        debug!("raw to CString to String done");
        debug!("got file path: {}", &path);

        // handle mkstemps error (if any)
        if fd < 0 {
            debug!("fd was error");
            return Err(Error::last_os_error());
        }

        // fd --> File
        let file = unsafe { File::from_raw_fd(fd) };
        debug!("got file from fd");

        // yay!
        Ok(TempFile {
            file: Some(file),
            path: path,
        })
    }

    /// Return the path to the underlying file
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Return the underlying File
    pub fn inner(&mut self) -> &mut File {
        self.file.as_mut().unwrap()
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        debug!("Dropping TempFile: {}", &self.path);
        self.file = None;
        let _ = fs::remove_file(&self.path);
    }
}

impl Seek for TempFile {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.inner().seek(pos)
    }
}

impl Read for TempFile {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.inner().read(buf)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        self.inner().read_to_end(buf)
    }

    fn read_to_string(&mut self, buf: &mut String) -> Result<usize> {
        self.inner().read_to_string(buf)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        self.inner().read_exact(buf)
    }
}

impl Write for TempFile {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.inner().write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.inner().flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.inner().write_all(buf)
    }

    fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result<()> {
        self.inner().write_fmt(fmt)
    }
}
