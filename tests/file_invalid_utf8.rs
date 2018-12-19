use std::env;
use std::ffi::OsString;

#[cfg(unix)]
use std::os::unix::ffi::OsStringExt;

#[cfg(windows)]
use std::os::windows::ffi::OsStringExt;

use mktemp::TempFile;

#[cfg(unix)]
#[test]
#[should_panic(expected = "Temporary directory path must be valid UTF-8")]
fn tf_invalid_utf8() {
    let key = "TMPDIR";
    let invalid_utf8: Vec<u8> = vec![104, 101, 255, 108, 111];
    env::set_var(key, OsString::from_vec(invalid_utf8));
    let tf = TempFile::new("", "");
    assert!(tf.is_ok(), tf.unwrap_or_else(|e| panic!("{}", e)));
}

/*
#[cfg(windows)]
#[test]
#[should_panic(expected = "Temporary directory path must be valid UTF-8")]
fn tf_invalid_utf8() {
    let key = "TMP";
    let invalid_utf8: &[u16; 5] = &[104, 101, 255, 108, 111];
    env::set_var(key, OsString::from_wide(invalid_utf8));
    let tf = TempFile::new("", "");
    assert!(tf.is_ok(), tf.unwrap_or_else(|e| panic!("{}", e)));
}
*/
