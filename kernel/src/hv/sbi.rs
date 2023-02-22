
struct Result {
    error: u64,
    value: u64
}

#[no_mangle]
pub static SBI_TABLE: [(u32, u32, *mut extern fn()); 20] = unsafe { [
    (0x0000_0010, 6, &SBI_BASE_TABLE)
] };

#[no_mangle]
pub static SBI_BASE_TABLE: [extern fn(); 6] = unsafe { [
    sbi_base_get_sbi_specification_version
] };

extern fn sbi_base_get_sbi_specification_version() -> Result {

}