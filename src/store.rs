//! 本地存储:浏览器 localStorage 的极简封装。
//!
//! 注意:在 iPhone 上,网页应用的本地存储不是绝对可靠的(系统可能在
//! 长期不用后回收),所以最终方案里应把"后端"当作真相来源(见 sync.rs),
//! 本地只当快取。但作为起步,先把本地跑通就够了。

use gloo_storage::{LocalStorage, Storage};
use serde::{de::DeserializeOwned, Serialize};

/// 读取某个 key 的数据;读不到或解析失败就返回该类型的默认值。
pub fn load<T: DeserializeOwned + Default>(key: &str) -> T {
    LocalStorage::get(key).unwrap_or_default()
}

/// 把数据写回某个 key。失败时静默忽略(本地存储满/被禁用等)。
pub fn save<T: Serialize>(key: &str, value: &T) {
    let _ = LocalStorage::set(key, value);
}
