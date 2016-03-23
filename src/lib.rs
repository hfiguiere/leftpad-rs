



/// pad a string to the left with ``pad`` spaces
pub fn left_pad(s: &str, pad: u32) -> String
{
    left_pad_char(s, pad, ' ')
}

/// pad a string to the left with ``pad`` ``padchar``
pub fn left_pad_char(s: &str, pad: u32, padchar: char) -> String
{
    let mut out = String::new();

    for _ in 0..pad {
        out.push(padchar);
    }
    out.push_str(s);

    out
}

#[cfg(test)]
#[test]
fn pad_test() {

    assert_eq!(left_pad("foo", 2), "  foo");
    assert_eq!(left_pad_char("foo", 2, 'X'), "XXfoo");
    assert_eq!(left_pad_char("bar", 5, '-'), "-----bar");
}

