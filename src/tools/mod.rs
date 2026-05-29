//! 工具注册表。
//!
//! 每个工具是 `src/tools/` 下的一个独立模块,只要满足两件事:
//!   1. 导出一个组件(例如 `pub fn TodoView() -> Element`)
//!   2. 在下面的 `registry()` 里登记一条元信息(用于底部标签栏)
//! 然后在 `main.rs` 的 `ToolOutlet` 里加一个 match 分支即可。

pub mod todo;
// pub mod notes;     // 以后加灵感/笔记工具时取消注释
// pub mod expenses;  // 以后加记账工具时取消注释

/// 一个工具在标签栏里的元信息。
#[derive(Clone, PartialEq)]
pub struct ToolMeta {
    /// 唯一标识,要和 main.rs 里 ToolOutlet 的 match 分支一致
    pub id: &'static str,
    /// 标签栏显示的名字
    pub name: &'static str,
    /// 标签栏显示的图标(先用 emoji,够用且零依赖)
    pub icon: &'static str,
}

/// 所有已启用的工具。调整顺序 = 调整标签栏顺序。
pub fn registry() -> Vec<ToolMeta> {
    vec![
        ToolMeta { id: "todo", name: "待办", icon: "✓" },
        // ToolMeta { id: "notes",    name: "灵感", icon: "💡" },
        // ToolMeta { id: "expenses", name: "记账", icon: "¥" },
    ]
}
