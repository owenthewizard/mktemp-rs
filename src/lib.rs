#![warn(missing_docs)]

//! # mktemp
//!
//! `mktemp` is a thin wrapper around `libc`'s `mkstemps` and `mkdtemp`.

macro_rules! debug {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            eprintln!("{f}:{l}:{c} {fmt}", f=file!(), l=line!(), c=column!(), fmt=format!($($arg)*));
        }
    }
}

mod temp_file;
pub use self::temp_file::TempFile;

mod temp_dir;
pub use self::temp_dir::TempDir;
