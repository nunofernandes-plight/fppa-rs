//! FPPA-specific hardware abstractions.

use crate::error::{FppaError, FppaResult};
use std::time::Duration;

/// Configuration for host-side FPPA device management.
#[derive(Debug, Clone)]
pub struct FppaDeviceConfig {
    initialization_timeout: Duration,
    device_label: Option<String>,
}
/// Represents the operational state of an FPPA device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FppaDeviceState {
    /// The device has been created but not initialized.
    Uninitialized,

    /// Hardware initialization is in progress.
    Initializing,

    /// Hardware initialization has completed successfully.
    Ready,

    /// Hardware initialization has failed.
    Failed,
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
/// Changes the device state after validating the transition.
pub(crate) fn transition_to(
    &mut self,
    next_state: FppaDeviceState,
) -> FppaResult<()> {
    let valid_transition = matches!(
        (self.state, next_state),
        (FppaDeviceState::Uninitialized, FppaDeviceState::Initializing)
            | (FppaDeviceState::Initializing, FppaDeviceState::Ready)
            | (FppaDeviceState::Initializing, FppaDeviceState::Failed)
            | (FppaDeviceState::Failed, FppaDeviceState::Initializing)
    );

    if !valid_transition {
        return Err(FppaError::InvalidState(format!(
            "Invalid FPPA device state transition: {:?} -> {:?}",
            self.state, next_state
        )));
    }

    self.state = next_state;

    Ok(())
}
    
}  



