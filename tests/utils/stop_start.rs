pub struct StopStart<R:std::io::Read> {
    reader : R,
    num_per_read : usize,
    offset : usize,
    eof : bool,
}
impl <R:std::io::Read> StopStart<R> {
    pub fn new(reader:R, num_per_read:usize) -> Self {
        let offset = 0;
        let eof = false;
        Self {reader, num_per_read, offset, eof}
    }
    pub fn kick(&mut self) {
        self.offset = 0;
    }
    pub fn is_eof(&self) -> bool{
        self.eof
    }
}

impl <R:std::io::Read> std::io::Read for StopStart<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.offset == self.num_per_read {
            self.offset = 0;
            Ok(0)
        } else {
            let n = self.num_per_read - self.offset;
            if n > buf.len() {
                match self.reader.read(buf) {
                    Ok(n) => {
                        self.offset += n;
                        self.eof = n==0;
                        Ok(n)
                    },
                    x => x,
                }
            } else {
                let buf : &mut[u8]= &mut buf[..n];
                match self.reader.read(buf) {
                    Ok(n) => {
                        self.offset += n;
                        self.eof = n==0;
                        Ok(n)
                    },
                    x => x,
                }
            }
        }
    }
}
