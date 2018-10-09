use std::collections::HashMap;

use super::unix::*;

use super::InterfaceBlacklist;

#[test]
fn if_test() {
    let mut test = HashMap::new();
    let sock = InetStreamSock::new().unwrap();
    let blacklist = InterfaceBlacklist::new();
    update_ifs(&mut test, &blacklist, sock);

    // I don't think cargo works offline so the compiling system needs to have working net
    // interface
    assert!(test.keys().count() > 0);

    assert!(test.get("lo").is_none());
    println!("{:#?}", test);

    use nix::ifaddrs::getifaddrs;
    println!("{:#?}", getifaddrs().unwrap().collect::<Vec<_>>());
}
