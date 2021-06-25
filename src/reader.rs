//a Imports
use crate::{Char, Error, Result, StreamPosition};

//a Constants
/// [BUFFER_SIZE] is the maximum number of bytes held in the UTF-8
/// character reader from the incoming stream.  The larger the value,
/// the larger the data read requests from the stream. This value must be larger than `BUFFER_SLACK`.
/// For testing purposes this value should be small (such as 8), to catch corner cases in the code where UTF-8 encodings
/// run over the end of a buffer; for performance, this value should be larger (e.g. 2048).
const BUFFER_SIZE  : usize = 2048;

/// [BUFFER_SLACK] must be at least 4 - the maximum number of bytes in
/// a UTF-8 encoding; when fewer than BUFFER_SLACK bytes are in the
/// buffer a read from the buffer stream is performed - attempting to
/// fill the `BUFFER_SIZE` buffer with current data and new read data.
/// There is no reason why `BUFFER_SLACK` should be larger than 4.
const BUFFER_SLACK : usize = 4;

//a Reader
//tp Reader
/// The [Reader] provides a stream of characters by UTF-8 decoding a byte
/// stream provided by any type that implements the [std::io::Read] stream trait.
///
/// It utilizes an internal buffer of bytes that are filled as
/// required from the read stream; it maintains a position with the
/// stream (line and character) for the next character, and provides
/// the ability to get a stream of characters from the stream with any
/// UTF-8 encoding errors reported by line and character.
///
/// The stream can be reclaimed by completing the use of the
/// [Reader], in which case any unused bytes that have been read from
/// the stream are also returned.
///
/// If simple short files are to be read, using
/// [std::fs::read_to_string] may a better approach than using the
/// `Reader`
///
/// # Example
///
/// ```
///     use utf8_read::Reader;
///     let str = "This is a \u{1f600} string\nWith a newline\n";
///     let mut buf_bytes = str.as_bytes();
///     let mut reader    = Reader::new(&mut buf_bytes);
///     for x in reader.into_iter() {
///         // use char x
///     }
/// ```
///
/// This example could just as easily use 'for x in str'
///
/// The [Reader], though, can be used over any object supporting the
/// [Read](std::io::Read) trait such as a a
/// [TcpStrema](std::net::TcpStream).
///
pub struct Reader<R:std::io::Read> {
    /// The reader from which data is to be fetched
    buf_reader  : R,
    /// `eof_on_no_data` defaults to true; it can be set to false to indicate that
    /// if the stream has no data then the reader should return Char::NoData
    /// when its buffer does not contain a complete UTF-8 character
    eof_on_no_data : bool,
    /// `eof` is set when the stream is complete - any character
    /// requested once `eof` is asserted will be `Char::Eof`.
    eof        : bool,
    /// Internal buffer
    current    : [u8; BUFFER_SIZE],
    /// Offset of the first byte within the internal buffer that is valid
    start      : usize,
    /// `Offset of the last byte + 1 within the internal buffer that is valid
    end        : usize,
    /// `valid_end` is the last byte + 1 within the internal buffer
    /// used by a valid UTF-8 byte stream that begins with `start` As
    /// such `start` <= `valid_end` <= `end` If `start` < `valid_end`
    /// then the bytes in the buffer between the two are a valid UTF-8
    /// byte stream; this should perhaps be kept in a string inside
    /// the structure for performance
    valid_end  : usize,
    /// position in the file
    stream_pos : StreamPosition,
}

//ip Reader
impl <R:std::io::Read> Reader<R> {

    //fp new
    /// Returns a new UTF-8 character [Reader], with a stream position
    /// set to the normal start of the file - byte 0, line 1,
    /// character 1
    ///
    /// The [Reader] will default to handling zero bytes returned by
    /// the stream as an EOF; to modify this default behavior use the
    /// [set_eof_on_no_data](Reader::set_eof_on_no_data) builder to
    /// modify the construction.
    pub fn new(buf_reader: R) -> Self {
        Self {
            buf_reader,
            eof_on_no_data : true,
            eof            : false,
            current        : [0; BUFFER_SIZE],
            start          : 0,
            end            : 0,
            valid_end      : 0,
            stream_pos     : StreamPosition::new(),
        }
    }

    //cp set_eof_on_no_data
    /// Build pattern function to set the `eof_on_no_data` on the [Reader] to true or false
    ///
    /// This should not need to be set dynamically; an external source
    /// can set the eof flag directly if required using the
    /// [set_eof](Reader::set_eof) method
    pub fn set_eof_on_no_data(mut self, eof_on_no_data:bool) -> Self {
        self.eof_on_no_data = eof_on_no_data;
        self
    }

    //mp set_position
    /// Set the current stream position
    ///
    /// This may be used if, for example, a stream is being restarted;
    /// or if a UTF8 encoded stream occurs in the middle of a byte
    /// file.
    pub fn set_position(&mut self, stream_pos:StreamPosition) {
        self.stream_pos = stream_pos;
    }

    //mp set_eof
    /// Set the eof indicator as required; when `true` this will halt
    /// any new data being returned, and the internal buffer points
    /// will not change when more data is requested of the [Reader].
    ///
    /// This method may be invoked on behalf of a stream that has
    /// completed, but that cannot indicate this by a read operation
    /// returning zero bytes. For example, it may be used by an
    /// application which uses a TcpStream for data, and which needs
    /// to ensure future operations on the [Reader] return no more
    /// data after the TcpStream has closed.
    pub fn set_eof(&mut self, eof:bool) {
        self.eof = eof;
    }

