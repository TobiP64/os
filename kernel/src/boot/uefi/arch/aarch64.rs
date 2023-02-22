
#[no_mangle]
pub fn AARCH64_init(
        system_table: &hw::uefi::SystemTable,
        framebuffer:  Option<crate::GenericFramebuffer>,
        memory_map:   hw::uefi::MemoryMap
) -> ! {

}