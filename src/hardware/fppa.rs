
//! FPPA-specific hardware abstractions.

use crate::error::{FppaError, FppaResult};
use std::time::Duration;

/// Configuration for host-side FPPA device management.
#[derive(Debug, Clone)]
pub struct FppaDeviceConfig {
    initialization_timeout: Duration,
    device_label: Option<String>,
}

impl FppaDeviceConfig {
    /// Creates a new configuration with the specified initialization timeout
    /// and optional device label.
    pub fn new(
        initialization_timeout: Duration,
        device_label: Option<String>,
    ) -> FppaResult<Self> {
        if initialization_timeout.is_zero() {
            return Err(FppaError::InvalidInput(
                "Initialization timeout must be greater than zero".to_string(),
            ));
        }

        Ok(Self {
            initialization_timeout,
            device_label,
        })
    }

    /// Returns the initialization timeout.
    pub fn initialization_timeout(&self) -> Duration {
        self.initialization_timeout
    }

    /// Returns the optional device label.
    pub fn device_label(&self) -> Option<&str> {
        self.device_label.as_deref()
    }
}

impl Default for FppaDeviceConfig {
    /// Creates a default configuration with a five-second timeout
    /// and no device label.
    fn default() -> Self {
        Self {
            initialization_timeout: Duration::from_secs(5),
            device_label: None,
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new_accepts_valid_timeout_and_label() {
        let config = FppaDeviceConfig::new(
            Duration::from_secs(10),
            Some("FPPA device".to_string()),
        )
        .unwrap();

        assert_eq!(
            config.initialization_timeout(),
            Duration::from_secs(10)
        );
        assert_eq!(config.device_label(), Some("FPPA device"));
    }

    #[test]
    fn config_new_accepts_missing_label() {
        let config = FppaDeviceConfig::new(
            Duration::from_secs(10),
            None,
        )
        .unwrap();

        assert_eq!(config.device_label(), None);
    }

    #[test]
    fn config_new_rejects_zero_timeout() {
        let result = FppaDeviceConfig::new(
            Duration::ZERO,
            None,
        );

        assert!(matches!(
            result,
            Err(FppaError::InvalidInput(_))
        ));
    }

    #[test]
    fn default_config_has_five_second_timeout() {
        let config = FppaDeviceConfig::default();

        assert_eq!(
            config.initialization_timeout(),
            Duration::from_secs(5)
        );
        assert_eq!(config.device_label(), None);
    }
}

