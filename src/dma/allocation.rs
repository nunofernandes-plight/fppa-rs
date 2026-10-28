
 //! DMA memory allocation abstractions.

/// Describes the address information associated with a DMA allocation.
///
/// CPU virtual addresses, physical addresses, and device-visible I/O
/// virtual addresses (IOVAs) are distinct address spaces. They must not
/// be treated as interchangeable.
pub struct DmaAllocation {
    /// Size of the allocation in bytes.
    pub size: usize,
}
