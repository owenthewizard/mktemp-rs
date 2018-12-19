use std::fs;

use mktemp::TempDir;

macro_rules! assert_ok {
    ($e:expr) => {
        assert!($e.is_ok(), $e.unwrap_or_else(|e| panic!("{}", e)));
    };
}

#[test]
fn td_create_none() {
    let td = TempDir::new("");
    assert_ok!(td);
}

#[test]
fn td_create_prefix() {
    let td = TempDir::new("prefix-");
    assert_ok!(td);
}

#[test]
#[should_panic(expected = "No such file or directory")]
fn td_drop() {
    let path;
    {
        let td = TempDir::new("").unwrap();
        path = td.path().to_string();
    }
    let md = fs::metadata(&path);
    assert_ok!(md);
}
