// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use tauri::GlobalShortcutManager;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let mut shortcut_manager = app.global_shortcut_manager();
            shortcut_manager.register("Command+Option+Shift+3", || {
                // TODO: read from clipboard
                // would be nice if you could force a 'copy' when the shortcut is triggered
                // use case is: copy something you don't get, trigger global shortcut
                // it will take selected text, then do:
                // 1. pre prompt it with `explain <text>`
                // 2. send the prompt to GPT-3/4
                // 3. wait for response
                // 4. response is done, send notification (maybe use https://github.com/hoodie/notify-rust)
                // 5. force update on the front end
                // 6. each answer is separate tab

                // also if possible each tab is it's own chat, so you can further talk with ChatGPT to expand on answer
                // also the big idea behind the app is that you can fire off multiple requests at once
                // if you do it with gpt-4 and it's busy, send the request to gpt-3
                // gpt-3/4 can be configurable to any model you want
                // println!("shortcut trigger");
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
