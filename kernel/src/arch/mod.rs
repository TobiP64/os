
#[cfg(target_arch = "x86_64")]
pub mod amd64;
#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64", target_arch = "riscv128"))]
pub mod riscv64;

#[cfg(target_arch = "x86_64")]
pub use amd64::*;
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64", target_arch = "riscv128"))]
pub use riscv64::*;