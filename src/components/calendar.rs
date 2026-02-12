use chrono::{Datelike, NaiveDate, Weekday};
use yew::prelude::*;

use crate::types::{CalendarEvent, ViewMode};
use crate::utils::{month_name, weekday_name};

#[derive(Properties, Clone, PartialEq)]
pub struct CalendarProps {
    pub current_date: NaiveDate,
    pub view_mode: ViewMode,
    pub events: Vec<CalendarEvent>,
    pub on_date_click: Callback<NaiveDate>,
    pub on_event_click: Callback<CalendarEvent>,
}

#[function_component(Calendar)]
pub fn calendar(props: &CalendarProps) -> Html {
    match props.view_mode {
        ViewMode::Month => html! {
            <MonthView
                current_date={props.current_date}
                events={props.events.clone()}
                on_date_click={props.on_date_click.clone()}
                on_event_click={props.on_event_click.clone()}
            />
        },
        ViewMode::Week => html! {
            <WeekView
                current_date={props.current_date}
                events={props.events.clone()}
                on_date_click={props.on_date_click.clone()}
                on_event_click={props.on_event_click.clone()}
            />
        },
        ViewMode::Day => html! {
            <DayView
                current_date={props.current_date}
                events={props.events.clone()}
                on_date_click={props.on_date_click.clone()}
                on_event_click={props.on_event_click.clone()}
            />
        },
    }
}

// ========== 月视图 ==========
#[derive(Properties, Clone, PartialEq)]
struct MonthViewProps {
    current_date: NaiveDate,
    events: Vec<CalendarEvent>,
    on_date_click: Callback<NaiveDate>,
    on_event_click: Callback<CalendarEvent>,
}

