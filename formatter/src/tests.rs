use super::*;

#[test]
fn parser_parse() {
    assert!(parser::parse("{{test }}this {test:.3}").is_ok());
    assert!(parser::parse("test this {test}").is_ok());
    assert!(parser::parse("{lol}test this {test}").is_ok());
    assert!(parser::parse("{lol:-3B.2}test this {test}").is_ok());
    assert!(parser::parse("").is_ok());
}

#[test]
fn formatstring_parse() {
    assert!(FormatString::parse_with_allowed_keys("{toast:.3}", &["test"]).is_err());
    assert!(FormatString::parse_with_allowed_keys("{test}", &["test"]).is_ok());
    assert!(FormatString::parse_with_allowed_keys("{toast:B.3}", &["toast"]).is_ok());
}

#[test]
fn format() {
    // basic formatting works
    let fmt = FormatString::parse("{test:S.2} this").unwrap();
    let mut map = FormatMap::new();
    map.insert("test", 4000.0);
    assert_eq!("4.00k this", &fmt.fmt(&map).unwrap().to_string());

    let fmt = FormatString::parse("{test:B.2} this").unwrap();
    map.insert("test", 4096.0);
    assert_eq!("4.00ki this", &fmt.fmt(&map).unwrap().to_string());

    map.insert("test", 100.0);
    let fmt = FormatString::parse("{test:S.2} this").unwrap();
    assert_eq!("100.00 this", &fmt.fmt(&map).unwrap().to_string());

    map.insert("test", 0.0);
    let fmt = FormatString::parse("{test:S.2} this").unwrap();
    assert_eq!("0.00 this", &fmt.fmt(&map).unwrap().to_string());

    map.insert("test", "asdf".to_owned());
    let fmt = FormatString::parse("{test:-2} this").unwrap();
    assert_eq!("as this", &fmt.fmt(&map).unwrap().to_string());

    map.insert("test", "asdf".to_owned());
    let fmt = FormatString::parse("{test:-20} this").unwrap();
    assert_eq!("asdf this", &fmt.fmt(&map).unwrap().to_string());

    map.insert("test", -20.0);
    let fmt = FormatString::parse("{test:.2} this").unwrap();
    assert_eq!("-20.00 this", &fmt.fmt(&map).unwrap().to_string());
}
