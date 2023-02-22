pub const INTID_IPI_HART_UP:       u32 = 0;
pub const INTID_IPI_HART_DOWN:     u32 = 1;
pub const INTID_IPI_PING:          u32 = 2;
pub const INTID_IPI_PONG:          u32 = 3;
pub const INTID_GT_SEL2_VIRTUAL:   u32 = 19;
pub const INTID_GT_SEL2_PHYSICAL:  u32 = 20;
pub const INTID_PMB_IRQ:           u32 = 21;
pub const INTID_COMM_IRQ:          u32 = 22;
pub const INTID_PM_IRQ:            u32 = 23;
pub const INTID_CTI_IRQ:           u32 = 24;
pub const INTID_GIC_MAINTENANCE:   u32 = 25;
pub const INTID_GT_NSEL2_PHYSICAL: u32 = 26;
pub const INTID_GT_EL1_VIRTUAL:    u32 = 27;
pub const INTID_GT_NSEL2_VIRTUAL:  u32 = 28;
pub const INTID_GT_EL3_PHYSICAL:   u32 = 29;
pub const INTID_GT_EL1_PHYSICAL:   u32 = 30;

#[no_mangle]
fn aarch64_int_sync() {

}

#[no_mangle]
fn aarch64_int_irq() {

}

#[no_mangle]
fn aarch64_int_fiq() {

}

#[no_mangle]
fn aarch64_int_serror() {

}

#[no_mangle]
fn aarch64_int_currrent_el_sp_el0_sync() {
    asm!("MSR SPSel, #1");
    aarch64_int_sync();
}

#[no_mangle]
fn aarch64_int_currrent_el_sp_el0_irq() {
    asm!("MSR SPSel, #1");
    aarch64_int_irq();
}

#[no_mangle]
fn aarch64_int_currrent_el_sp_el0_fiq() {
    asm!("MSR SPSel, #1");
    aarch64_int_fiq();
}

#[no_mangle]
fn aarch64_int_currrent_el_sp_el0_serror() {
    asm!("MSR SPSel, #1");
    aarch64_int_serror();
}

#[no_mangle]
fn aarch64_int_currrent_el_sp_elx_sync() {
    aarch64_int_sync();
}

#[no_mangle]
fn aarch64_int_currrent_el_sp_elx_irq() {
    aarch64_int_irq();
}

#[no_mangle]
fn aarch64_int_currrent_el_sp_elx_fiq() {
    aarch64_int_fiq();
}

#[no_mangle]
fn aarch64_int_currrent_el_sp_elx_serror() {
    aarch64_int_serror();
}

#[no_mangle]
fn aarch64_int_lower_el_aarch64_sync() {
    aarch64_int_sync();
}

#[no_mangle]
fn aarch64_int_lower_el_aarch64_irq() {
    aarch64_int_irq();
}

#[no_mangle]
fn aarch64_int_lower_el_aarch64_fiq() {
    aarch64_int_fiq();
}

#[no_mangle]
fn aarch64_int_lower_el_aarch64_serror() {
    aarch64_int_serror();
}

#[no_mangle]
fn aarch64_int_lower_el_aarch32_sync() {
    aarch64_int_sync();
}

#[no_mangle]
fn aarch64_int_lower_el_aarch32_irq() {
    aarch64_int_irq();
}

#[no_mangle]
fn aarch64_int_lower_el_aarch32_fiq() {
    aarch64_int_fiq();
}

#[no_mangle]
fn aarch64_int_lower_el_aarch32_serror() {
    aarch64_int_serror();
}