#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

fn main() {
    // prepare() checks if it's a single instance and tries to send the args otherwise.
    // It should always be the first line in your main function (with the exception of loggers or similar)
    tauri_plugin_deep_link::prepare("de.fabianlars.deep-link-test");
    // It's expected to use the identifier from tauri.conf.json
    // Unfortuenetly getting it is pretty ugly without access to sth that implements `Manager`.

    tauri::Builder::default()
    .setup(|app| {
      // If you need macOS support this must be called in .setup() !
      // Otherwise this could be called right after prepare() but then you don't have access to tauri APIs
      let handle = app.handle();
      tauri_plugin_deep_link::register(
        "my-scheme",
        move |request| {
          dbg!(&request);
          handle.emit_all("scheme-request-received", request).unwrap();
        },
      )
      .unwrap(/* If listening to the scheme is optional for your app, you don't want to unwrap here. */);
      Ok(())
    })
    // .plugin(tauri_plugin_deep_link::init()) // consider adding a js api later
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
