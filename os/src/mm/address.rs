#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysAddr(pub usize);

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtAddr(pub usize);

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysPageNum(pub usize);

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtPageNum(pub usize);

/// T: {PhysAddr, VirtAddr, PhysPageNum, VirtPageNum}
/// T -> usize: T.0
/// usize -> T: usize.into()

impl From<usize> for PhysAddr {
    fn from(v: usize) -> Self { Self(v) }
}
impl From<usize> for PhysPageNum {
    fn from(v: usize) -> Self { Self(v) }
}
impl From<usize> for VirtAddr {
    fn from(v: usize) -> Self { Self(v) }
}
impl From<usize> for VirtPageNum {
    fn from(v: usize) -> Self { Self(v) }
}
impl From<PhysAddr> for usize {
    fn from(v: PhysAddr) -> Self { v.0 }
}
impl From<PhysPageNum> for usize {
    fn from(v: PhysPageNum) -> Self { v.0 }
}
impl From<VirtAddr> for usize {
    fn from(v: VirtAddr) -> Self { v.0 }
}
impl From<VirtPageNum> for usize {
    fn from(v: VirtPageNum) -> Self { v.0 }
}
