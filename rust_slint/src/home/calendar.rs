use chrono::{Datelike, Local};

// 获取当前星期
pub fn get_current_weekday() -> String {
    let now = Local::now();
    let weekday = now.weekday();

    // 将星期转换为中文显示
    match weekday {
        chrono::Weekday::Mon => "一".to_string(),
        chrono::Weekday::Tue => "二".to_string(),
        chrono::Weekday::Wed => "三".to_string(),
        chrono::Weekday::Thu => "四".to_string(),
        chrono::Weekday::Fri => "五".to_string(),
        chrono::Weekday::Sat => "六".to_string(),
        chrono::Weekday::Sun => "日".to_string(),
    }
}
