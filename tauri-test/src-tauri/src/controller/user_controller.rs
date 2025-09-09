use crate::model::user::User;

/**
 *  后端可以接受前端传过来的任意类型，只有该类型实现了Serialize trait。
 *  对于错误处理可以整合thiserror来实现自定义错误类型，因为错误类型也需要实现Serialize trait。
 */
#[tauri::command]
pub fn get_user_list() -> Result<Vec<User>, ()>{
    let user_list = vec![
        User::new(1, "张三".to_string(), 18, "中国".to_string()),
        User::new(2, "李四".to_string(), 19, "中国".to_string())
    ];
    Ok(user_list)
}

/**
 * 异步命令使用 async_runtime::spawn 在单独的异步任务上执行。 除非使用 #[tauri::command(async)] 定义，否则没有 async 关键字的命令将在主线程上执行。
 * 要么使用#[tauri::command(async)]来定义异步命令
 * 要么使用tokio带async来修饰方法实现异步
 */
#[tauri::command(async)]
pub async fn get_user(_id: i32) -> Result<User, ()> {
    let user = User::new(1, "张三".to_string(), 18, "中国".to_string());
    Ok(user)
}


#[tauri::command]
pub fn save_user(user: User, webview_window: tauri::WebviewWindow) -> Result<(), ()> {
    println!("{:?}", user);
    println!("WebviewWindow: {}", webview_window.label());
    Ok(())
}