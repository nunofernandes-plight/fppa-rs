//! Reusable Rust library for programmable photonic hardware infrastructure.
// Public module declarations.

pub mod error;

pub use error::{FppaError, FppaResult};

pub mod dma;
pub mod hardware;
pub mod iommu;
pub mod mmio;
pub mod vfio;
