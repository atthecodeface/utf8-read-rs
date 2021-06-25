//a Imports
use utf8_read::{Error, StreamPosition, Reader};

fn test_buf_exp(buf_bytes : &[u8], expectation :&[Result<char, Error>]) {
    let mut reader = Reader::new(buf_bytes);
    let ch : Vec<Result<char, Error>> = reader.into_iter().collect();
    let mut last_n = 0;
    for (n,(a,b)) in ch.iter().zip(expectation).enumerate() {
        match (a, b) {
            (Ok(x), Ok(y))  => {assert_eq!(x,y, "Mismatch in character for {}", n);}
            ( Err(Error::MalformedUtf8(p0,n0)), Err(Error::MalformedUtf8(p1,n1)) ) => {
                assert_eq!(p0,p1,"Mismatch in stream positions for errors for {}",n);
                assert_eq!(n0,n1,"Mismatch in byte length for errors for {}",n);
            },
            _ => {
                assert!(false, "Mismatch in expectation for {} : {:?} != {:?}", n, a, b);
            }
        }
        last_n = n;
    }
    assert_eq!(last_n+1, expectation.len(), "All expectation not met");
}

#[test]
fn test_me() {
    test_buf_exp("a".as_bytes(), &[Ok('a')]);
    test_buf_exp(b"\x80  ", &[
        Err(Error::MalformedUtf8(StreamPosition::of_blc(0,1,1),1)),
        Ok(' '), Ok(' ')
            ]);
    test_buf_exp(b" \x80", &[
        Ok(' '),
        Err(Error::MalformedUtf8(StreamPosition::of_blc(1,1,2),1)),
            ]);

    test_buf_exp(b" \xc0", &[
        Ok(' '),
        Err(Error::MalformedUtf8(StreamPosition::of_blc(1,1,2),1)),
    ]);

    test_buf_exp(b"1\xc2\x80345\x80", &[
        Ok('1'),
        Ok('\u{80}'),
        Ok('3'),
        Ok('4'),
        Ok('5'),
        Err(Error::MalformedUtf8(StreamPosition::of_blc(6,1,6),1)),
    ]);

    // 0xc0 0x80 is an overlong encoding
    // Rust utf8 indicates the 0xc0 is a single byte UTF8 error
    // Then the 0x80 will be a single byte UTF8 error too
    test_buf_exp(b" \xc0\x80", &[
        Ok(' '),
        Err(Error::MalformedUtf8(StreamPosition::of_blc(1,1,2),1)),
        Err(Error::MalformedUtf8(StreamPosition::of_blc(2,1,2),1)),
            ]);

    // 0xe0 0x80 expects another 0x8_ - it is a short encoding
    // Rust utf8 indicates the 0xe0 is a single byte UTF8 error
    // Then the 0x80 will be a single byte UTF8 error too
    // The following byte is not an error - it is a space
    test_buf_exp(b" \xe0\x80\x20", &[
        Ok(' '),
        Err(Error::MalformedUtf8(StreamPosition::of_blc(1,1,2),1)),
        Err(Error::MalformedUtf8(StreamPosition::of_blc(2,1,2),1)),
        Ok(' '),
            ]);

    test_buf_exp(b"Hello\nFun!\n123\xc0", &[
        Ok('H'),Ok('e'),Ok('l'),Ok('l'),Ok('o'),Ok('\n'),
        Ok('F'),Ok('u'),Ok('n'),Ok('!'),Ok('\n'),
        Ok('1'),Ok('2'),Ok('3'),
        Err(Error::MalformedUtf8(StreamPosition::of_blc(14,3,4),1)),
    ]);

    test_buf_exp(b"Hello\nFun!\n123\xc0", &[
        Ok('H'),Ok('e'),Ok('l'),Ok('l'),Ok('o'),Ok('\n'),
        Ok('F'),Ok('u'),Ok('n'),Ok('!'),Ok('\n'),
        Ok('1'),Ok('2'),Ok('3'),
        Err(Error::MalformedUtf8(StreamPosition::of_blc(14,3,4),1)),
    ]);

}
