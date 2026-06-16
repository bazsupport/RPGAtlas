/* RPGAtlas desktop wrapper — native commands.
   GPL-3.0-or-later (see ../LICENSE).

   The editor is the existing static web app, embedded as the frontend. These
   commands give it the few things a browser tab cannot do well: native file
   dialogs for project save/load, and a dedicated window for play-testing. */

use tauri::{Manager, WindowEvent};
use tauri_plugin_dialog::DialogExt;

/// Save the editor's project JSON to a user-chosen file. Returns the chosen
/// path, or `None` if the user cancelled the dialog.
#[tauri::command]
fn save_project(
    app: tauri::AppHandle,
    json: String,
    suggested: String,
) -> Result<Option<String>, String> {
    let picked = app
        .dialog()
        .file()
        .add_filter("RPGAtlas project", &["json"])
        .set_file_name(format!("{suggested}.json"))
        .blocking_save_file();

    match picked {
        Some(file) => {
            let path = file.into_path().map_err(|e| e.to_string())?;
            std::fs::write(&path, json).map_err(|e| e.to_string())?;
            Ok(Some(path.to_string_lossy().into_owned()))
        }
        None => Ok(None),
    }
}

/// Write the project JSON straight to a known path (no dialog). Used by the
/// Save button once the project is bound to a file. The path originates from a
/// prior Save dialog, so it is already user-authorized.
#[tauri::command]
fn save_project_to_path(path: String, json: String) -> Result<(), String> {
    std::fs::write(&path, json).map_err(|e| e.to_string())
}

/// Open a project file chosen by the user and return its contents. Returns
/// `None` if the user cancelled.
#[tauri::command]
fn open_project(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let picked = app
        .dialog()
        .file()
        .add_filter("RPGAtlas project", &["json"])
        .blocking_pick_file();

    match picked {
        Some(file) => {
            let path = file.into_path().map_err(|e| e.to_string())?;
            let contents = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
            Ok(Some(contents))
        }
        None => Ok(None),
    }
}

/// Open (or focus) the play-test window, pointed at the bundled play.html.
/// localStorage is shared across windows of the same origin, so the player
/// reads the project the editor just autosaved.
#[tauri::command]
fn open_playtest(app: tauri::AppHandle) -> Result<(), String> {
    // The play-test window is declared in tauri.conf.json and created at startup
    // (hidden). Building a window on demand from inside a command instead causes
    // a blank/frozen webview, so we reuse the pre-built one: reload it to re-read
    // the project the editor just autosaved, then show and focus it. Closing it
    // only hides it (see the window-event handler in `run`), so it is always
    // here to reuse, no matter how many times the user plays and closes.
    let playtest = app
        .get_webview_window("playtest")
        .ok_or_else(|| "Play-test window was not initialized.".to_string())?;

    playtest.reload().map_err(|e| e.to_string())?;
    playtest.show().map_err(|e| e.to_string())?;
    playtest.set_focus().map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .on_window_event(|window, event| {
            // Hide the play-test window on close rather than destroying it, so it
            // can be reused for every subsequent play-test. Destroying it would
            // free its "playtest" label and leave nothing for open_playtest to
            // reopen. The main window keeps the default behavior (quits the app).
            if window.label() == "playtest" {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            save_project,
            save_project_to_path,
            open_project,
            open_playtest
        ])
        .run(tauri::generate_context!())
        .expect("error while running RPGAtlas");
}
