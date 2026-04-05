// 这是Tauri 2.x 标准入口，已启用自动更新
#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

fn main() {
  tauri::Builder::default()
    // 核心：注册自动更新插件
    .plugin(tauri_plugin_updater::init())
    .run(tauri::generate_context!())
    .expect("运行应用失败");
}