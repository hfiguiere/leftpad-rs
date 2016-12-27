



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
    // pad_len is the number of padchar that actually need to be put before the string.
    let pad_len = pad.saturating_sub(s.len());
    let required_capacity = s.as_bytes().len() + (pad_len as usize) * padchar.len_utf8();
    let mut out = String::with_capacity(required_capacity);

    for _ in 0..pad_len {
        out.push(padchar);
    }
    out.push_str(s);
    // Assert a correct prediction of the required capacity.
    debug_assert_eq!(out.as_bytes().len(), required_capacity);
    // Assert no further allocation was done.
    debug_assert_eq!(out.capacity(), required_capacity);

    out
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
    assert_eq!(left_pad_char("rust", 7, '\u{2764}'), "❤❤❤rust");
}

