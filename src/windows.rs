use std::{
    io::{BufRead, BufReader, Write},
    path::Path,
};

use interprocess::local_socket::{LocalSocketListener, LocalSocketStream};
use winreg::{enums::HKEY_CURRENT_USER, RegKey};

use crate::ID;

// Consider adding a function to register without starting the listener.
// Plugin needs linux and macOS support before making decisions.

pub fn register<F: FnMut(String) + Send + 'static>(
    scheme: &str,
    handler: F,
) -> Result<(), std::io::Error> {
    listen(handler)?;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let base = Path::new("Software").join("Classes").join(scheme);

    let exe = tauri_utils::platform::current_exe()?
        .to_string_lossy()
        .replace("\\\\?\\", "");

    let (key, _) = hkcu.create_subkey(&base)?;
    key.set_value(
        "",
        &format!(
            "URL:{}",
            ID.get().expect("register() called before prepare()")
        ),
    )?;
    key.set_value("URL Protocol", &"")?;

    let (icon, _) = hkcu.create_subkey(base.join("DefaultIcon"))?;
    icon.set_value("", &format!("{},0", &exe))?;

    let (cmd, _) = hkcu.create_subkey(base.join("shell").join("open").join("command"))?;

    cmd.set_value("", &format!("{} \"%1\"", &exe))?;

    Ok(())
}

pub fn unregister(scheme: &str) -> Result<(), std::io::Error> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let base = Path::new("Software").join("Classes").join(scheme);

    hkcu.delete_subkey_all(base)?;

    Ok(())
}

pub fn listen<F: FnMut(String) + Send + 'static>(mut handler: F) -> Result<()> {
    std::thread::spawn(move || {
        let listener =
            LocalSocketListener::bind(ID.get().expect("listen() called before prepare()").as_str())
                .expect("Can't create listener");

        for conn in listener.incoming().filter_map(|c| {
            c.map_err(|error| log::error!("Incoming connection failed: {}", error))
                .ok()
        }) {
            let mut conn = BufReader::new(conn);
            let mut buffer = String::new();
            if let Err(io_err) = conn.read_line(&mut buffer) {
                log::error!("Error reading incoming connection: {}", io_err.to_string());
            };
            buffer.pop();

            handler(buffer);
        }
    });

    Ok(())
}

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
