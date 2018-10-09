use std::{convert::TryFrom, mem, ptr};

use linux_wireless::{iwreq, IFNAMSIZ, SIOCGIWNAME};
use nix::{convert_ioctl_res, ioctl_read_bad};

use super::unix::InetStreamSock;

ioctl_read_bad!(siocgiwname, SIOCGIWNAME, iwreq);

pub fn is_wireless_if(sock: InetStreamSock, if_: &str) -> bool {
    let mut req: iwreq = unsafe { mem::zeroed() };

    assert!(
        u32::try_from(if_.len())
            .map(|len| len < IFNAMSIZ)
            .unwrap_or(false)
    );

    // safe because of assertion
    unsafe {
        let if_name = req.ifr_ifrn.ifrn_name.as_mut_ptr() as *mut u8;
        ptr::copy_nonoverlapping(if_.as_bytes().as_ptr(), if_name, if_.len());
        *if_name.offset(if_.len() as isize + 1) = 0;
        siocgiwname(sock.fd(), &mut req)
            .map(|ret| ret != -1)
            .unwrap_or(false)
    }
}
