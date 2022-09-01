#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod menu;
mod tray;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let context = tauri::generate_context!();

    tauri::Builder::default()
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler) // ✅ 注册系统托盘事件处理程序
        .menu(menu::init(&context)) // ✅ 将菜单添加到所有窗口
        .on_menu_event(menu::handler)
        .invoke_handler(tauri::generate_handler![greet])
        .run(context)
        .expect("error while running tauri application");
}
