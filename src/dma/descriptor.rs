//! DMA transfer descriptor abstractions.

use crate::error::{FppaError, FppaResult};

/// Describes the direction of a DMA transfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaDirection {
    /// Transfers data from host memory to the device.
    ToDevice,

    /// Transfers data from the device to host memory.
    FromDevice,
}

/// Represents the software-side status of a DMA transfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaTransferStatus {
    /// The descriptor has been created but not submitted.
    Prepared,

    /// The transfer has been submitted for execution.
    Submitted,

    /// The transfer has completed successfully.
    Completed,

    /// The transfer has failed.
    Failed,
}

/// Describes a single DMA transfer operation.
///
/// This type represents transfer metadata only. It does not submit
/// transfers or interact directly with DMA hardware.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DmaDescriptor {
    /// Number of bytes to transfer.
    length: usize,

    /// Direction of the transfer.
    direction: DmaDirection,

    /// Current software-side transfer status.
    status: DmaTransferStatus,
}

impl DmaDescriptor {
    /// Creates a new DMA transfer descriptor.
    ///
    /// The transfer length must be greater than zero.
    pub fn new(length: usize, direction: DmaDirection) -> FppaResult<Self> {
        if length == 0 {
            return Err(FppaError::InvalidInput(
                "DMA transfer length must be greater than zero".to_string(),
            ));
        }

        Ok(Self {
            length,
            direction,
            status: DmaTransferStatus::Prepared,
        })
    }

    /// Returns the transfer length in bytes.
    pub fn length(&self) -> usize {
        self.length
    }

    /// Returns the transfer direction.
    pub fn direction(&self) -> DmaDirection {
        self.direction
    }

    /// Returns the current transfer status.
    pub fn status(&self) -> DmaTransferStatus {
        self.status
    }

    /// Marks the descriptor as submitted.
    pub fn mark_submitted(&mut self) -> FppaResult<()> {
        if self.status != DmaTransferStatus::Prepared {
            return Err(FppaError::InvalidState(
                "only prepared DMA transfers can be submitted".to_string(),
            ));
        }

        self.status = DmaTransferStatus::Submitted;

        Ok(())
    }

    /// Marks the transfer as successfully completed.
    pub fn mark_completed(&mut self) -> FppaResult<()> {
        if self.status != DmaTransferStatus::Submitted {
            return Err(FppaError::InvalidState(
                "only submitted DMA transfers can be completed".to_string(),
            ));
        }

        self.status = DmaTransferStatus::Completed;

        Ok(())
    }

    /// Marks the transfer as failed.
    pub fn mark_failed(&mut self) -> FppaResult<()> {
        if self.status != DmaTransferStatus::Submitted {
            return Err(FppaError::InvalidState(
                "only submitted DMA transfers can be marked as failed".to_string(),
            ));
        }

        self.status = DmaTransferStatus::Failed;

        Ok(())
    }
}
