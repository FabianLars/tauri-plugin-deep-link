use std::path::Path;

use winreg::{enums::HKEY_CURRENT_USER, RegKey};

use crate::shared::listen;

pub fn register<F: FnMut(String) + Send + 'static>(
    identifier: &str,
    scheme: &str,
    handler: F,
) -> Result<(), std::io::Error> {
    listen(identifier.to_string(), handler);

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let base = Path::new("Software").join("Classes").join(scheme);

    let exe = tauri::utils::platform::current_exe()?
        .to_string_lossy()
        .replace("\\\\?\\", "");

    let (key, _) = hkcu.create_subkey(&base)?;
    key.set_value("", &identifier)?;
    key.set_value("URL Protocol", &"")?;

    let (icon, _) = hkcu.create_subkey(base.join("DefaultIcon"))?;
    icon.set_value("", &format!("{},0", &exe))?;

    let (cmd, _) = hkcu.create_subkey(base.join("shell").join("open").join("command"))?;

    cmd.set_value("", &format!("{} \"%1\"", &exe))?;

    println!("register end");

    Ok(())
}

pub fn unregister(scheme: &str) -> Result<(), std::io::Error> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let base = Path::new("Software").join("Classes").join(scheme);

    hkcu.delete_subkey_all(base)?;

    Ok(())
}
