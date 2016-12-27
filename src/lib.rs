



/// pad a string to the left to ``pad`` length with spaces
/// If str.len() is not less than pad, then the string is returned verbatim
pub fn left_pad(s: &str, pad: usize) -> String
{
    left_pad_char(s, pad, ' ')
}

/// pad a string to the left to ``pad`` length with ``padchar``
/// If str.len() is not less than pad, then the string is returned verbatim
pub fn left_pad_char(s: &str, pad: usize, padchar: char) -> String
{
    let len = s.len();
    if pad > len {
        let left_pad = pad - len;
        let capacity = s.len() + left_pad * padchar.len_utf8();
        let mut out = String::with_capacity(capacity);

        for _ in 0..left_pad {
            out.push(padchar);
        }
        out.push_str(s);

        out
    } else {
        s.to_owned()
    }
}

#[cfg(test)]
#[test]
fn pad_test() {

    assert_eq!(left_pad("foo", 0), "foo");
    assert_eq!(left_pad("foo", 2), "foo");
    assert_eq!(left_pad_char("bar", 0, 'Y'), "bar");
    assert_eq!(left_pad("foo", 5), "  foo");
    assert_eq!(left_pad_char("foo", 7, 'X'), "XXXXfoo");
    assert_eq!(left_pad_char("bar", 5, '-'), "--bar");
}

