pub(crate) mod model;
pub(crate) mod controller;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // 配置处理器，全局就只能配置一个，因为内部是一个赋值语句，而不是append，所以当有多个的时候就会使用最后一个配置
        .invoke_handler(generate_hander())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn generate_hander() -> impl Fn(tauri::ipc::Invoke<tauri::Wry>) -> bool + Send + Sync + 'static {
    tauri::generate_handler![
        greet, 
        controller::user_controller::get_user_list, 
        controller::user_controller::get_user,
        controller::user_controller::save_user
    ]
}