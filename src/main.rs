use chrono::{Duration, NaiveDate};
use yew::prelude::*;

mod api;
mod components;
mod pages;
mod types;
mod utils;

use components::{Calendar, EventModal, SettingsPanel};
use types::{AppState, CalendarEvent, DatabaseConfig, ViewMode};
use utils::load_state;

#[function_component(App)]
fn app() -> Html {
    wasm_logger::init(wasm_logger::Config::default());

    let initial_state = load_state().unwrap_or_default();
    let state = use_state(|| initial_state);
    let events = use_state(Vec::<CalendarEvent>::new);
    let show_settings = use_state(|| false);
    let show_event_modal = use_state(|| false);
    let selected_date = use_state(|| None::<NaiveDate>);
    let editing_event = use_state(|| None::<CalendarEvent>);
    let is_loading = use_state(|| false);
    let error_message = use_state(|| None::<String>);

    // 刷新事件
    let refresh_events = {
        let state = state.clone();
        let events = events.clone();
        let is_loading = is_loading.clone();
        let error_message = error_message.clone();

        Callback::from(move |_: ()| {
            let state = state.clone();
            let events = events.clone();
            let is_loading = is_loading.clone();
            let error_message = error_message.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if state.notion_api_key.is_none() {
                    return;
                }

                is_loading.set(true);
                error_message.set(None);

                let client = api::NotionClient::new(
                    state.notion_api_key.clone().unwrap()
                );

                let mut all_events = Vec::new();

                for db_config in &state.databases {
                    if db_config.notion_database_id.is_empty() {
                        continue;
                    }

                    match api::load_events_from_database(&client, db_config).await {
                        Ok(mut db_events) => {
                            all_events.append(&mut db_events);
                        }
                        Err(e) => {
                            log::error!("加载数据库 {} 失败: {}", db_config.name, e);
                        }
                    }
                }

                events.set(all_events);
                is_loading.set(false);
            });
        })
    };

    // 初始加载
    {
        let refresh_events = refresh_events.clone();
        use_effect_with((), move |_: &()| {
            refresh_events.emit(());
            || ()
        });
    }

    // 视图切换
    let on_view_change = {
        let state = state.clone();
        Callback::from(move |view: ViewMode| {
            let mut new_state = (*state).clone();
            new_state.current_view = view;
            state.set(new_state);
        })
    };

    // 日期导航
    let on_prev = {
        let state = state.clone();
        Callback::from(move |_| {
            let mut new_state = (*state).clone();
            new_state.current_date = match new_state.current_view {
                ViewMode::Month => {
                    new_state.current_date - Duration::days(30)
                }
                ViewMode::Week => {
                    new_state.current_date - Duration::days(7)
                }
                ViewMode::Day => {
                    new_state.current_date - Duration::days(1)
                }
            };
            state.set(new_state);
        })
    };

    let on_next = {
        let state = state.clone();
        Callback::from(move |_| {
            let mut new_state = (*state).clone();
            new_state.current_date = match new_state.current_view {
                ViewMode::Month => {
                    new_state.current_date + Duration::days(30)
                }
                ViewMode::Week => {
                    new_state.current_date + Duration::days(7)
                }
                ViewMode::Day => {
                    new_state.current_date + Duration::days(1)
                }
            };
            state.set(new_state);
        })
    };

    let on_today = {
        let state = state.clone();
        Callback::from(move |_| {
            let mut new_state = (*state).clone();
            new_state.current_date = chrono::Local::now().naive_local().date();
            state.set(new_state);
        })
    };

    // 日期点击 - 新建事件
    let on_date_click = {
        let show_event_modal = show_event_modal.clone();
        let selected_date = selected_date.clone();
        let editing_event = editing_event.clone();
        Callback::from(move |date: NaiveDate| {
            selected_date.set(Some(date));
            editing_event.set(None);
            show_event_modal.set(true);
        })
    };

    // 事件点击 - 编辑
    let on_event_click = {
        let show_event_modal = show_event_modal.clone();
        let editing_event = editing_event.clone();
        Callback::from(move |event: CalendarEvent| {
            editing_event.set(Some(event));
            show_event_modal.set(true);
        })
    };

    // 保存事件
    let on_save_event = {
        let state = state.clone();
        let refresh_events = refresh_events.clone();
        let error_message = error_message.clone();

        Callback::from(move |(title, date, db_id, page_id): (String, String, String, Option<String>)| {
            let state = state.clone();
            let refresh_events = refresh_events.clone();
            let error_message = error_message.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Some(ref api_key) = state.notion_api_key {
                    let client = api::NotionClient::new(api_key.clone());

                    let db_config = state.databases.iter()
                        .find(|d| d.id == db_id);

                    if let Some(config) = db_config {
                        let result = if let Some(page_id) = page_id {
                            client.update_page(&page_id, Some(&title), Some(&date)).await
                        } else {
                            client.create_page(&config.notion_database_id, &title, &date).await
                        };

                        match result {
                            Ok(_) => {
                                refresh_events.emit(());
                            }
                            Err(e) => {
                                error_message.set(Some(format!("保存失败: {}", e)));
                            }
                        }
                    }
                }
            });
        })
    };

    // 删除事件
    let on_delete_event = {
        let state = state.clone();
        let refresh_events = refresh_events.clone();
        let error_message = error_message.clone();

        Callback::from(move |page_id: String| {
            let state = state.clone();
            let refresh_events = refresh_events.clone();
            let error_message = error_message.clone();

            wasm_bindgen_futures::spawn_local(async move {
                if let Some(ref api_key) = state.notion_api_key {
                    let client = api::NotionClient::new(api_key.clone());

                    match client.delete_page(&page_id).await {
                        Ok(_) => {
                            refresh_events.emit(());
                        }
                        Err(e) => {
                            error_message.set(Some(format!("删除失败: {}", e)));
                        }
                    }
                }
            });
        })
    };

    // 保存设置
    let on_settings_save = {
        let state = state.clone();
        let refresh_events = refresh_events.clone();
        Callback::from(move |new_state: AppState| {
            state.set(new_state);
            refresh_events.emit(());
        })
    };

    let on_close_settings = {
        let show_settings = show_settings.clone();
        Callback::from(move |_| show_settings.set(false))
    };

    let on_open_settings = {
        let show_settings = show_settings.clone();
        Callback::from(move |_| show_settings.set(true))
    };

    let on_close_event_modal = {
        let show_event_modal = show_event_modal.clone();
        Callback::from(move |_| show_event_modal.set(false))
    };

    let on_close_error = {
        let error_message = error_message.clone();
        Callback::from(move |_| error_message.set(None))
    };

    html! {
        <div class="app">
            <header class="app-header">
                <div class="logo">
                    <span class="logo-icon">{"☕️"}</span>
                    <h1>{"Notion Cafe"}</h1>
                </div>

                <div class="nav-controls">
                    <button class="nav-btn" onclick={on_prev}>
                        {"◀"}
                    </button>
                    <button class="today-btn" onclick={on_today}>
                        {"今天"}
                    </button>
                    <button class="nav-btn" onclick={on_next}>
                        {"▶"}
                    </button>
                </div>

                <div class="view-controls">
                    <button
                        class={classes!("view-btn", (state.current_view == ViewMode::Month).then_some("active"))}
                        onclick={on_view_change.reform(|_| ViewMode::Month)}
                    >
                        {"月"}
                    </button>
                    <button
                        class={classes!("view-btn", (state.current_view == ViewMode::Week).then_some("active"))}
                        onclick={on_view_change.reform(|_| ViewMode::Week)}
                    >
                        {"周"}
                    </button>
                    <button
                        class={classes!("view-btn", (state.current_view == ViewMode::Day).then_some("active"))}
                        onclick={on_view_change.reform(|_| ViewMode::Day)}
                    >
                        {"日"}
                    </button>
                </div>

                <div class="header-actions">
                    <button
                        class="settings-btn"
                        onclick={on_open_settings}
                    >
                        {"⚙️"}
                    </button>
                </div>
            </header>

            <main class="app-main">
                {if *is_loading {
                    html! {
                        <div class="loading-overlay">
                            <div class="loading-spinner"></div>
                            <span>{"同步中..."}</span>
                        </div>
                    }
                } else {
                    html! {}
                }}

                {if let Some(ref error) = *error_message {
                    html! {
                        <div class="error-banner">
                            <span>{error}</span>
                            <button onclick={on_close_error}>
                                {"✕"}
                            </button>
                        </div>
                    }
                } else {
                    html! {}
                }}

                <Calendar
                    current_date={state.current_date}
                    view_mode={state.current_view}
                    events={(*events).clone()}
                    on_date_click={on_date_click}
                    on_event_click={on_event_click}
                />
            </main>

            {if *show_settings {
                html! {
                    <SettingsPanel
                        state={(*state).clone()}
                        on_state_change={on_settings_save}
                        on_close={on_close_settings}
                    />
                }
            } else {
                html! {}
            }}

            {if *show_event_modal {
                html! {
                    <EventModal
                        event={(*editing_event).clone()}
                        databases={state.databases.clone()}
                        selected_date={*selected_date}
                        on_save={on_save_event}
                        on_delete={on_delete_event}
                        on_close={on_close_event_modal}
                    />
                }
            } else {
                html! {}
            }}
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
