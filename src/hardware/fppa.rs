//! FPPA-specific hardware abstractions.

/// Represents the operational state of an FPPA device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FppaDeviceState {
    /// The device has been created but not initialized.
    Uninitialized,

    /// The device has been initialized and is ready for operation.
    Ready,
}

/// Represents an FPPA hardware device.
#[derive(Debug)]
pub struct FppaDevice {
    state: FppaDeviceState,
}

impl FppaDevice {
    /// Creates a new FPPA device in the uninitialized state.
    pub fn new() -> Self {
        Self {
            state: FppaDeviceState::Uninitialized,
        }
    }
  /// Returns the current device state.
pub fn state(&self) -> FppaDeviceState {
    self.state
  }

}  



