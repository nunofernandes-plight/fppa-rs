//! Device-level abstractions for VFIO-managed PCIe devices.

// Linux VFIO device-handle abstractions.

#[cfg(target_os = "linux")]
use std::os::fd::{AsFd, BorrowedFd, OwnedFd};

/// Represents an opened VFIO device.
///
/// The device file descriptor is owned by this structure
/// and is closed automatically when the structure is dropped.
#[cfg(target_os = "linux")]
#[derive(Debug)]
pub struct VfioDevice {
    fd: OwnedFd,
}

#[cfg(target_os = "linux")]
impl VfioDevice {
    /// Creates a VFIO device handle from an owned file descriptor.
    ///
    /// The caller must obtain this descriptor through a valid
    /// VFIO device-access procedure.
    pub(crate) fn from_owned_fd(fd: OwnedFd) -> Self {
        Self { fd }
    }
}

#[cfg(target_os = "linux")]
impl AsFd for VfioDevice {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.fd.as_fd()
    }
}
