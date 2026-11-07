//! DMA memory allocation abstractions.

use crate::error::{FppaError, FppaResult};

/// Describes the metadata associated with a DMA memory allocation.
///
/// This type does not allocate memory or establish DMA mappings.
///
/// CPU virtual addresses, physical addresses, and device-visible I/O
/// virtual addresses (IOVAs) are distinct address spaces. They must not
/// be treated as interchangeable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DmaAllocation {
    /// Size of the allocation in bytes.
    size: usize,

    /// Required memory alignment in bytes.
    alignment: usize,
}

impl DmaAllocation {
    /// Creates a new DMA allocation description.
    ///
    /// The size must be greater than zero.
    /// The alignment must be a non-zero power of two.
    pub fn new(size: usize, alignment: usize) -> FppaResult<Self> {
        if size == 0 {
            return Err(FppaError::InvalidInput(
                "DMA allocation size must be greater than zero".to_string(),
            ));
        }

        if alignment == 0 || !alignment.is_power_of_two() {
            return Err(FppaError::InvalidInput(
                "DMA allocation alignment must be a non-zero power of two"
                    .to_string(),
            ));
        }

        Ok(Self { size, alignment })
    }

    /// Returns the allocation size in bytes.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the required memory alignment in bytes.
    pub fn alignment(&self) -> usize {
        self.alignment
    }
}
