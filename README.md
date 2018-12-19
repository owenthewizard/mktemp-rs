# mktemp-rs

A thin wrapper around `libc`'s `mkstemps` and `mkdtemp`.

## Quick Start

`Cargo.toml`:
```diff
name = "my-awesome-project"
version = "0.1.0"
authors = ["me"]

[dependencies]
+mktemp-rs = "0.1.0"
```

`main.rs`:
```rust
use std::fs;
use std::io::{Seek, SeekFrom, Read, Write};
use mktemp::TempFile;

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
```

`mktemp-rs` currently only support Unix platforms. As always, pull requests are welcome.

### Tests

[`dir_invalid_utf8`](tests/dir_invalid_utf8.rs) tests that TempDir::new() panics when the OS temporary directory path is not valid UTF-8.

[`file_invalid_utf8`](tests/file_invalid_utf8.rs) tests that TempFile::new() panics when the OS temporary directory path is not valid UTF-8.

[`readme`](tests/readme.rs) tests the example in this readme.

[`temp_dir`](tests/temp_dir.rs) tests various TempDir functions.

[`temp_file`](tests/temp_file.rs) tests various TempFile functions.

### Coding Style

Obey `rustfmt` and Rust 2018 conventions.

## Contributing

Pull requests are always welcome. See [TODO](TODO.md).

## Versioning

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Changes are documented in the [Changelog](CHANGELOG.md).

See the [tags on this repository](https://github.com/owenthewizard/mktemp-rs/tags) for available releases.

## Authors

See [the list of contributors](https://github.com/owenthewizard/mktemp-rs/contributors).

## License

`mktemp-rs` is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE.md) and [LICENSE-MIT](LICENSE-MIT.md) for details.

## Acknowledgments

* [mkstemp](https://gitlab.com/worr/mkstemp) by William Orr for inspiration and code base.
