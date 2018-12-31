use std::{mem, ptr};

use linux_wireless::{iwreq, IFNAMSIZ, SIOCGIWNAME};
use nix::{convert_ioctl_res, ioctl_read_bad};

use super::unix::InetStreamSock;

ioctl_read_bad!(siocgiwname, SIOCGIWNAME, iwreq);

pub fn is_wireless_if(sock: InetStreamSock, if_: &str) -> bool {
    unsafe {
        let mut req: iwreq = mem::zeroed();
        let if_name = req.ifr_ifrn.ifrn_name.as_mut_ptr() as *mut u8;
        // memcpy and deref safe because copy_len can't be bigger than IFNAMSIZ-1
        // ifrn_name is zero terminated because the entire struct is initialized to zero
        let copy_len = if_.len().min(IFNAMSIZ.checked_sub(1).unwrap() as usize);
        ptr::copy_nonoverlapping(if_.as_bytes().as_ptr(), if_name, copy_len);
        siocgiwname(sock.fd(), &mut req)
            .map(|ret| ret != -1)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_wireless_if_doesnt_segfault() {
        let stream = InetStreamSock::new().unwrap();
        let really_long_string: String = std::iter::repeat('a').take(20000).collect();
        is_wireless_if(stream, &really_long_string);
        is_wireless_if(stream, "");
        is_wireless_if(stream, "aaaaa");
    }
}
