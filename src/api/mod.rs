use gloo_net::http::Request;
use serde_json::json;
use web_sys::window;

use crate::types::{CalendarEvent, DatabaseConfig, NotionDatabase, NotionListResponse, NotionPage, extract_event_from_page};

const NOTION_VERSION: &str = "2022-06-28";

/// 获取 Notion API 基础 URL
/// 生产环境使用 Vercel Edge Function 代理解决 CORS
fn get_notion_api_base() -> String {
    // 检测是否在 Vercel 环境
    if let Ok(hostname) = get_hostname() {
        if hostname.contains("vercel.app") || hostname.contains("now.sh") {
            // 使用相对路径调用 Vercel Edge Function
            return "/api/notion".to_string();
        }
    }

    // 默认使用直接 API（本地开发）
    "https://api.notion.com/v1".to_string()
}

/// 获取当前主机名
fn get_hostname() -> Result<String, String> {
    if let Some(window) = window() {
        if let Ok(host) = window.location().host() {
            return Ok(host);
        }
    }
    Err("无法获取主机名".to_string())
}

/// Notion API 客户端
pub struct NotionClient {
    api_key: String,
    base_url: String,
}

impl NotionClient {
    pub fn new(api_key: String) -> Self {
        let base_url = get_notion_api_base();
        Self { api_key, base_url }
    }

    /// 手动设置代理 URL
    pub fn with_proxy(mut self, proxy_url: String) -> Self {
        self.base_url = proxy_url;
        self
    }

    /// 获取所有可访问的数据库
    pub async fn list_databases(&self) -> Result<Vec<NotionDatabase>, String> {
        let url = format!("{}/databases", self.base_url);

        let response = Request::get(&url)
            .header("Authorization", &format!("Bearer {}", self.api_key))
            .header("Notion-Version", NOTION_VERSION)
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| format!("请求失败: {:?}", e))?;

        if !response.ok() {
            return Err(format!("API 错误: {}", response.status()));
        }

        let data: NotionListResponse<NotionDatabase> = response
            .json()
            .await
            .map_err(|e| format!("解析失败: {:?}", e))?;

        Ok(data.results)
    }

    /// 获取单个数据库详情
    pub async fn get_database(&self, database_id: &str) -> Result<NotionDatabase, String> {
        let url = format!("{}/databases/{}", self.base_url, database_id);

        let response = Request::get(&url)
            .header("Authorization", &format!("Bearer {}", self.api_key))
            .header("Notion-Version", NOTION_VERSION)
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| format!("请求失败: {:?}", e))?;

        if !response.ok() {
            return Err(format!("API 错误: {}", response.status()));
        }

        response
            .json()
            .await
            .map_err(|e| format!("解析失败: {:?}", e))
    }

    /// 查询数据库中的页面
    pub async fn query_database(&self, database_id: &str) -> Result<Vec<NotionPage>, String> {
        let url = format!("{}/databases/{}/query", self.base_url, database_id);

        let body = json!({
            "page_size": 100
        });

        let response = Request::post(&url)
            .header("Authorization", &format!("Bearer {}", self.api_key))
            .header("Notion-Version", NOTION_VERSION)
            .header("Content-Type", "application/json")
            .json(&body)
            .map_err(|e| format!("构建请求失败: {:?}", e))?
            .send()
            .await
            .map_err(|e| format!("请求失败: {:?}", e))?;

        if !response.ok() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("API 错误 ({}): {}", status, text));
        }

        let data: NotionListResponse<NotionPage> = response
            .json()
            .await
            .map_err(|e| format!("解析失败: {:?}", e))?;

        Ok(data.results)
    }

    /// 创建新页面（事件）
    pub async fn create_page(
        &self,
        database_id: &str,
        title: &str,
        date: &str,
    ) -> Result<NotionPage, String> {
        let url = format!("{}/pages", self.base_url);

        let body = json!({
            "parent": {
                "database_id": database_id
            },
            "properties": {
                "Name": {
                    "title": [
                        {
                            "text": {
                                "content": title
                            }
                        }
                    ]
                },
                "Date": {
                    "date": {
                        "start": date
                    }
                }
            }
        });

        let response = Request::post(&url)
            .header("Authorization", &format!("Bearer {}", self.api_key))
            .header("Notion-Version", NOTION_VERSION)
            .header("Content-Type", "application/json")
            .json(&body)
            .map_err(|e| format!("构建请求失败: {:?}", e))?
            .send()
            .await
            .map_err(|e| format!("请求失败: {:?}", e))?;

        if !response.ok() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("API 错误 ({}): {}", status, text));
        }

        response
            .json()
            .await
            .map_err(|e| format!("解析失败: {:?}", e))
    }

    /// 更新页面
    pub async fn update_page(
        &self,
        page_id: &str,
        title: Option<&str>,
        date: Option<&str>,
    ) -> Result<NotionPage, String> {
        let url = format!("{}/pages/{}", self.base_url, page_id);

        let mut properties = serde_json::Map::new();

        if let Some(t) = title {
            properties.insert(
                "Name".to_string(),
                json!({
                    "title": [{ "text": { "content": t } }]
                }),
            );
        }

        if let Some(d) = date {
            properties.insert(
                "Date".to_string(),
                json!({
                    "date": { "start": d }
                }),
            );
        }

        let body = json!({ "properties": properties });

        let response = Request::patch(&url)
            .header("Authorization", &format!("Bearer {}", self.api_key))
            .header("Notion-Version", NOTION_VERSION)
            .header("Content-Type", "application/json")
            .json(&body)
            .map_err(|e| format!("构建请求失败: {:?}", e))?
            .send()
            .await
            .map_err(|e| format!("请求失败: {:?}", e))?;

        if !response.ok() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("API 错误 ({}): {}", status, text));
        }

        response
            .json()
            .await
            .map_err(|e| format!("解析失败: {:?}", e))
    }

    /// 删除页面（实际是将页面归档）
    pub async fn delete_page(&self, page_id: &str) -> Result<(), String> {
        let url = format!("{}/pages/{}", self.base_url, page_id);

        let body = json!({
            "archived": true
        });

        let response = Request::patch(&url)
            .header("Authorization", &format!("Bearer {}", self.api_key))
            .header("Notion-Version", NOTION_VERSION)
            .header("Content-Type", "application/json")
            .json(&body)
            .map_err(|e| format!("构建请求失败: {:?}", e))?
            .send()
            .await
            .map_err(|e| format!("请求失败: {:?}", e))?;

        if !response.ok() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("API 错误 ({}): {}", status, text));
        }

        Ok(())
    }
}

/// 加载指定数据库的所有事件
pub async fn load_events_from_database(
    client: &NotionClient,
    db_config: &DatabaseConfig,
) -> Result<Vec<CalendarEvent>, String> {
    let pages = client.query_database(&db_config.notion_database_id).await?;

    let events: Vec<CalendarEvent> = pages
        .iter()
        .filter_map(|page| extract_event_from_page(page, db_config))
        .collect();

    Ok(events)
}
