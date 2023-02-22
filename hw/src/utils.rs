
#[derive(Copy, Clone, Debug, Default)]
pub struct Ptr32<T> {
    ptr:    u32,
    marker: PhantomData<T>,
}

impl<T> Ptr32<T> {
    pub fn get(self) -> *mut T {
        self.ptr as usize as _
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Ptr64<T> {
    ptr:    u64,
	marker: PhantomData<T>,
}

impl<T> Ptr64<T> {
    pub fn get(self) -> *mut T {
        self.ptr as usize as _
    }
}

#[macro_export]
macro_rules! define_ro_bit {
    ( $getter_name:ident, $bit:expr ) => {
		pub fn $getter_name(&self) -> bool {
			self.0 & !(1 << $bit) != 0
		}
	};
}

#[macro_export]
macro_rules! define_wo_bit {
    ( $setter_name:ident, $bit:expr ) => {
		pub fn $setter_name(&mut self, v: bool) {
			if v {
				self.0 |= 1 << $bit;
			} else {
				self.0 &= 1 << !$bit;
			}
		}
	};
}

#[macro_export]
macro_rules! define_rw_bit {
    ( $getter_name:ident, $setter_name:ident, $bit:expr ) => {
		define_ro_bit!($getter_name, $bit);
		define_wo_bit!($setter_name, $bit);
	};
}

macro_rules! define_bits {
    ( $( $tt:tt );* ) => {
		$( define_bits!( $tt ); )*
	};
	( RO $bit:expr:$end:expr, $getter:ident ) => {
		pub fn $getter(&self) -> bool {
			self.0 & !(1 << $bit) != 0
		}
	};
	( WO $bit:expr, $setter:ident ) => {
		pub fn $setter(&self) -> bool {
			self.0 & !(1 << $bit) != 0
		}
	};
    ( RW $bit:expr, $getter:ident, $setter:ident ) => {
		define_bits!(RO $bit, $getter);
		define_bits!(WO $bit, $setter);
	};( RO $bit:expr, $getter:ident ) => {
		pub fn $getter(&self) -> bool {
			self.0 & !(1 << $bit) != 0
		}
	};
	( WO $bit:expr, $setter:ident ) => {
		pub fn $setter(&self) -> bool {
			self.0 & !(1 << $bit) != 0
		}
	};
    ( RW $bit:expr, $getter:ident, $setter:ident ) => {
		define_bits!(RO $bit, $getter);
		define_bits!(WO $bit, $setter);
	};
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptr32<T>(u32, core::marker::PhantomData<T>);

impl<T> Ptr32<T> {
	pub fn new(ptr: *mut T) -> Self {
		Self(ptr as usize as _, core::marker::PhantomData)
	}

	pub unsafe fn as_ref<'a>(&self) -> Option<&'a T> {
		(self.0 as usize as *const T).as_ref()
	}

	pub unsafe fn as_mut<'a>(&mut self) -> Option<&'a mut T> {
		(self.0 as usize as *mut T).as_mut()
	}

	pub fn as_ptr(&self) -> *const T {
		self.0 as usize as *const T
	}

	pub fn as_mut_ptr(&mut self) -> *mut T {
		self.0 as usize as *mut T
	}
}

impl<T> Default for Ptr32<T> {
	fn default() -> Self {
		Self::new(core::ptr::null_mut())
	}
}

impl<T: core::fmt::Debug> core::fmt::Debug for Ptr32<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		if core::mem::size_of::<T>() == 0 {
			write!(f, "{:?}", self.0 as usize as *const T)
		} else {
			unsafe { (self.0 as usize as *const T).as_ref() }.fmt(f)
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptr64<T>(u64, core::marker::PhantomData<T>);

impl<T> Ptr64<T> {
	pub fn new(ptr: *mut T) -> Self {
		Self(ptr as usize as _, core::marker::PhantomData)
	}

	pub unsafe fn as_ref<'a>(&self) -> Option<&'a T> {
		(self.0 as usize as *const T).as_ref()
	}

	pub unsafe fn as_mut<'a>(&mut self) -> Option<&'a mut T> {
		(self.0 as usize as *mut T).as_mut()
	}

	pub fn as_ptr(&self) -> *const T {
		self.0 as usize as *const T
	}

	pub fn as_mut_ptr(&mut self) -> *mut T {
		self.0 as usize as *mut T
	}
}

impl<T> Default for Ptr64<T> {
	fn default() -> Self {
		Self::new(core::ptr::null_mut())
	}
}

impl<T: core::fmt::Debug> core::fmt::Debug for Ptr64<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		if core::mem::size_of::<T>() == 0 {
			write!(f, "{:?}", self.0 as usize as *const T)
		} else {
			unsafe { (self.0 as usize as *const T).as_ref() }.fmt(f)
		}
	}
}

pub struct DbgHex<T: core::fmt::UpperHex>(pub T);

impl<T: core::fmt::UpperHex> core::fmt::Debug for DbgHex<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{:#X}", self.0)
	}
}

pub struct DbgBin<T: core::fmt::Binary>(pub T);

impl<T: core::fmt::Binary> core::fmt::Debug for DbgBin<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{:#b}", self.0)
	}
}