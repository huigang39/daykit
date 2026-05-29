//! 同步层(目前是占位骨架)。
//!
//! 设计思路(单用户多设备,所以很简单):
//!   - 本地(store.rs)是你随时可写的"工作副本",离线也能用。
//!   - 后端(推荐 PocketBase)是跨设备的"真相来源"。
//!   - 同步策略用最朴素的"最后写入者获胜":每条记录带一个 updated_at
//!     时间戳,合并时谁的时间新就用谁。对个人工具足够了,不需要 CRDT。
//!
//! 为什么先留空:真正的同步需要你的 PocketBase 实例跑起来,而且要选一个
//! HTTP 客户端(WASM 下推荐 `gloo-net` 的 fetch)。README 里有完整步骤。
//! 在后端就绪前,应用纯靠本地存储也能完整使用。
//!
//! 接好后端后,典型用法:启动时 pull() 一次,本地每次改动后 push()。

#![allow(dead_code)]

/// 从后端拉取最新数据并合并到本地(按 updated_at 取新)。
pub async fn pull() {
    // TODO: GET {BASE}/api/collections/<tool>/records
    //       解析后逐条与本地比较 updated_at,新的覆盖旧的,写回 store。
}

/// 把本地的新增/改动推送到后端。
pub async fn push() {
    // TODO: 对每条本地比后端新的记录:
    //       新增 -> POST {BASE}/api/collections/<tool>/records
    //       更新 -> PATCH {BASE}/api/collections/<tool>/records/<id>
}
