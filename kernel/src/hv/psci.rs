
#[no_mangle]
pub static PCSI_TABLE: [extern fn() -> u64; 20] = unsafe { [
    pcsi_psci_version,
    psci_cpu_suspend
] };

extern fn pcsi_psci_version(power_state: u32) {

}

extern fn psci_cpu_suspend(power_state: u32, entry_point_address: u64, context_id: u64) -> u64 {

}

