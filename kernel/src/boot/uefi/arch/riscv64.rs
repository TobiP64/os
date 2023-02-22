
#[no_mangle]
pub fn RISCV64_init(
        system_table: &hw::uefi::SystemTable,
        framebuffer:  Option<crate::GenericFramebuffer>,
        memory_map:   hw::uefi::MemoryMap
) -> ! {

}