#[function_component(MonthView)]
fn month_view(props: &MonthViewProps) -> Html {
    let year = props.current_date.year();
    let month = props.current_date.month();

    // 计算当月第一天是星期几
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let first_weekday = first_day.weekday();

    // 计算需要显示的上月天数
    let days_from_prev_month = match first_weekday {
        Weekday::Mon => 0,
        Weekday::Tue => 1,
        Weekday::Wed => 2,
        Weekday::Thu => 3,
        Weekday::Fri => 4,
        Weekday::Sat => 5,
        Weekday::Sun => 6,
    };

    // 计算当月天数
    let days_in_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    }
    .unwrap()
    .signed_duration_since(first_day)
    .num_days() as usize;

    // 生周日历格子
    let total_cells = days_from_prev_month + days_in_month;
    let rows = (total_cells + 6) / 7;
    let total_cells = rows * 7;

    let cells: Vec<Html> = (0..total_cells)
        .map(|i| {
            let day_offset = i as i64 - days_from_prev_month as i64;
            let date = first_day + chrono::Duration::days(day_offset);
            let is_current_month = date.month() == month;
            let is_today = date == chrono::Local::now().naive_local().date();

            // 获取当天的事件
            let day_events: Vec<&CalendarEvent> = props
                .events
                .iter()
                .filter(|e| {
                    date >= e.start_date
                        && date
                            <= e.end_date.unwrap_or(e.start_date)
                })
                .collect();

            let on_click = {
                let on_date_click = props.on_date_click.clone();
                let date = date;
                Callback::from(move |_| on_date_click.emit(date))
            };

            html! {
                <div
                    class={classes!(
                        "calendar-day",
                        (!is_current_month).then_some("other-month"),
                        is_today.then_some("today")
                    )}
                    onclick={on_click}
                >
                    <span class="day-number">{date.day()}</span>
                    <div class="day-events">
                        {day_events.iter().map(|event| {
                            let on_event_click = {
                                let on_event_click = props.on_event_click.clone();
                                let event = (*event).clone();
                                Callback::from(move |e: MouseEvent| {
                                    e.stop_propagation();
                                    on_event_click.emit(event.clone())
                                })
                            };
                            html! {
                                <div
                                    class="event-chip"
                                    style={format!("background-color: {}", event.color)}
                                    onclick={on_event_click}
                                >
                                    {&event.title}
                                </div>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>
            }
        })
        .collect();

    html! {
        <div class="month-view">
            <div class="calendar-header">
                <h2>{format!("{} {}", year, month_name(month))}</h2>
            </div>
            <div class="weekdays">
                {["周一", "周二", "周三", "周四", "周五", "周六", "周日"]
                    .iter()
                    .map(|d| html! { <div class="weekday">{d}</div> })
                    .collect::<Html>()}
            </div>
            <div class="calendar-grid">
                {cells}
            </div>
        </div>
    }
}

// ========== 周视图 ==========
#[derive(Properties, Clone, PartialEq)]
struct WeekViewProps {
    current_date: NaiveDate,
    events: Vec<CalendarEvent>,
    on_date_click: Callback<NaiveDate>,
    on_event_click: Callback<CalendarEvent>,
}

#[function_component(WeekView)]
fn week_view(props: &WeekViewProps) -> Html {
    // 计算本周开始（周一）
    let weekday = props.current_date.weekday();
    let days_from_monday = match weekday {
        Weekday::Mon => 0,
        Weekday::Tue => 1,
        Weekday::Wed => 2,
        Weekday::Thu => 3,
        Weekday::Fri => 4,
        Weekday::Sat => 5,
        Weekday::Sun => 6,
    };

    let week_start = props.current_date - chrono::Duration::days(days_from_monday);

    let days: Vec<Html> = (0..7)
        .map(|i| {
            let date = week_start + chrono::Duration::days(i);
            let is_today = date == chrono::Local::now().naive_local().date();

            let day_events: Vec<&CalendarEvent> = props
                .events
                .iter()
                .filter(|e| {
                    date >= e.start_date
                        && date <= e.end_date.unwrap_or(e.start_date)
                })
                .collect();

            let on_click = {
                let on_date_click = props.on_date_click.clone();
                let date = date;
                Callback::from(move |_| on_date_click.emit(date))
            };

            html! {
                <div
                    class={classes!("week-day", is_today.then_some("today"))}
                    onclick={on_click}
                >
                    <div class="week-day-header">
                        <span class="week-day-name">{weekday_name(date.weekday())}</span>
                        <span class="week-day-number">{date.day()}</span>
                    </div>
                    <div class="week-day-events">
                        {day_events.iter().map(|event| {
                            let on_event_click = {
                                let on_event_click = props.on_event_click.clone();
                                let event = (*event).clone();
                                Callback::from(move |e: MouseEvent| {
                                    e.stop_propagation();
                                    on_event_click.emit(event.clone())
                                })
                            };
                            html! {
                                <div
                                    class="week-event-card"
                                    style={format!("border-left-color: {}", event.color)}
                                    onclick={on_event_click}
                                >
                                    <div class="event-title">{&event.title}</div>
                                </div>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>
            }
        })
        .collect();

    html! {
        <div class="week-view">
            <div class="calendar-header">
                <h2>{
                    format!(
                        "{} {} - {} {}",
                        month_name(week_start.month()),
                        week_start.day(),
                        month_name((week_start + chrono::Duration::days(6)).month()),
                        (week_start + chrono::Duration::days(6)).day()
                    )
                }</h2>
            </div>
            <div class="week-grid">
                {days}
            </div>
        </div>
    }
}

// ========== 日视图 ==========
#[derive(Properties, Clone, PartialEq)]
struct DayViewProps {
    current_date: NaiveDate,
    events: Vec<CalendarEvent>,
    on_date_click: Callback<NaiveDate>,
    on_event_click: Callback<CalendarEvent>,
}

#[function_component(DayView)]
fn day_view(props: &DayViewProps) -> Html {
    let is_today = props.current_date == chrono::Local::now().naive_local().date();

    let day_events: Vec<&CalendarEvent> = props
        .events
        .iter()
        .filter(|e| {
            props.current_date >= e.start_date
                && props.current_date <= e.end_date.unwrap_or(e.start_date)
        })
        .collect();

    html! {
        <div class="day-view">
            <div class="calendar-header">
                <h2 class={classes!(is_today.then_some("today-header"))}>
                    {format!(
                        "{} {} {}日",
                        props.current_date.year(),
                        month_name(props.current_date.month()),
                        props.current_date.day()
                    )}
                </h2>
            </div>
            <div class="day-content">
                <div class="day-sidebar">
                    <div class="day-info">
                        <span class="day-weekday">{weekday_name(props.current_date.weekday())}</span>
                        <span class="day-number-large">{props.current_date.day()}</span>
                    </div>
                </div>
                <div class="day-events-list">
                    {if day_events.is_empty() {
                        html! {
                            <div class="no-events">
                                <span class="no-events-icon">{"☕️"}</span>
                                <p>{"今天没有安排"}</p>
                            </div>
                        }
                    } else {
                        html! {
                            {day_events.iter().map(|event| {
                                let on_event_click = {
                                    let on_event_click = props.on_event_click.clone();
                                    let event = (*event).clone();
                                    Callback::from(move |_| on_event_click.emit(event.clone()))
                                };
                                html! {
                                    <div
                                        class="day-event-item"
                                        style={format!("border-left-color: {}", event.color)}
                                        onclick={on_event_click}
                                    >
                                        <div class="event-color-dot" style={format!("background-color: {}", event.color)}></div>
                                        <div class="event-details">
                                            <div class="event-title-large">{&event.title}</div>
                                            <div class="event-date">
                                                {format!("{} - {}",
                                                    crate::utils::format_date(&event.start_date),
                                                    event.end_date.map(|d| crate::utils::format_date(&d))
                                                        .unwrap_or_else(|| "无结束日期".to_string())
                                                )}
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()}
                        }
                    }}
                </div>
            </div>
        </div>
    }
}
