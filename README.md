# "Deep link" plugin for Tauri

[![](https://img.shields.io/crates/v/tauri-plugin-deep-link.svg)](https://crates.io/crates/tauri-plugin-deep-link) [![](https://img.shields.io/docsrs/tauri-plugin-deep-link)](https://docs.rs/tauri-plugin-deep-link)

Temporary solution until https://github.com/tauri-apps/tauri/issues/323 lands.

Depending on your use case, for example a `Login with Google` button, you may be looking for https://github.com/FabianLars/tauri-plugin-oauth instead which uses a localhost server for the OAuth process instead of custom uri schemes (because it's required by some oauth providers like the aforementioned Google).

Check out the [`example/`](https://github.com/FabianLars/tauri-plugin-deep-link/tree/main/example) dir for a minimal example. You must copy it into an actual tauri app first!

## macOS

In case you're one of the very few people that didn't know this already: macOS hates developers! Not only is that why the macOS implementation took me so long, it also means _you_ have to be a bit more careful if your app targets macOS:

-   Read through the methods' platform-specific notes.
-   On macOS you need to register the schemes in a `Info.plist` file at build time, the plugin can't change the schemes at runtime.
-   macOS apps are in single-instance by default so this plugin will not manually exit secondary instances in release mode.
    -   To make `tauri dev` a little bit more pleasant to work with, the plugin will work similar-ish to Linux and Windows _in debug mode_ but you will see secondary instances show on the screen for a split second and the event will trigger twice in the primary instance (one of these events will be an empty string). You still have to install a `.app` bundle you got from `tauri build --debug` for this to work!
