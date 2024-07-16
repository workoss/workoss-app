use tauri::Manager;
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_cli::CliExt;
use tauri_plugin_fs::FsExt;
use tauri_plugin_global_shortcut::GlobalShortcutExt;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            #[cfg(desktop)]
            {
                use tauri::Emitter;
                use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};

                println!("------");
                let scope = app.fs_scope();
                scope.allow_directory("/home/workoss", false);
                dbg!(scope.allowed());

                match app.cli().matches() {
                    Ok(matches) => {
                        println!("{:?}", matches)
                    }
                    Err(_) => {}
                }

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_shortcuts(["ctrl+d", "alt+n"])?
                        .with_handler(|app, shortcut, event| {
                            println!("==========");
                            if event.state == ShortcutState::Pressed {
                                if shortcut.matches(Modifiers::CONTROL, Code::KeyD) {
                                    let _ = app.emit("shortcut-event", "Ctrl+D triggered");
                                }
                                if shortcut.matches(Modifiers::ALT, Code::KeyN) {
                                    let _ = app.emit("shortcut-event", "Alt+N triggered");
                                }
                            }
                        })
                        .build(),
                )?;
            }

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
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
