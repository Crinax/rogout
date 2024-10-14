use std::process::Command;

#[tauri::command]
pub fn system_shutdown() {
  println!("Shutting down...");
}

#[tauri::command]
pub fn system_restart() {
  println!("Restarting...");
}

#[tauri::command]
pub fn system_suspend() -> bool {
  let output = Command::new("systemctl")
    .arg("suspend")
    .output();

  return output.is_ok();
}