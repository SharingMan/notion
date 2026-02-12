use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::types::{DatabaseConfig, AppState};
use crate::utils::{is_valid_notion_key, is_valid_database_id, clean_database_id, generate_color};

#[derive(Properties, Clone, PartialEq)]
pub struct SettingsPanelProps {
    pub state: AppState,
    pub on_state_change: Callback<AppState>,
    pub on_close: Callback<()>,
}

#[function_component(SettingsPanel)]
pub fn settings_panel(props: &SettingsPanelProps) -> Html {
    let state = use_state(|| props.state.clone());
    let error_message = use_state(|| None::<String>);
    let success_message = use_state(|| None::<String>);

    // 更新 API Key
    let on_api_key_change = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_state = (*state).clone();
            let value = input.value();
            new_state.notion_api_key = if value.is_empty() { None } else { Some(value) };
            state.set(new_state);
        })
    };

    // 添加新数据库
    let on_add_database = {
        let state = state.clone();
        let error_message = error_message.clone();
        Callback::from(move |_| {
            error_message.set(None);
            let mut new_state = (*state).clone();
            let new_db = DatabaseConfig {
                id: uuid::Uuid::new_v4().to_string(),
                name: format!("数据库 {}", new_state.databases.len() + 1),
                color: generate_color(new_state.databases.len()),
                ..Default::default()
            };
            new_state.databases.push(new_db);
            state.set(new_state);
        })
    };

    // 删除数据库
    let on_remove_database = {
        let state = state.clone();
        Callback::from(move |db_id: String| {
            let mut new_state = (*state).clone();
            new_state.databases.retain(|d| d.id != db_id);
            state.set(new_state);
        })
    };

    // 更新数据库配置
    let on_update_database = {
        let state = state.clone();
        Callback::from(move |(index, db): (usize, DatabaseConfig)| {
            let mut new_state = (*state).clone();
            if index < new_state.databases.len() {
                new_state.databases[index] = db;
            }
            state.set(new_state);
        })
    };

    // 保存设置
    let on_save = {
        let state = state.clone();
        let error_message = error_message.clone();
        let success_message = success_message.clone();
        let on_state_change = props.on_state_change.clone();
        Callback::from(move |_: MouseEvent| {
            // 验证 API Key
            if let Some(ref key) = state.notion_api_key {
                if !is_valid_notion_key(key) {
                    error_message.set(Some("Notion API Key 格式不正确（应以 'secret_' 开头）".to_string()));
                    return;
                }
            }

            // 验证数据库配置
            for db in &state.databases {
                if !db.notion_database_id.is_empty()
                    && !is_valid_database_id(&clean_database_id(&db.notion_database_id)) {
                    error_message.set(Some(format!(
                        "数据库 '{}' 的 ID 格式不正确",
                        db.name
                    )));
                    return;
                }
            }

            // 保存到本地存储
            if let Err(e) = crate::utils::save_state(&*state) {
                error_message.set(Some(format!("保存失败: {}", e)));
                return;
            }

            success_message.set(Some("设置已保存".to_string()));
            on_state_change.emit((*state).clone());

            // 3秒后清除成功消息
            let success_message = success_message.clone();
            gloo::timers::callback::Timeout::new(3000, move || {
                success_message.set(None);
            })
            .forget();
        })
    };

    html! {
        <div class="settings-panel-overlay" onclick={props.on_close.reform(|_| ())}>
            <div class="settings-panel" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                <div class="settings-header">
                    <h2>{"⚙️ 设置"}</h2>
                    <button class="close-btn" onclick={props.on_close.reform(|_| ())}>
                        {"✕"}
                    </button>
                </div>

                <div class="settings-content">
                    // API Key 设置
                    <section class="settings-section">
                        <h3>{"Notion 集成"}</h3>
                        <div class="form-group">
                            <label>{"Notion API Key"}</label>
                            <input
                                type="password"
                                value={state.notion_api_key.clone().unwrap_or_default()}
                                onchange={on_api_key_change}
                                placeholder="secret_xxxxxxxxxxxxxxxxxxxx"
                            />
                            <span class="help-text">
                                {"在 "}
                                <a href="https://www.notion.so/my-integrations" target="_blank">
                                    {"Notion Integrations"}
                                </a>
                                {" 获取"}
                            </span>
                        </div>
                    </section>

                    // 数据库配置
                    <section class="settings-section">
                        <h3>{"日历数据库"}</h3>
                        <div class="databases-list">
                            {state.databases.iter().enumerate().map(|(i, db)| {
                                let on_update = on_update_database.clone();
                                let on_remove = on_remove_database.clone();
                                let db = db.clone();
                                html! {
                                    <DatabaseConfigCard
                                        index={i}
                                        db={db}
                                        on_update={on_update}
                                        on_remove={on_remove}
                                    />
                                }
                            }).collect::<Html>()}
                        </div>
                        <button class="add-db-btn" onclick={on_add_database}>
                            {"+ 添加数据库"}
                        </button>
                    </section>

                    // 消息提示
                    {if let Some(ref error) = *error_message {
                        html! { <div class="alert alert-error">{error}</div> }
                    } else if let Some(ref success) = *success_message {
                        html! { <div class="alert alert-success">{success}</div> }
                    } else {
                        html! {}
                    }}
                </div>

                <div class="settings-footer">
                    <button class="btn-primary" onclick={on_save}>
                        {"保存设置"}
                    </button>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct DatabaseConfigCardProps {
    index: usize,
    db: DatabaseConfig,
    on_update: Callback<(usize, DatabaseConfig)>,
    on_remove: Callback<String>,
}

#[function_component(DatabaseConfigCard)]
fn database_config_card(props: &DatabaseConfigCardProps) -> Html {
    let is_expanded = use_state(|| props.index == 0);

    let on_toggle = {
        let is_expanded = is_expanded.clone();
        Callback::from(move |_| {
            is_expanded.set(!*is_expanded);
        })
    };

    let update_name = {
        let db = props.db.clone();
        let on_update = props.on_update.clone();
        let index = props.index;
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_db = db.clone();
            new_db.name = input.value();
            on_update.emit((index, new_db));
        })
    };

    let update_db_id = {
        let db = props.db.clone();
        let on_update = props.on_update.clone();
        let index = props.index;
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_db = db.clone();
            new_db.notion_database_id = clean_database_id(&input.value());
            on_update.emit((index, new_db));
        })
    };

    let update_date_prop = {
        let db = props.db.clone();
        let on_update = props.on_update.clone();
        let index = props.index;
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_db = db.clone();
            new_db.date_property = input.value();
            on_update.emit((index, new_db));
        })
    };

    let update_title_prop = {
        let db = props.db.clone();
        let on_update = props.on_update.clone();
        let index = props.index;
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_db = db.clone();
            new_db.title_property = input.value();
            on_update.emit((index, new_db));
        })
    };

    let on_remove = {
        let on_remove = props.on_remove.clone();
        let db_id = props.db.id.clone();
        Callback::from(move |_: MouseEvent| {
            on_remove.emit(db_id.clone());
        })
    };

    html! {
        <div class="database-card">
            <div class="database-card-header" onclick={on_toggle}>
                <div class="db-info">
                    <span
                        class="db-color-dot"
                        style={format!("background-color: {}", props.db.color)}
                    />
                    <span class="db-name">{&props.db.name}</span>
                </div>
                <div class="db-actions">
                    <span class={classes!("expand-icon", (*is_expanded).then_some("expanded"))}>
                        {"▼"}
                    </span>
                    <button class="remove-btn" onclick={on_remove}>
                        {"🗑️"}
                    </button>
                </div>
            </div>

            {if *is_expanded {
                html! {
                    <div class="database-card-body">
                        <div class="form-group">
                            <label>{"显示名称"}</label>
                            <input
                                type="text"
                                value={props.db.name.clone()}
                                onchange={update_name}
                            />
                        </div>
                        <div class="form-group">
                            <label>{"Notion Database ID"}</label>
                            <input
                                type="text"
                                value={props.db.notion_database_id.clone()}
                                onchange={update_db_id}
                                placeholder="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
                            />
                            <span class="help-text">
                                {"从 Notion 数据库页面 URL 复制 32 位 ID"}
                            </span>
                        </div>
                        <div class="form-row">
                            <div class="form-group">
                                <label>{"日期属性名"}</label>
                                <input
                                    type="text"
                                    value={props.db.date_property.clone()}
                                    onchange={update_date_prop}
                                    placeholder="Date"
                                />
                            </div>
                            <div class="form-group">
                                <label>{"标题属性名"}</label>
                                <input
                                    type="text"
                                    value={props.db.title_property.clone()}
                                    onchange={update_title_prop}
                                    placeholder="Name"
                                />
                            </div>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}
