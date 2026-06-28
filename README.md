# Flows

面向个人的时间管理桌面应用。支持每日待办、复盘总结、AI 周报自动生成、Markdown 笔记与 Git 同步。

## 下载与安装

最新安装包可以在 GitHub Releases 下载：

- [下载最新版](https://github.com/Yunshiro/flows/releases/latest)
- [查看所有安装包](https://github.com/Yunshiro/flows/releases)

根据你的系统选择对应文件：

| 系统 | 安装包 |
|---|---|
| macOS Apple Silicon | `Flows_*_aarch64.dmg` |
| macOS Intel | `Flows_*_x64.dmg` |
| Windows | `Flows_*.msi` 或 `Flows_*.exe` |

macOS 如果提示“无法验证开发者”，可以在“系统设置 → 隐私与安全性”里允许打开，或右键应用选择“打开”。

## 功能

| 模块 | 说明 |
|---|---|
| 今日待办 | CRUD、拖拽排序、优先级标签、筛选搜索、内联编辑 |
| 每日复盘 | Markdown 编辑器、心情标记、关联已完成待办、时间线浏览 |
| 每周总览 | 自动统计图表、AI 流式生成周报（支持 DeepSeek/OpenAI/Ollama） |
| Markdown 笔记 | 文件树管理、实时预览、Wiki `[[双向链接]]`、全文搜索 |
| Git 同步 | 推送前文件选择、自动同步、多设备数据同步 |
| 多模型配置 | 支持保存多个 AI 配置，随时切换 |

## 技术栈

| 层 | 技术 |
|---|---|
| 桌面框架 | Tauri v2 (Rust) |
| 前端 | Vue 3 + TypeScript + Vite 6 |
| 状态管理 | Pinia |
| 路由 | Vue Router 4 |
| 数据库 | SQLite (rusqlite) |
| Markdown | marked |
| HTTP | reqwest |
| 图标 | 内联 SVG |

## 快速开始

### 环境要求

- Node.js 18+
- Rust 1.70+
- Git（可选，用于笔记同步）

### 安装

```bash
# 克隆项目
git clone <repo-url> flows
cd flows

# 安装前端依赖
npm install

# 安装 Rust 依赖（自动下载）
cd src-tauri && cargo build && cd ..
```

### 开发

```bash
# 浏览器模式（仅前端，数据存 localStorage）
npm run dev

# 桌面模式（完整功能，数据存 SQLite）
npm run tauri dev
```

### 打包

```bash
npm run tauri build
```

产物在 `src-tauri/target/release/bundle/` 目录：
- Windows: `.msi` / `.exe`
- macOS: `.dmg`

### 发布到 GitHub Releases

安装 GitHub CLI 并登录：

```bash
brew install gh
gh auth login
```

创建一个 Release 并上传安装包：

```bash
gh release create v0.1.0 \
  src-tauri/target/release/bundle/dmg/Flows_0.1.0_aarch64.dmg \
  --title "Flows v0.1.0" \
  --notes "首次发布 Flows 桌面应用。"
```

如果 Release 已经存在，只需要继续上传新的安装包：

```bash
gh release upload v0.1.0 \
  src-tauri/target/release/bundle/dmg/Flows_0.1.0_aarch64.dmg \
  --clobber
```

## 项目结构

```
flows/
├── src/                          # 前端源码
│   ├── main.ts                   # 入口
│   ├── App.vue                   # 根组件
│   ├── router/index.ts           # 路由配置
│   ├── stores/                   # Pinia 状态管理
│   │   ├── useTodoStore.ts
│   │   ├── useReviewStore.ts
│   │   ├── useNotesStore.ts
│   │   └── useSettingsStore.ts
│   ├── api/                      # Tauri invoke 封装
│   │   ├── adapter.ts            # Tauri / localStorage 双模适配
│   │   ├── todos.ts
│   │   ├── reviews.ts
│   │   ├── notes.ts
│   │   ├── settings.ts
│   │   └── llm.ts
│   ├── views/                    # 页面组件
│   │   ├── TodosPage.vue
│   │   ├── ReviewPage.vue
│   │   ├── WeeklyPage.vue
│   │   ├── NotesPage.vue
│   │   └── SettingsPage.vue
│   └── components/               # 共享组件
│       ├── AppSidebar.vue
│       ├── AppToast.vue
│       ├── MarkdownEditor.vue
│       ├── TagPicker.vue
│       ├── InlineEdit.vue
│       └── GitPushDialog.vue
├── src-tauri/                    # Rust 后端
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── src/
│       ├── main.rs
│       ├── lib.rs                # Tauri Builder + 命令注册
│       ├── db.rs                 # SQLite 初始化
│       ├── models/               # 数据模型
│       │   ├── todo.rs
│       │   ├── review.rs
│       │   ├── note.rs
│       │   ├── settings.rs
│       │   └── llm_config.rs
│       └── commands/             # Tauri 命令
│           ├── todo_cmd.rs
│           ├── review_cmd.rs
│           ├── weekly_cmd.rs     # AI 周报生成 + 流式输出
│           ├── note_cmd.rs
│           ├── git_cmd.rs
│           ├── llm_config_cmd.rs
│           └── settings_cmd.rs
├── docs/
│   ├── development-plan.html     # 开发计划文档
│   └── dashboard-preview.png     # 预览截图
├── package.json
├── vite.config.ts
└── tsconfig.json
```

## 数据存储

| 类型 | 位置 |
|---|---|
| 待办/复盘/设置 | `~/.flows/flows.db` (SQLite) |
| 笔记 | `~/flows-notes/` (Markdown 文件) |
| LLM 配置 | `flows.db` → `llm_configs` 表 |

## 配置 AI 周报

支持任意 OpenAI 兼容接口，包括 DeepSeek、OpenAI、Ollama 等。

1. 打开设置页 → AI 模型配置 → 添加配置
2. 填写 API 地址、Key、模型名称
3. 点击测试连接验证
4. 在周报页下拉选择配置，点击生成即可流式输出

```
# DeepSeek 示例
API 地址: https://api.deepseek.com/v1
模型名称: deepseek-chat
```

## License

MIT
