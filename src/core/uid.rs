
extern "C" {
    fn geteuid() -> u32;
    fn getegid() -> u32;
}

pub fn is_root() -> bool {
    unsafe {
        geteuid() == 0
    }
}