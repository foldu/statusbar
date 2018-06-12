use nom::{digit, types::CompleteStr};

named!(
    h_rgb(CompleteStr) -> (),
    do_parse!(
        char!('#') >>
        count!(digit, 6) >>
        eof!() >>
        ()
    )
);

pub fn hex_rgb(s: &str) -> Result<String, failure::Error> {
    h_rgb(CompleteStr(s))
        .map_err(|_| format_err!("Can't parse {} as hex rgb color like #001122", s))
        .map(|_| s.to_string())
}
