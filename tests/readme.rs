use std::fs;
use std::io::{Seek, SeekFrom, Read, Write};

use mktemp::TempFile;

#[test]
fn readme() {
    let path;
    {
        let mut tf = TempFile::new("my-temp-file-", ".txt").expect("Failed to create tempfile");
        let mut buf = [0u8; 12];
            tf.write(b"Hello world!").expect("Failed to write to tempfile");
        tf.seek(SeekFrom::Start(0)).expect("Failed to seek in tempfile");
        tf.read(&mut buf).expect("Failed to read tempfile");
        assert_eq!(&buf, b"Hello world!");
        path = tf.path().to_string();
    }
    assert!(fs::metadata(&path).is_err());
}
