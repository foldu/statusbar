use failure::format_err;
use nom::{types::CompleteStr, *};

fn is_hex_digit_c(c: char) -> bool {
    nom::is_hex_digit(c as u8)
}

named!(
    h_rgb(CompleteStr) -> (),
    do_parse!(
        char!('#') >>
        take_while_m_n!(6, 6, is_hex_digit_c) >>
        eof!() >>
        ()
    )
);

pub fn hex_rgb(s: &str) -> Result<String, failure::Error> {
    h_rgb(CompleteStr(s))
        .map_err(|_| format_err!("Can't parse {} as hex rgb color like #001122", s))
        .map(|_| s.to_string())
}

#[test]
fn test_hex_rgb() {
    assert!(hex_rgb("#000000").is_ok());
    assert!(hex_rgb("000000").is_err());
    assert!(hex_rgb("#FF0033").is_ok());
    assert!(hex_rgb("#ff0033").is_ok());
}
