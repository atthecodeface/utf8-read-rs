//a Imports
use crate::StreamPosition;

//a Character result and error
//tp Char
/// [Char] represents a unicode character, insufficient data, or an EOF marker
///
/// It is returned, for example, by the [next_char](crate::Reader::next_char) method.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Char {
    /// [Eof](Char::Eof) indicates end of stream/file reached; once a reader
    /// returns Eof, it should continue to do so
    Eof,
    /// [NoData](Char::NoData) indicates that the stream/file did not supply data,
    /// but this is configured to not be EOF.
    ///
    /// This can only be returned
    /// by the reader if [crate::Reader::set_eof_on_no_data] has been used
    NoData,
    /// [Char](Char::Char) indicates a char of a valid Unicode codepoint decoded
    /// from the stream with UTF8
    Char(char)
}

//ip std::fmt::Display for Char
impl std::fmt::Display for Char {
    //mp fmt - format a character for display
    /// Display the character as either the character itself, or '<EOF>'
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::fmt::Write; // for f.write_char
        match self {
            Char::Eof      => write!(f, "<EOF>"),
            Char::NoData   => write!(f, "<NoData>"),
            Char::Char(ch) => f.write_char(*ch),
        }
    }
}

//a Result
//tp Result
/// The [Result] type is a result with an error type of [crate::Error]
pub type Result<T> = std::result::Result<T, Error>;

//a Error
//tp Error
/// [Error] represents an error from the UTF-8 character reader,
/// either an IO error from the reader or a malformed UTF-8 encoded
/// set of bytes.
#[derive(Debug)]
pub enum Error {
    /// An [IoError](std::io::Error) is passed through from the underlying read object.
    IoError(std::io::Error),
    /// A [MalformedUtf8](Error::MalformedUtf8) error occurs when a byte stream contains
    /// invalid UTF-8; the position within the stream of the Unicode decoding error is
    /// recorded, and the number of bytes that form the invalid UTF-8
    /// encoding (which will be from 1 to 3).
    MalformedUtf8(StreamPosition, usize),
}

//ip Error
impl Error {
    //mp malformedutf8
    /// Create an error for a malformed UTF8 decoding within a stream
    pub fn malformed_utf8<T>(stream_pos:StreamPosition, num_bytes:usize) -> Result<T> {
        Err(Self::MalformedUtf8(stream_pos, num_bytes))
    }
}

//ip From<std::io::Error> for Error
/// Provides an implicit conversion from a std::io::Error to a Error
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e)
    }
}

//ip std::fmt::Display for Error
impl std::fmt::Display for Error {
    //mp fmt - format a `Error` for display
    /// Display the `Error` in a human-readable form
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::MalformedUtf8(pos, n) => write!(f, "malformed UTF-8 of {} bytes at {}", n, pos),
            Error::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

//ip std::error::Error for Error
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IoError(e) => Some(e),
            _ => None,
        }
    }
}
