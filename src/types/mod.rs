use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Notion 数据库配置
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub id: String,
    pub name: String,
    pub notion_database_id: String,
    pub date_property: String,
    pub title_property: String,
    pub color: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "我的日历".to_string(),
            notion_database_id: String::new(),
            date_property: "Date".to_string(),
            title_property: "Name".to_string(),
            color: "#667eea".to_string(),
        }
    }
}

/// 日历事件
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub id: String,
    pub title: String,
    pub database_id: String,
    pub notion_page_id: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub all_day: bool,
    pub description: Option<String>,
    pub color: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 日历视图模式
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewMode {
    Month,
    Week,
    Day,
}

impl ViewMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ViewMode::Month => "月",
            ViewMode::Week => "周",
            ViewMode::Day => "日",
        }
    }
}

/// 应用状态
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppState {
    pub notion_api_key: Option<String>,
    pub databases: Vec<DatabaseConfig>,
    pub current_view: ViewMode,
    pub current_date: NaiveDate,
    pub selected_database_ids: Vec<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            notion_api_key: None,
            databases: vec![DatabaseConfig::default()],
            current_view: ViewMode::Month,
            current_date: chrono::Local::now().naive_local().date(),
            selected_database_ids: vec![],
        }
    }
}

/// Notion API 响应类型
#[derive(Clone, Debug, Deserialize)]
pub struct NotionDatabase {
    pub id: String,
    pub title: Vec<NotionText>,
    pub properties: serde_json::Value,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NotionPage {
    pub id: String,
    pub properties: serde_json::Value,
    pub created_time: String,
    pub last_edited_time: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NotionText {
    pub plain_text: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NotionListResponse<T> {
    pub results: Vec<T>,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

/// 从 Notion 页面提取事件
pub fn extract_event_from_page(
    page: &NotionPage,
    db_config: &DatabaseConfig,
) -> Option<CalendarEvent> {
    let props = &page.properties;

    let title = props
        .get(&db_config.title_property)
        .and_then(|p| p.get("title"))
        .and_then(|t| t.as_array())
        .and_then(|arr| arr.first())
        .and_then(|t| t.get("plain_text"))
        .and_then(|t| t.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "无标题".to_string());

    let date_value = props.get(&db_config.date_property)?;
    let date_range = date_value.get("date")?;

    let start_str = date_range.get("start")?.as_str()?;
    let start_date = if start_str.len() == 10 {
        NaiveDate::parse_from_str(start_str, "%Y-%m-%d").ok()?
    } else {
        DateTime::parse_from_rfc3339(start_str)
            .ok()
            .map(|dt| dt.naive_local().date())?
    };

    let end_date = date_range
        .get("end")
        .and_then(|e| e.as_str())
        .and_then(|s| {
            if s.len() == 10 {
                NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()
            } else {
                DateTime::parse_from_rfc3339(s)
                    .ok()
                    .map(|dt| dt.naive_local().date())
            }
        });

    Some(CalendarEvent {
        id: Uuid::new_v4().to_string(),
        title,
        database_id: db_config.id.clone(),
        notion_page_id: Some(page.id.clone()),
        start_date,
        end_date,
        all_day: true,
        description: None,
        color: db_config.color.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    })
}
