//a Imports
use utf8_read::{Reader};
mod utils;
use utils::StopStart;

fn test_string_whole(buf:&str) {
    let char_list : Vec<char> = buf.chars().collect();

    let mut buf_bytes : &[u8] = buf.as_bytes();

    let mut reader = Reader::new(&mut buf_bytes);
    let mut n = 0;
    for (i,ch) in reader.enumerate() {
        assert!(ch.is_ok(), "No errors expected in this test");
        let ch = ch.unwrap();
        assert_eq!(ch, char_list[i], "Mismatch in characters from string {} {}", char_list[i], ch );
        n += 1;
    }
    assert_eq!(n, char_list.len(), "Must have consumed the whole string");
}

fn test_string_stop_start(buf:&str) {
    let char_list : Vec<char> = buf.chars().collect();

    let mut buf_bytes : &[u8] = buf.as_bytes();

    let mut reader = Reader::new(StopStart::new(&mut buf_bytes, 17)).set_eof_on_no_data(false);
    let mut n = 0;
    loop {
        reader.borrow_mut().kick();
        for ch in reader.into_iter() {
            assert!(ch.is_ok(), "No errors expected in this test");
            let ch = ch.unwrap();
            // println!("{} {}",n,ch);
            assert_eq!(ch, char_list[n], "Mismatch in characters {} from string {} {}", n, char_list[n], ch );
            n += 1;
        }
        if reader.borrow().is_eof() {break;}
    }
    assert_eq!(n, char_list.len(), "Must have consumed the whole string");
}

fn test_string_stop_start2(buf:&str) {
    let char_list : Vec<char> = buf.chars().collect();

    let mut buf_bytes : &[u8] = buf.as_bytes();

    let mut reader = Reader::new(StopStart::new(&mut buf_bytes, 17)).set_eof_on_no_data(false);
    let mut n = 0;
    loop {
        use utf8_read::Char;
        match reader.next_char() {
            Ok(Char::NoData) => {
                reader.borrow_mut().kick();
                if reader.borrow_mut().is_eof() {
                    reader.set_eof(true);
                }
            }
            Ok(Char::Char(ch)) => {
                assert_eq!(ch, char_list[n], "Mismatch in characters {} from string {} {}", n, char_list[n], ch );
                n += 1;
            }
            Ok(_) => {
                break;
            }
            Err(_) => {
                assert!(false, "Unexpected error returned");
            }
        }
    }
    assert_eq!(n, char_list.len(), "Must have consumed the whole string");
}

fn test_string(buf:&str) {
    test_string_whole(buf);
    test_string_stop_start(buf);
    test_string_stop_start2(buf);

    let mut a_long_string : String = buf.into();

    for _ in 0..8 {
        a_long_string = String::new() + &a_long_string + &a_long_string;
        test_string_whole(&a_long_string);
        test_string_stop_start(&a_long_string);
        test_string_stop_start2(&a_long_string);
    }
}

#[test]
fn test_hello() {
    test_string("Hello");
}

#[test]
fn test_longer() {
    test_string("13 characters");
}

#[test]
fn test_utf8_short() {
    test_string("\u{2764} \u{1f600}");

}

#[test]
fn test_utf8_long() {
    test_string("This is a \u{2764} string\nWith a newline\n\u{0065}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}");

}

