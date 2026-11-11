//! DMA abstractions for memory allocation, transfer descriptors, and rings.

pub mod allocation;
pub mod descriptor;
pub mod ring;

pub use allocation::DmaAllocation;
pub use descriptor::{DmaDescriptor, DmaDirection, DmaTransferStatus};
pub use ring::DmaRing;
