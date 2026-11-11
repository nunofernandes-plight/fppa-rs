//! PCIe Base Address Register (BAR) region metadata.

use crate::error::{FppaError, FppaResult};

/// Number of BAR registers in a conventional PCI Type 0 header.
const PCI_BAR_COUNT: u8 = 6;

/// Describes a memory-mapped PCIe BAR region.
///
/// This structure stores BAR metadata only. It does not map the BAR
/// into the process address space or provide hardware register access.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BarRegion {
    /// PCI BAR register index (0 through 5).
    index: u8,

    /// Size of the BAR region in bytes.
    size: usize,
}

impl BarRegion {
    /// Creates a BAR region description.
    ///
    /// The index must identify one of the six conventional PCI BAR
    /// registers, and the region size must be greater than zero.
    pub fn new(index: u8, size: usize) -> FppaResult<Self> {
        if index >= PCI_BAR_COUNT {
            return Err(FppaError::InvalidInput(
                "PCI BAR index must be between 0 and 5".to_string(),
            ));
        }

        if size == 0 {
            return Err(FppaError::InvalidInput(
                "PCI BAR region size must be greater than zero".to_string(),
            ));
        }

        Ok(Self { index, size })
    }

    /// Returns the PCI BAR register index.
    pub fn index(&self) -> u8 {
        self.index
    }

    /// Returns the BAR region size in bytes.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Validates whether a byte range fits within the BAR region.
    ///
    /// The offset is relative to the beginning of the BAR.
    /// A zero-length range or a range outside the BAR is rejected.
    pub fn validate_range(&self, offset: usize, length: usize) -> FppaResult<()> {
        if length == 0 {
            return Err(FppaError::InvalidInput(
                "BAR access length must be greater than zero".to_string(),
            ));
        }

        let end = offset.checked_add(length).ok_or_else(|| {
            FppaError::InvalidInput("BAR access range calculation overflowed".to_string())
        })?;

        if end > self.size {
            return Err(FppaError::InvalidInput(
                "BAR access range exceeds the region size".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_valid_bar_region() {
        let bar = BarRegion::new(0, 4096).unwrap();

        assert_eq!(bar.index(), 0);
        assert_eq!(bar.size(), 4096);
    }

    #[test]
    fn rejects_invalid_bar_index() {
        assert!(BarRegion::new(6, 4096).is_err());
    }

    #[test]
    fn rejects_zero_size() {
        assert!(BarRegion::new(0, 0).is_err());
    }

    #[test]
    fn accepts_valid_range() {
        let bar = BarRegion::new(0, 4096).unwrap();

        assert!(bar.validate_range(0, 4).is_ok());
        assert!(bar.validate_range(4092, 4).is_ok());
    }

    #[test]
    fn rejects_range_outside_bar() {
        let bar = BarRegion::new(0, 4096).unwrap();

        assert!(bar.validate_range(4094, 4).is_err());
    }

    #[test]
    fn rejects_zero_length_range() {
        let bar = BarRegion::new(0, 4096).unwrap();

        assert!(bar.validate_range(0, 0).is_err());
    }

    #[test]
    fn rejects_range_calculation_overflow() {
        let bar = BarRegion::new(0, 4096).unwrap();

        assert!(bar.validate_range(usize::MAX, 4).is_err());
    }
}
