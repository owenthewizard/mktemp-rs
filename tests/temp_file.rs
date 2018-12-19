use std::{fs, iter};
use std::io::{Read, Seek, SeekFrom, Write};

use mktemp::TempFile;

macro_rules! assert_ok {
    ($e:expr) => {
        assert!($e.is_ok(), $e.unwrap_or_else(|e| panic!("{}", e)));
    };
}

#[test]
fn tf_create_neither() {
    let tf = TempFile::new("", "");
    assert_ok!(tf);
}

#[test]
fn tf_create_prefix() {
    let tf = TempFile::new("prefix-", "");
    assert_ok!(tf);
}

#[test]
fn tf_create_suffix() {
    let tf = TempFile::new("", "-suffix");
    assert_ok!(tf);
}

#[test]
fn tf_create_both() {
    let tf = TempFile::new("prefix-", "-suffix");
    assert_ok!(tf);
}

#[test]
#[should_panic(expected = "No such file or directory")]
fn tf_drop() {
    let path;
    {
        let tf = TempFile::new("", "").unwrap();
        path = tf.path().to_string();
    }
    let md = fs::metadata(&path);
    assert_ok!(md);
}

#[test]
fn tf_read_write() {
    let mut tf = TempFile::new("", "").unwrap();
    let data = b"Hello world!";
    let res = tf.write(data);
    assert_ok!(res);

    let res = tf.seek(SeekFrom::Start(0));
    assert_ok!(res);

    let mut buf = [0u8; 12];
    let res = tf.read(&mut buf);
    assert_ok!(res);
}

#[test]
#[should_panic(expected = "Suffix length must be less than std::i32::MAX")]
#[ignore] // takes a few minutes and a lot of RAM on debug, recommend using --release
fn tf_suffix_overflow() {
    let super_long_string = iter::repeat("ab")
        .take((std::i32::MAX as usize + 1) / 2)
        .collect::<String>();
    let tf = TempFile::new("", &super_long_string);
    assert_ok!(tf);
}
