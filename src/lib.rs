/*a Copyright

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

@file    lib.rs
@brief   UTF-8 stream reader
 */

//a Documentation
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

/*!

# UTF8 reader

The `utf8-read` module provides a streaming `char` [Reader] that
converts any stream with the [std::io::Read] into a stream of `char`
values, performing UTF8 decoding incrementally.

If the [std::io::Read] stream comes from a file then this is just a
streaming version of (e.g.) std::fs::read_to_string, but if the it
comes from, e.g., a [std::net::TcpStream] then it has more value:
iterating through the characters of the stream will terminate when the
TCP stream has stalled mid-UTF8, and can restart when the TCP stream
gets more data.

The [Reader] provided also allows for reading large UTF8 files
piecewise; it only reads up to 2kB of data at a time from its stream.


# Example

An example use would be:

```
use utf8_read::Reader;
let str = "This is a \u{1f600} string\nWith a newline\n";
let mut buf_bytes = str.as_bytes();
let mut reader    = Reader::new(&mut buf_bytes);
for x in reader.into_iter() {
    // use char x
}

```

From a file, one could do:

```
use utf8_read::Reader;
let in_file = std::fs::File::open("Cargo.toml").unwrap();
let mut reader = Reader::new(&in_file);
for x in reader.into_iter() {
    // use char x
}

```

!*/

//a Imports
mod types;
mod stream_position;
mod reader;

//a Exports
pub use types::{Char, Error, Result};
pub use stream_position::StreamPosition;
pub use reader::Reader;
