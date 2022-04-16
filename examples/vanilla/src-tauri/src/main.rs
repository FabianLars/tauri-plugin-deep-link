#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  // prepare() checks if it's a single instance and sends args otherwise.
  tauri_plugin_deep_link::prepare("de.FabianLars.deep-link-test").unwrap();
  // It's expected to use the identifier from tauri.conf.json
  // Unfortuenetly getting it is pretty ugly without access to sth that implements `Manager`.

  tauri::Builder::default()
    .setup(|app| {
      // This could be called right after prepare() but then you don't have access to tauri APIs
      let handle = app.handle();
      tauri_plugin_deep_link::register(
        "de.FabianLars.deep-link-test",
        "temp-scheme",
        move |request| {
          dbg!(&request);
          handle.emit_all("scheme", request).unwrap();
        },
      )
      .unwrap(/* If listening to the scheme is optional for your app, you don't want to unwrap here. */);
      Ok(())
    })
    // .plugin(tauri_plugin_deep_link::init()) // consider adding a js api later
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
