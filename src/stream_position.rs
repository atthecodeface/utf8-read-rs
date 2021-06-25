//a StreamPosition
/// This representes the position of a character within a UTF8 stream
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct StreamPosition {
    /// Byte offset from start of file - starting at 0
    byte     : usize,
    /// Line number in the file - starting at 1
    line_num : usize,
    /// Character offset within the file - starting at 1
    char_ofs : usize,
}

impl StreamPosition {
    //fp new
    /// Constructs a new [StreamPosition] for the default of byte 0,
    /// first line, first character
    pub fn new() -> Self {
        Self { byte:0, line_num:1, char_ofs:1 }
    }

    //fp of_blc
    /// Construct a new [StreamPosition] from byte, line and character offset
    pub fn of_blc(byte:usize, line_num:usize, char_ofs:usize) -> Self {
        Self { byte, line_num, char_ofs }
    }

    //mp move_on_bytes
    /// Move the byte count on (to get past a bad UTF encoding, for example)
    #[inline]
    pub(crate) fn move_on_bytes(&mut self, n:usize) {
        self.byte += n;
    }

    //mp move_by
    /// Move the [StreamPosition] on by a number of bytes, and a
    /// particular character
    #[inline]
    pub(crate) fn move_by(&mut self, n:usize, ch:char) {
        self.byte += n;
        match ch {
            '\n' => {
                self.line_num += 1;
                self.char_ofs = 1;
            }
            _ => {
                self.char_ofs += 1;
            }
        }
    }

    //mp byte
    /// Find the byte that the [StreamPosition] holds
    #[inline]
    pub fn byte(&self) -> usize {
        self.byte
    }


    //mp line_position
    /// Get the line number and character within the line of the [StreamPosition]
    #[inline]
    pub fn line_position(&self) -> (usize, usize) {
        (self.line_num, self.char_ofs)
    }

    //zz All done
}


//ip Display for StreamPosition
impl std::fmt::Display for StreamPosition {
    //mp fmt - format for humans
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "line {} char {}", self.line_num, self.char_ofs)
    }
}


