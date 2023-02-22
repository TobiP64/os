use crate::*;

#[derive(Clone)]
pub struct Node {
    pub parent: *mut Self,
	pub flags:  u32,
	pub refs:   u32,
	/// Pointer to the linked node if FLAG_LINK is set
	pub pages:  *mut mem::PageDescriptor,
	pub users:  *mut ctx::ResourceDescriptor,
}

impl Node {
	// For mount namespaces only, root FS is always RWX
	pub const FLAG_READ:   u32 = 1 << 0;
	pub const FLAG_WRITE:  u32 = 1 << 1;
	pub const FLAG_EXEC:   u32 = 1 << 2;

    pub const TYPE_LINK:       u32 = 0x1;
    pub const TYPE_CACHED:     u32 = 0x2;
    pub const TYPE_PERIPHERAL: u32 = 0x3;
    pub const TYPE_PIPE:       u32 = 0x3;
}