use chrono::NaiveDate;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::types::{CalendarEvent, DatabaseConfig};

#[derive(Properties, Clone, PartialEq)]
pub struct EventModalProps {
    pub event: Option<CalendarEvent>, // None = 新建事件
    pub databases: Vec<DatabaseConfig>,
    pub selected_date: Option<NaiveDate>,
    pub on_save: Callback<(String, String, String, Option<String>)>, // (title, date, db_id, page_id)
    pub on_delete: Callback<String>, // page_id
    pub on_close: Callback<()>,
}

#[function_component(EventModal)]
pub fn event_modal(props: &EventModalProps) -> Html {
    let is_new = props.event.is_none();
    let page_id = props.event.as_ref().and_then(|e| e.notion_page_id.clone());

    // 表单状态
    let title = use_state(|| {
        props
            .event
            .as_ref()
            .map(|e| e.title.clone())
            .unwrap_or_default()
    });

    let date = use_state(|| {
        props
            .event
            .as_ref()
            .map(|e| e.start_date.to_string())
            .unwrap_or_else(|| {
                props
                    .selected_date
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| chrono::Local::now().naive_local().date().to_string())
            })
    });

    let selected_db = use_state(|| {
        props
            .event
            .as_ref()
            .map(|e| e.database_id.clone())
            .unwrap_or_else(|| {
                props
                    .databases
                    .first()
                    .map(|d| d.id.clone())
                    .unwrap_or_default()
            })
    });

    let error_message = use_state(|| None::<String>);

    let on_title_change = {
        let title = title.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            title.set(input.value());
        })
    };

    let on_date_change = {
        let date = date.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            date.set(input.value());
        })
    };

    let on_db_change = {
        let selected_db = selected_db.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            selected_db.set(input.value());
        })
    };

    let on_submit = {
        let title = title.clone();
        let date = date.clone();
        let selected_db = selected_db.clone();
        let page_id = page_id.clone();
        let on_save = props.on_save.clone();
        let on_close = props.on_close.clone();
        let error_message = error_message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if title.is_empty() {
                error_message.set(Some("请输入事件标题".to_string()));
                return;
            }

            on_save.emit((
                (*title).clone(),
                (*date).clone(),
                (*selected_db).clone(),
                page_id.clone(),
            ));
            on_close.emit(());
        })
    };

    let on_delete = {
        let on_delete = props.on_delete.clone();
        let on_close = props.on_close.clone();
        let page_id = page_id.clone();

        Callback::from(move |_: MouseEvent| {
            if let Some(ref id) = page_id {
                on_delete.emit(id.clone());
                on_close.emit(());
            }
        })
    };

    html! {
        <div class="modal-overlay" onclick={props.on_close.reform(|_| ())}>
            <div class="modal" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                <div class="modal-header">
                    <h3>{if is_new { "✨ 新建事件" } else { "✏️ 编辑事件" }}</h3>
                    <button class="close-btn" onclick={props.on_close.reform(|_| ())}>
                        {"✕"}
                    </button>
                </div>

                <form onsubmit={on_submit}>
                    <div class="modal-body">
                        <div class="form-group">
                            <label>{"标题"}</label>
                            <input
                                type="text"
                                value={(*title).clone()}
                                onchange={on_title_change}
                                placeholder="事件标题"
                                required={true}
                            />
                        </div>

                        <div class="form-group">
                            <label>{"日期"}</label>
                            <input
                                type="date"
                                value={(*date).clone()}
                                onchange={on_date_change}
                                required={true}
                            />
                        </div>

                        <div class="form-group">
                            <label>{"所属数据库"}</label>
                            <select value={(*selected_db).clone()} onchange={on_db_change}>
                                {props.databases.iter().map(|db| {
                                    html! {
                                        <option value={db.id.clone()}>
                                            {&db.name}
                                        </option>
                                    }
                                }).collect::<Html>()}
                            </select>
                        </div>

                        {if let Some(ref error) = *error_message {
                            html! { <div class="alert alert-error">{error}</div> }
                        } else {
                            html! {}
                        }}
                    </div>

                    <div class="modal-footer">
                        {if !is_new {
                            html! {
                                <button
                                    type="button"
                                    class="btn-danger"
                                    onclick={on_delete}
                                >
                                    {"删除"}
                                </button>
                            }
                        } else {
                            html! {}
                        }}
                        <button type="button" class="btn-secondary" onclick={props.on_close.reform(|_| ())}>
                            {"取消"}
                        </button>
                        <button type="submit" class="btn-primary">
                            {if is_new { "创建" } else { "保存" }}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}
