use std::{
    fs::{create_dir_all, remove_file, File},
    io::{Error, ErrorKind, Read, Result, Write},
    os::unix::net::{UnixListener, UnixStream},
    process::Command,
};

use dirs::data_dir;

use crate::ID;

pub fn register<F: FnMut(String) + Send + 'static>(scheme: &str, handler: F) -> Result<()> {
    listen(handler)?;

    let mut target = data_dir()
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "data directory not found."))?
        .join("applications");

    create_dir_all(&target)?;

    let exe = tauri_utils::platform::current_exe()?;

    let file_name = format!(
        "{}-handler.desktop",
        exe.file_name()
            .ok_or_else(|| Error::new(
                ErrorKind::NotFound,
                "Couldn't get file name of curent executable.",
            ))?
            .to_string_lossy()
    );

    target.push(&file_name);

    let mime_types = format!("x-scheme-handler/{};", scheme);

    let mut file = File::create(&target)?;
    file.write_all(
        format!(
            include_str!("template.desktop"),
            name = ID
                .get()
                .expect("Called register() before prepare()")
                .split('.')
                .last()
                .unwrap(),
            exec = std::env::var("APPIMAGE").unwrap_or_else(|_| exe.display().to_string()),
            mime_types = mime_types
        )
        .as_bytes(),
    )?;

    target.pop();

    Command::new("update-desktop-database")
        .arg(target)
        .status()?;

    Command::new("xdg-mime")
        .args(["default", &file_name, scheme])
        .status()?;

    Ok(())
}

pub fn unregister(_scheme: &str) -> Result<()> {
    let mut target =
        data_dir().ok_or_else(|| Error::new(ErrorKind::NotFound, "data directory not found."))?;

    target.push("applications");
    target.push(format!(
        "{}-handler.desktop",
        tauri_utils::platform::current_exe()?
            .file_name()
            .ok_or_else(|| Error::new(
                ErrorKind::NotFound,
                "Couldn't get file name of curent executable.",
            ))?
            .to_string_lossy()
    ));

    remove_file(&target)?;

    Ok(())
}

pub fn listen<F: FnMut(String) + Send + 'static>(mut handler: F) -> Result<()> {
    std::thread::spawn(move || {
        let addr = format!(
            "/tmp/{}-deep-link.sock",
            ID.get().expect("listen() called before prepare()")
        );

        let listener = UnixListener::bind(addr).expect("Can't create listener");

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buffer = String::new();
                    if let Err(io_err) = stream.read_to_string(&mut buffer) {
                        log::error!("Error reading incoming connection: {}", io_err.to_string());
                    };

                    handler(dbg!(buffer));
                }
                Err(err) => {
                    log::error!("Incoming connection failed: {}", err);
                    continue;
                }
            }
        }
    });

    Ok(())
}

pub fn prepare(identifier: &str) {
    let addr = format!("/tmp/{}-deep-link.sock", identifier);

    match UnixStream::connect(&addr) {
        Ok(mut stream) => {
            if let Err(io_err) =
                stream.write_all(std::env::args().nth(1).unwrap_or_default().as_bytes())
            {
                log::error!(
                    "Error sending message to primary instance: {}",
                    io_err.to_string()
                );
            };
            std::process::exit(0);
        }
        Err(err) => {
            log::error!("Error creating socket listener: {}", err.to_string());
            if err.kind() == ErrorKind::ConnectionRefused {
                let _ = remove_file(&addr);
            }
        }
    };
    ID.set(identifier.to_string())
        .expect("prepare() called more than once with different identifiers.");
}
