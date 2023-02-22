
#[no_mangle]
pub static UEFI_TABLE: [extern fn() -> hw::uefi::Status; 20] = unsafe { [
    uefi_create_event,
    uefi_create_event_ex
] };

extern fn uefi_create_event(r#type: u32, ) -> hw::uefi::Status {

}

extern fn uefi_create_event_ex(r#type: u32, ) -> hw::uefi::Status {

}
