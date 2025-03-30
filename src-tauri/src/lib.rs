// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod db;

use crate::db::*;
use tauri::{Emitter, Listener, Manager, WindowEvent};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if let Err(e) = init() {
        eprintln!("数据库操作失败: {:?}", e);
    }
    tauri::Builder::default()
        .on_window_event(|window, event| {
            // 监听保存完成事件
            let window_clone = window.clone();
            window.app_handle().once("config-saved", move|_| {
                // 执行关闭窗口
                window_clone.destroy().unwrap();
            });
            match event {
                WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();
                    // 调用前端保存函数
                    window.app_handle().emit("save-config", ()).unwrap();
                }
                _ => {}
            }
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_config,
            update_config,
            get_clients,
            add_client,
            update_client,
            remove_client,
            get_todo,
            add_todo,
            update_todo,
            remove_todo,
            remove_finished_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}