# utf8-read

A UTF8 character stream reader using a provided std::io::Read byte stream reader, that provides a stream of UTF8 characters; the underlying stream reader can be a stop-start stream such as a TCP stream, where a read() returning 0 does not indicate end-of-stream.

The UTF8 reader provides a step above std::fs::read_to_string function; for reading short UTF8 files that function is a better approach.

This crate is in beta; it is used in a small number of applications,
and the functionality is mature; the API is stable, but may be enhanced.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
utf8-read = "0.5.0"
```

## Releases

Release notes are available in [RELEASES.md](RELEASES.md).

## License

Licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
