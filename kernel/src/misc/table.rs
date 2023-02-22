
pub type Table<T>                  = *mut SparseIndexTableLevel0<T>;
pub type SparseIndexTableLevel0<T> = [*mut SparseIndexTableLevel1<T>; 512];
pub type SparseIndexTableLevel1<T> = [*mut SparseIndexTableLevel2<T>; 512];
pub type SparseIndexTableLevel2<T> = [*mut T; 512];