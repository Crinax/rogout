use std::{os::unix::process::CommandExt, process::Command};

#[tauri::command]
pub fn system_shutdown() {
    Command::new("poweroff").exec();
}

#[tauri::command]
pub fn system_restart() {
    Command::new("reboot").exec();
}

#[tauri::command]
pub fn system_suspend() -> bool {
    let output = Command::new("systemctl").arg("suspend").output();

    return output.is_ok();
}
