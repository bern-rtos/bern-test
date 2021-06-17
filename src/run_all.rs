/* these must be put in a linker section that does get initialized */
#[link_section = ".uninit"]
static mut TEST_SECRET: u32 = 0;
#[link_section = ".uninit"]
static mut TEST_NEXT: u8 = 0;
#[link_section = ".uninit"]
static mut TEST_SUCCESSFUL: u8 = 0;

const SECRET_NUMBER: u32 = 0x12345678;

pub fn activate() {
    unsafe {
        TEST_SECRET = SECRET_NUMBER;
        TEST_SUCCESSFUL = 0;
    }
}
pub fn deactivate() {
    unsafe { TEST_SECRET = 0; }
}

pub fn is_active() -> bool {
    unsafe { TEST_SECRET == SECRET_NUMBER }
}

pub fn get_next_test() -> u8 {
    unsafe { TEST_NEXT }
}
pub fn set_next_test(index: u8) {
    unsafe { TEST_NEXT = index; }
}

pub fn test_succeeded() {
    unsafe { TEST_SUCCESSFUL += 1; }
}
pub fn get_success_count() -> u8 {
    unsafe { TEST_SUCCESSFUL }
}