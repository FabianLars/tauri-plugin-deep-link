use std::io::Write;

use interprocess::local_socket::LocalSocketStream;
use once_cell::sync::OnceCell;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;

mod shared;

static ID: OnceCell<String> = OnceCell::new();

pub fn prepare(identifier: &str) {
    if let Ok(mut conn) = LocalSocketStream::connect(identifier) {
        if let Err(io_err) = conn.write_all(std::env::args().nth(1).unwrap_or_default().as_bytes())
        {
            log::error!(
                "Error sending message to primary instance: {}",
                io_err.to_string()
            );
        };
        let _ = conn.write_all(b"\n");
        std::process::exit(0);
    };
    ID.set(identifier.to_string())
        .expect("prepare() called more than once with different identifiers.");
}

/// This function is meant for use-cases where the default [`prepare`] function can't be used.
///
/// # Errors
/// If ID was already set this functions returns an error containing the ID as String.
pub fn set_identifier(identifier: &str) -> Result<(), String> {
    ID.set(identifier.to_string())
}
