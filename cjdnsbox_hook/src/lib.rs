extern crate libc;
#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

use libc::*;

struct Fd {
}

const FD_OFFSET: usize = 10000;
lazy_static! {
    static ref FD_TABLE: Mutex<Vec<Fd>> = Mutex::new(Vec::new());
}

#[no_mangle]
pub extern fn socket(_domain: c_int, _type: c_int, _protocol: c_int) -> c_int {
    let new_fd = Fd {};
    let mut table = FD_TABLE.lock().unwrap();
    table.push(new_fd);
    (FD_OFFSET + table.len()) as c_int
}

#[no_mangle]
pub extern fn setsockopt(
        _sockfd: c_int, _level: c_int, _optname: c_int, _optval: *const [u8], _optlen: socklen_t)
        -> c_int {
    0
}

#[no_mangle]
pub extern fn bind(_sockfd: c_int, _addr: *const sockaddr, _addrlen: socklen_t) -> c_int {
    0
}
