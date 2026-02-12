use crate::types::AppState;
use gloo_storage::{LocalStorage, Storage};

const STORAGE_KEY: &str = "notion-cafe-state";

/// 保存应用状态到本地存储
pub fn save_state(state: &AppState) -> Result<(), String> {
    let json = serde_json::to_string(state).map_err(|e| format!("序列化失败: {}", e))?;
    LocalStorage::set(STORAGE_KEY, json).map_err(|e| format!("存储失败: {:?}", e))
}

/// 从本地存储加载应用状态
pub fn load_state() -> Option<AppState> {
    let json: String = LocalStorage::get(STORAGE_KEY).ok()?;
    serde_json::from_str(&json).ok()
}

/// 清除本地存储
pub fn clear_state() -> Result<(), String> {
    LocalStorage::delete(STORAGE_KEY);
    Ok(())
}

/// 格式化日期为显示文本
pub fn format_date(date: &chrono::NaiveDate) -> String {
    date.format("%Y年%m月%d日").to_string()
}

/// 格式化日期为简洁显示
pub fn format_date_short(date: &chrono::NaiveDate) -> String {
    date.format("%m/%d").to_string()
}

/// 获取月份名称
pub fn month_name(month: u32) -> &'static str {
    match month {
        1 => "一月",
        2 => "二月",
        3 => "三月",
        4 => "四月",
        5 => "五月",
        6 => "六月",
        7 => "七月",
        8 => "八月",
        9 => "九月",
        10 => "十月",
        11 => "十一月",
        12 => "十二月",
        _ => "未知",
    }
}

/// 获取星期名称
pub fn weekday_name(weekday: chrono::Weekday) -> &'static str {
    match weekday {
        chrono::Weekday::Mon => "周一",
        chrono::Weekday::Tue => "周二",
        chrono::Weekday::Wed => "周三",
        chrono::Weekday::Thu => "周四",
        chrono::Weekday::Fri => "周五",
        chrono::Weekday::Sat => "周六",
        chrono::Weekday::Sun => "周日",
    }
}

/// 生成颜色变体（用于不同数据库）
pub fn generate_color(index: usize) -> String {
    let colors = vec![
        "#667eea", // 紫色
        "#f093fb", // 粉色
        "#4facfe", // 蓝色
        "#43e97b", // 绿色
        "#fa709a", // 珊瑚色
        "#feca57", // 黄色
        "#48dbfb", // 天蓝
        "#ff9ff3", // 浅粉
        "#54a0ff", // 亮蓝
        "#5f27cd", // 深紫
    ];
    colors[index % colors.len()].to_string()
}

/// 验证 Notion API Key 格式
pub fn is_valid_notion_key(key: &str) -> bool {
    key.starts_with("secret_") && key.len() > 20
}

/// 验证 Notion Database ID 格式
pub fn is_valid_database_id(id: &str) -> bool {
    id.len() == 32 && id.chars().all(|c| c.is_ascii_hexdigit())
}

/// 清理 database ID（移除连字符，转为小写）
pub fn clean_database_id(id: &str) -> String {
    id.replace("-", "").to_lowercase()
}