    //mp eof
    /// Get the current eof indicator value.
    ///
    /// The `EOF` indication is normally set for [Reader]s that have a
    /// stream that returns no data on a read operation, with that
    /// behavior modified by the
    /// [set_eof_on_no_data](Reader::set_eof_on_no_data) method.
    pub fn eof(&self) -> bool {
        self.eof
    }

    //mp complete
    /// Finish with the stream, returning the buffer handle, the
    /// position of the *next* character in the stream (if there were
    /// to be one), and any unused buffer data.
    pub fn complete(self) -> (R, StreamPosition, Vec<u8>) {
        (self.buf_reader, self.stream_pos, self.current[self.start..self.end].into())
    }

    //mp drop_buffer
    /// Drop the unconsumed data, for example after it has been borrowed and used, and before [complete](Reader::complete) is invoked
    pub fn drop_buffer(&mut self) {
        self.stream_pos.move_on_bytes(self.end - self.start);
        self.start = self.end;
    }

    //mp buffer_is_empty
    /// Returns true if the internal buffer is empty
    pub fn buffer_is_empty(&self) -> bool {
        self.start == self.end
    }

    //mp borrow_buffer
    /// Borrow the data held in the [Reader]'s buffer.
    pub fn borrow_buffer(&self) -> &[u8] {
        &self.current[self.start..self.end]
    }

    //mp borrow_pos
    /// Borrow the stream position of the next character to be returned
    pub fn borrow_pos(&self) -> &StreamPosition {
        &self.stream_pos
    }

    //mp borrow
    /// Borrow the underlying stream
    pub fn borrow(&self) -> &R {
        &self.buf_reader
    }

    //mp borrow_mut
    /// Borrow the underlying stream as a mutable reference
    pub fn borrow_mut(&mut self) -> &mut R {
        &mut self.buf_reader
    }

    //fi fetch_input
    /// Fetch input from the underlying stream into the internal buffer,
    /// moving valid data to the start of the buffer first if
    /// required.  This method should only be invoked if more data is
    /// required; it is relatively code-heavy.
    fn fetch_input(&mut self) -> Result<usize> {
        if self.start>BUFFER_SIZE-BUFFER_SLACK {
            // Move everything down by self.start
            let n = self.end - self.start;
            if n>0 {
                for i in 0..n {
                    self.current[i] = self.current[self.start+i];
                }
            }
            self.valid_end -= self.start;
            self.start      = 0; // == self.start - self.start
            self.end        = n; // == self.end   - self.start
        }
        let n = self.buf_reader.read( &mut self.current[self.end..BUFFER_SIZE] )?;
        self.end += n;
        if n==0 && self.eof_on_no_data {
            self.eof = true;
        }
        Ok(n)
    }

    //mp next_char
    /// Return the next character from the stream, if one is available, or [EOF](Char::Eof).
    ///
    /// If there is no data - or not enough data - from the underlying stream, and the [Reader] is operating with the underlying stream *not* indicating EOF with a zero-byte read result, then [NoData](Char::NoData) is returned.
    ///
    /// # Errors
    ///
    /// May return [Error::MalformedUtf8] if the next bytes in the stream do not make a well-formed UTF8 character.
    ///
    /// May return [Error::IoError] if the underlying stream has an IO Error.
    pub fn next_char(&mut self) -> Result<Char> {
        if self.eof {
            Ok(Char::Eof)
        } else if self.start == self.end { // no data present, try reading data
            if self.fetch_input()? == 0 {
                Ok(Char::NoData)
            } else {
                self.next_char()
            }
        } else if self.start < self.valid_end { // there is valid UTF-8 data at buffer+self.start
            let s = {
                // std::str::from_utf8(&self.current[self.start..self.valid_end]).unwrap()
                unsafe {
                    std::str::from_utf8_unchecked(&self.current[self.start..self.valid_end])
                }
            };
            let ch = s.chars().next().unwrap();
            let n = ch.len_utf8();
            self.start += n;
            self.stream_pos.move_by(n, ch);
            Ok(Char::Char(ch))
        } else { // there is data but it may or may not be valid
            match std::str::from_utf8(&self.current[self.start..self.end]) {
                Ok(_) => { // the data is valid, mark it and the return from there
                    self.valid_end = self.end;
                    self.next_char()
                }
                Err(e) => { // the data is not all valid
                    if e.valid_up_to()>0 { // some bytes form valid UTF-8 - mark them and return that data
                        self.valid_end = self.start+e.valid_up_to();
                        self.next_char()
                    } else { // no valid data - check it is just incomplete, or an actual error
                        match e.error_len() {
                            None => { // incomplete UTF-8 fetch more
                                match self.fetch_input()? {
                                    0 => { // ... and eof reached when incomplete UTF8 is present
                                        if self.eof {
                                            Error::malformed_utf8(self.stream_pos, self.end-self.start)
                                        } else {
                                            Ok(Char::NoData)
                                        }
                                    }
                                    _ => { // ... but got more data so try that!
                                        self.next_char()
                                    }
                                }
                            }
                            Some(n) => { // Bad UTF-8 with n bytes used
                                let r = Error::malformed_utf8(self.stream_pos, n);
                                self.stream_pos.move_on_bytes(n);
                                self.start += n;
                                r
                            },
                        }
                    }
                },
            }
        }
    }

    //zz All done
}


//ip Iterator for Reader - iterate over characters
//
// allow missing doc code examples for this as it *has* an example but
// rustdoc does not pick it up.
#[allow(missing_doc_code_examples)]
impl <'a, R:std::io::Read> Iterator for &'a mut Reader<R> {
    // we will be counting with usize
    type Item = Result<char>;

    //mp next - return next character or None if end of file
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_char() {
            Ok(Char::Char(ch)) => Some(Ok(ch)),
            Ok(_)              => None,
            Err(x)             => Some(Err(x)),
        }
    }

    //zz All done
}
