#[cfg(target_os = "windows")]
mod handlers;
mod providers;
#[cfg(target_os = "windows")]
mod stubs;

use crate::platforms::WifiError;
use std::{fmt, io};

/// Wireless network connectivity functionality.
pub trait Connectivity: fmt::Debug {
    /// Makes an attempt to connect to a selected wireless network with password specified.
    fn connect(&mut self, ssid: &str, password: &str) -> Result<bool, WifiConnectionError>;

    /// Disconnects from a wireless network currently connected to.
    fn disconnect(&self) -> Result<bool, WifiConnectionError>;

    /// Performs a wireless scan
    fn scan(&self) -> Result<bool, WifiConnectionError>;
}

/// Error that occurs when attempting to connect to a wireless network.
#[derive(Debug)]
pub enum WifiConnectionError {
    /// Adding the newtork profile failed.
    #[cfg(target_os = "windows")]
    AddNetworkProfileFailed,
    /// Failed to connect to wireless network.
    FailedToConnect(String),
    /// Failed to disconnect from wireless network. Try turning the wireless interface down.
    FailedToDisconnect(String),
    /// Scan failed
    FailedScan(String),
    /// A wireless error occurred.
    Other { kind: WifiError },
    // SsidNotFound,
}

impl From<io::Error> for WifiConnectionError {
    fn from(error: io::Error) -> Self {
        WifiConnectionError::Other {
            kind: WifiError::IoError(error),
        }
    }
}
