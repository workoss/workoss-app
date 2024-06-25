use tauri::Manager;
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_cli::CliExt;
use tauri_plugin_fs::FsExt;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            println!("im here11"); // not here

            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{
                    Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
                };

                let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyN);
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        // .with_shortcuts(["ctrl+n", "alt+space"])?
                        .with_handler(|app, shortcut, event| {
                            println!("im here"); // not here
                            if event.state == ShortcutState::Pressed {
                                if shortcut.matches(Modifiers::CONTROL, Code::KeyN) {
                                    println!("Ctrl+N triggered");
                                    let _ = app.emit("shortcut-event", "Ctrl+N triggered");
                                }
                                if shortcut.matches(Modifiers::ALT, Code::Space) {
                                    println!("Alt+Space triggered");
                                    let _ = app.emit("shortcut-event", "Alt+Space triggered");
                                }
                            }
                        })
                        .build(),
                )?;

                app.global_shortcut().register(ctrl_n_shortcut)?;
            }
            Ok(())
        })
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let scope = app.fs_scope();
            scope.allow_directory("/home/workoss", false);
            dbg!(scope.allowed());
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_cli::init())
        .setup(|app| {
            match app.cli().matches() {
                Ok(matches) => {
                    println!("{:?}", matches)
                }
                Err(_) => {}
            }
            Ok(())
        })
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .setup(|app| {
            let autostart_manager = app.autolaunch();
            //enable autostart
            let _ = autostart_manager.enable();
            println!(
                "registered for autostart? {}",
                autostart_manager.is_enabled().unwrap()
            );
            //disable autostart
            let _ = autostart_manager.disable();
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
