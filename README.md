# 我的工具(mytools)

一个用 **Rust + Dioxus** 写的个人小工具集合,做成**网页应用**,在 iPhone 上"添加到主屏幕"后就像原生 app 一样用。同一份代码,电脑浏览器里也能开。

为什么是这个形态:你主力机是 iPhone,而把自制原生 app 装进自己 iPhone 要 Mac + Xcode + 签名、还得每周重签——很烦。网页应用 + 加到主屏,完全绕开这套折磨,迭代也快。

> ⚠️ 说明:这个骨架是在一个没有 Rust 工具链的环境里生成的,**没能在生成时编译验证**。代码是严格对照 Dioxus 0.7.9 官方示例写的,但你第一次 `dx serve` 时如果遇到个别 API 微调,属正常,照编译器提示改即可。

---

## 一、装工具链(一次性)

```bash
# 1) 装 Rust(rustup)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# 2) 加 WebAssembly 目标(网页应用要编译到 wasm)
rustup target add wasm32-unknown-unknown

# 3) 装 Dioxus 命令行工具 dx(与本项目同版本)
cargo install dioxus-cli --version 0.7.9
```

## 二、跑起来(开发)

在项目根目录:

```bash
dx serve
```

它会编译并起一个本地服务器,终端会打印访问地址(通常是 `http://localhost:8080`)。改代码自动热重载。

## 三、构建 + 部署

```bash
dx build --release
```

产物在 `dist/`,是一堆纯静态文件。任何静态托管都能放:自己的小服务器、Cloudflare Pages、GitHub Pages、或局域网里 `python3 -m http.server` 临时跑一下。

> **PWA / Service Worker 需要 HTTPS**(或 localhost)才生效。所以正式用的话,把 `dist/` 放到一个有 HTTPS 的地址。

## 四、装到 iPhone

1. 用 **Safari** 打开你部署好的网址(必须是 Safari,不能是 Chrome)。
2. 点底部分享按钮 → **添加到主屏幕**。
3. 桌面就会出现图标,点开是全屏、无浏览器地址栏的样子,跟 app 一样。

---

## 五、加一个新工具(三步)

整个架构就是为这件事设计的。以加"灵感/笔记"工具为例:

**第 1 步**:复制 `src/tools/todo.rs` 为 `src/tools/notes.rs`,改成你要的数据结构和界面,把入口组件命名为 `pub fn NotesView() -> Element`,并把存储 key 改成 `"mytools:notes"`。

**第 2 步**:在 `src/tools/mod.rs` 里
- 取消注释(或新增) `pub mod notes;`
- 在 `registry()` 里加一行:`ToolMeta { id: "notes", name: "灵感", icon: "💡" },`

**第 3 步**:在 `src/main.rs` 的 `ToolOutlet` 里加一个 match 分支:
```rust
"notes" => rsx! { tools::notes::NotesView {} },
```

完成。底部标签栏会自动多一个入口,各工具数据互不干扰。

---

## 六、加上跨设备同步(下一步)

现在数据存在每台设备的本地。要跨设备同步,推荐这套最省事的组合:

### 后端:PocketBase
[PocketBase](https://pocketbase.io) 是**一个单独的可执行文件**,自带数据库 + REST 接口 + 用户认证,自托管极简单:

```bash
./pocketbase serve
```

在它的管理后台为每个工具建一个 collection(如 `todo`),字段对应你的数据结构,再加一个 `updated` 时间字段。

### 前端:填好 `src/sync.rs`
那里已经写好了设计和接口占位。补充思路:
- 加一个 HTTP 客户端依赖(WASM 下推荐 `gloo-net`:`gloo-net = "0.6"`)。
- `pull()`:`GET {你的地址}/api/collections/todo/records`,把每条记录按 `updated` 时间和本地比较,新的覆盖旧的。
- `push()`:本地比后端新的记录,新增用 `POST`、改动用 `PATCH`。
- 时机:应用启动时 `pull()` 一次;本地每次改动后 `push()`。

### 冲突怎么办
你是单用户多设备,几乎不会在两台设备同一秒改同一条。所以用最朴素的**"最后写入者获胜"**(谁的 `updated` 新用谁)就够了,**不需要 CRDT**。

### 为什么要后端,而不是只靠本地文件同步
iPhone 上网页应用的本地存储**不保证长期可靠**(系统可能在长期不用后回收)。所以让后端当"真相来源"、本地当快取,是这条路线最稳的形态。这也顺带绕开了 Syncthing 在 iOS 上不好使的问题。

---

## 目录结构

```
mytools/
├─ Cargo.toml          依赖
├─ Dioxus.toml         构建配置(web 平台、静态资源目录)
├─ index.html          页面外壳 + SW 注册 + iPhone 相关 meta
├─ assets/
│  └─ main.css         样式(移动端优先,带 iPhone 安全区)
├─ public/             静态资源(原样发布到根目录)
│  ├─ manifest.json    PWA 清单
│  ├─ sw.js            Service Worker(离线缓存)
│  └─ icon-192/512.png 应用图标
└─ src/
   ├─ main.rs          外壳:标签栏 + 工具分发
   ├─ store.rs         本地存储封装
   ├─ sync.rs          同步层(占位骨架)
   └─ tools/
      ├─ mod.rs        工具注册表
      └─ todo.rs       待办工具(也是新工具的模板)
```

## 小提醒(路径)
`index.html` 里把 `manifest.json` / `sw.js` / 图标都按根路径 `/xxx` 引用,对应 `Dioxus.toml` 里 `asset_dir = "public"`。如果 `dx serve` 后浏览器控制台报这几个文件 404,看一下 `dist/` 里它们实际落在根目录还是 `/assets/` 下,把 `index.html` 里的路径对齐即可。
