
pub type SvcId   = usize;
pub type Status  = usize;
pub type Flags   = usize;
pub type Rd      = usize;
pub type IoOpId  = usize;
pub type CtxId   = usize;

#[no_mangle]
pub static SVC_TABLE: [fn (usize, usize, usize, usize, usize, usize) -> (usize, usize, usize, usize);  0] = [

];