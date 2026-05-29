use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

use crate::store;

const KEY: &str = "daykit:todo";

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Todo {
    id: u64,
    text: String,
    done: bool,
    #[serde(default)]
    notes: String,
    #[serde(default)]
    due_at: Option<String>, // "YYYY-MM-DD"
    #[serde(default)]
    created_at: u64,
    #[serde(default)]
    updated_at: u64,
}

#[derive(Clone, PartialEq)]
enum Filter {
    All,
    Active,
    Done,
}

pub fn TodoView() -> Element {
    let mut items = use_signal(|| store::load::<Vec<Todo>>(KEY));
    let mut draft = use_signal(String::new);
    let mut filter = use_signal(|| Filter::All);
    let mut expanded: Signal<Option<u64>> = use_signal(|| None);

    use_effect(move || {
        store::save(KEY, &items());
    });

    let all = items();
    let total = all.len();
    let done_count = all.iter().filter(|t| t.done).count();
    let active_count = total - done_count;
    let pct = if total > 0 { done_count * 100 / total } else { 0 };
    let today_str = today();

    let visible: Vec<Todo> = all
        .iter()
        .filter(|t| match filter() {
            Filter::All => true,
            Filter::Active => !t.done,
            Filter::Done => t.done,
        })
        .cloned()
        .collect();

    rsx! {
        div { class: "tool",

            div { class: "add-row",
                input {
                    class: "add-input",
                    placeholder: "新增待办…",
                    value: "{draft}",
                    oninput: move |e| draft.set(e.value()),
                    onkeydown: move |e| {
                        if e.key() == Key::Enter { add_todo(items, draft); }
                    },
                }
                button {
                    class: "add-btn",
                    onclick: move |_| add_todo(items, draft),
                    "+"
                }
            }

            if total > 0 {
                div { class: "toolbar",
                    div { class: "progress",
                        div { class: "progress-track",
                            div { class: "progress-fill", style: "width:{pct}%" }
                        }
                        span { class: "progress-label", "{done_count}/{total} 完成" }
                    }
                    div { class: "filters",
                        button {
                            class: "filter-btn",
                            class: if matches!(filter(), Filter::All) { "active" },
                            onclick: move |_| filter.set(Filter::All),
                            "全部"
                            span { class: "badge", "{total}" }
                        }
                        button {
                            class: "filter-btn",
                            class: if matches!(filter(), Filter::Active) { "active" },
                            onclick: move |_| filter.set(Filter::Active),
                            "待办"
                            if active_count > 0 { span { class: "badge", "{active_count}" } }
                        }
                        button {
                            class: "filter-btn",
                            class: if matches!(filter(), Filter::Done) { "active" },
                            onclick: move |_| filter.set(Filter::Done),
                            "已完成"
                            if done_count > 0 { span { class: "badge", "{done_count}" } }
                        }
                    }
                }
            }

            if visible.is_empty() {
                div { class: "empty-state",
                    p { class: "empty",
                        if total == 0 { "暂无待办 — 加一条吧" } else { "此分类下暂无内容" }
                    }
                }
            }

            ul { class: "list",
                for item in visible {
                    {
                        let item_id   = item.id;
                        let item_done = item.done;
                        let is_open   = expanded() == Some(item_id);
                        let is_over   = item.due_at.as_deref()
                            .map(|d| !item_done && d < today_str.as_str())
                            .unwrap_or(false);
                        let due_disp  = item.due_at.as_deref()
                            .filter(|s| !s.is_empty())
                            .map(fmt_due)
                            .unwrap_or_default();
                        let (dy, dm, dd) = ymd(item.due_at.as_deref());
                        let item_created = item.created_at;
                        let item_updated = item.updated_at;
                        let created_disp = if item_created > 0 { fmt_date(item_created) } else { String::new() };
                        let updated_disp = if item_updated > item_created { fmt_date(item_updated) } else { String::new() };

                        rsx! {
                            li {
                                key: "{item_id}",
                                class: "row",
                                class: if item_done { "done" },
                                class: if is_open { "open" },

                                div { class: "row-main",
                                    label { class: "check-wrap",
                                        input {
                                            r#type: "checkbox",
                                            checked: item_done,
                                            onchange: { move |_| toggle(items, item_id) },
                                        }
                                        span { class: "checkmark" }
                                    }

                                    div {
                                        class: "row-body",
                                        onclick: {
                                            move |_| {
                                                expanded.set(
                                                    if expanded() == Some(item_id) { None } else { Some(item_id) }
                                                );
                                            }
                                        },
                                        span { class: "row-text", "{item.text}" }
                                        if !due_disp.is_empty() {
                                            span {
                                                class: "due-badge",
                                                class: if is_over { "overdue" },
                                                "📅 {due_disp}"
                                            }
                                        }
                                    }

                                    button {
                                        class: "del",
                                        onclick: { move |_| remove(items, item_id) },
                                        "×"
                                    }
                                }

                                if is_open {
                                    div { class: "row-detail",
                                        textarea {
                                            class: "notes-input",
                                            placeholder: "添加备注…",
                                            value: "{item.notes}",
                                            rows: "3",
                                            oninput: { move |e| update_notes(items, item_id, e.value()) },
                                        }

                                        div { class: "detail-meta",
                                            div { class: "detail-row",
                                                span { class: "detail-label", "截止日期" }
                                                div { class: "date-parts",
                                                    input {
                                                        r#type: "number",
                                                        class: "date-part-input",
                                                        placeholder: "年",
                                                        min: "2020", max: "2099",
                                                        value: "{dy}",
                                                        onchange: {
                                                            move |e| {
                                                                let cur = get_due(items, item_id);
                                                                let (_, m, d) = ymd(cur.as_deref());
                                                                update_due(items, item_id, build_due(&e.value(), &m, &d));
                                                            }
                                                        },
                                                    }
                                                    span { class: "date-sep", "年" }
                                                    input {
                                                        r#type: "number",
                                                        class: "date-part-input",
                                                        placeholder: "月",
                                                        min: "1", max: "12",
                                                        value: "{dm}",
                                                        onchange: {
                                                            move |e| {
                                                                let cur = get_due(items, item_id);
                                                                let (y, _, d) = ymd(cur.as_deref());
                                                                update_due(items, item_id, build_due(&y, &e.value(), &d));
                                                            }
                                                        },
                                                    }
                                                    span { class: "date-sep", "月" }
                                                    input {
                                                        r#type: "number",
                                                        class: "date-part-input",
                                                        placeholder: "日",
                                                        min: "1", max: "31",
                                                        value: "{dd}",
                                                        onchange: {
                                                            move |e| {
                                                                let cur = get_due(items, item_id);
                                                                let (y, m, _) = ymd(cur.as_deref());
                                                                update_due(items, item_id, build_due(&y, &m, &e.value()));
                                                            }
                                                        },
                                                    }
                                                    span { class: "date-sep", "日" }
                                                    if item.due_at.is_some() {
                                                        button {
                                                            class: "date-clear",
                                                            onclick: { move |_| update_due(items, item_id, String::new()) },
                                                            "✕"
                                                        }
                                                    }
                                                }
                                            }

                                            div { class: "timestamps",
                                                if !created_disp.is_empty() {
                                                    span { "创建 {created_disp}" }
                                                }
                                                if !updated_disp.is_empty() {
                                                    span { "· 更新 {updated_disp}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if done_count > 0 {
                button {
                    class: "clear-btn",
                    onclick: move |_| items.write().retain(|t| !t.done),
                    "清除已完成 {done_count} 项"
                }
            }
        }
    }
}

// ─── 时间 ─────────────────────────────────────────────────────────

fn now_ms() -> u64 { js_sys::Date::now() as u64 }

fn today() -> String {
    let d = js_sys::Date::new_0();
    format!("{}-{:02}-{:02}", d.get_full_year(), d.get_month() + 1, d.get_date())
}

fn fmt_date(ms: u64) -> String {
    let d = js_sys::Date::new(&JsValue::from_f64(ms as f64));
    let y = d.get_full_year();
    let mo = d.get_month() + 1;
    let day = d.get_date();
    let h = d.get_hours();
    let mi = d.get_minutes();
    if format!("{y}-{mo:02}-{day:02}") == today() {
        format!("今天 {h:02}:{mi:02}")
    } else {
        format!("{y}/{mo:02}/{day:02}  {h:02}:{mi:02}")
    }
}

// 把 "YYYY-MM-DD" 格式化成"X月X日"（本年）或"X年X月X日"（跨年）
fn fmt_due(s: &str) -> String {
    let (y, m, d) = ymd(Some(s));
    if y.is_empty() || m.is_empty() || d.is_empty() { return String::new(); }
    let this_year = js_sys::Date::new_0().get_full_year().to_string();
    if y == this_year {
        format!("{}月{}日", m, d)
    } else {
        format!("{}年{}月{}日", y, m, d)
    }
}

// ─── 日期字符串工具 ───────────────────────────────────────────────

// "YYYY-MM-DD" → ("YYYY", "M", "D")（去掉前导零方便 number input 显示）
fn ymd(s: Option<&str>) -> (String, String, String) {
    let s = match s.filter(|s| !s.is_empty()) {
        Some(v) => v,
        None => return (String::new(), String::new(), String::new()),
    };
    let p: Vec<&str> = s.splitn(3, '-').collect();
    if p.len() != 3 {
        return (String::new(), String::new(), String::new());
    }
    let m = p[1].trim_start_matches('0');
    let d = p[2].trim_start_matches('0');
    (
        p[0].to_string(),
        if m.is_empty() { "0".into() } else { m.into() },
        if d.is_empty() { "0".into() } else { d.into() },
    )
}

// ("YYYY", "M", "D") → "YYYY-MM-DD"，任一为空/无效则返回空串
fn build_due(y: &str, m: &str, d: &str) -> String {
    let yn: u32 = y.parse().unwrap_or(0);
    let mn: u32 = m.parse().unwrap_or(0);
    let dn: u32 = d.parse().unwrap_or(0);
    if yn >= 2000 && mn >= 1 && mn <= 12 && dn >= 1 && dn <= 31 {
        format!("{}-{:02}-{:02}", yn, mn, dn)
    } else {
        String::new()
    }
}

fn get_due(items: Signal<Vec<Todo>>, id: u64) -> Option<String> {
    items().into_iter().find(|t| t.id == id).and_then(|t| t.due_at)
}

// ─── 状态变更 ─────────────────────────────────────────────────────

fn add_todo(mut items: Signal<Vec<Todo>>, mut draft: Signal<String>) {
    let text = draft().trim().to_string();
    if text.is_empty() { return; }
    let next_id = items().iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let now = now_ms();
    items.write().push(Todo {
        id: next_id, text, done: false,
        notes: String::new(), due_at: None,
        created_at: now, updated_at: now,
    });
    draft.set(String::new());
}

fn toggle(mut items: Signal<Vec<Todo>>, id: u64) {
    let now = now_ms();
    if let Some(t) = items.write().iter_mut().find(|t| t.id == id) {
        t.done = !t.done;
        t.updated_at = now;
    }
}

fn remove(mut items: Signal<Vec<Todo>>, id: u64) {
    items.write().retain(|t| t.id != id);
}

fn update_notes(mut items: Signal<Vec<Todo>>, id: u64, notes: String) {
    let now = now_ms();
    if let Some(t) = items.write().iter_mut().find(|t| t.id == id) {
        t.notes = notes;
        t.updated_at = now;
    }
}

fn update_due(mut items: Signal<Vec<Todo>>, id: u64, due: String) {
    let now = now_ms();
    if let Some(t) = items.write().iter_mut().find(|t| t.id == id) {
        t.due_at = if due.is_empty() { None } else { Some(due) };
        t.updated_at = now;
    }
}
