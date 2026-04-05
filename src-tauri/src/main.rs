#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        // 自动更新插件
        .plugin(tauri_plugin_updater::Builder::new().build())
        // 🔴 必须注册 process 插件，否则 relaunch 无法使用
        .plugin(tauri_plugin_process::init())
        .run(tauri::generate_context!())
        .expect("运行应用失败");
}