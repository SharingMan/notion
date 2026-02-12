# ☕️ Notion Cafe ⤫ Calendar 🗓️

一个精美的 Notion 日历集成应用，基于 Rust + WebAssembly 构建。

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)
![Yew](https://img.shields.io/badge/yew-0.21-green.svg)

## ✨ 功能特性

- 📅 **多视图日历** - 支持月视图、周视图、日视图
- 🔄 **Notion 同步** - 双向同步事件（读取/创建/更新/删除）
- 🗄️ **多数据库支持** - 同时管理多个 Notion 数据库
- 💾 **本地存储** - 配置自动保存到浏览器
- 🎨 **精美 UI** - 渐变主题，毛玻璃效果
- 📱 **响应式设计** - 支持桌面和移动设备

## 🛠️ 技术栈

- **Rust** - 系统编程语言
- **Yew** - Rust 前端框架 (React-like)
- **WebAssembly** - 高性能浏览器执行
- **Trunk** - Rust WASM 构建工具
- **Notion API** - 数据同步

## 🚀 快速开始

### 本地开发

```bash
# 克隆仓库
git clone https://github.com/yourusername/notion-cafe.git
cd notion-cafe

# 安装依赖
cargo install trunk
rustup target add wasm32-unknown-unknown

# 启动开发服务器
trunk serve --open
```

访问 http://localhost:8080

### 构建生产版本

```bash
trunk build --release
```

构建产物位于 `dist/` 目录。

## 🌐 部署到 Railway

### 方法一：通过 Railway CLI（推荐）

1. **安装 Railway CLI**
   ```bash
   npm install -g @railway/cli
   ```

2. **登录 Railway**
   ```bash
   railway login
   ```

3. **初始化项目**
   ```bash
   railway init
   # 选择 "Empty Project"
   # 项目名：notion-cafe
   ```

4. **部署**
   ```bash
   railway up
   ```

5. **生成域名**
   ```bash
   railway domain
   ```

### 方法二：通过 GitHub 集成（自动部署）

1. **创建 GitHub 仓库**
   ```bash
   git init
   git add .
   git commit -m "Initial commit"
   git branch -M main
   git remote add origin https://github.com/yourusername/notion-cafe.git
   git push -u origin main
   ```

2. **在 Railway 中连接 GitHub**
   - 登录 [Railway Dashboard](https://railway.app/dashboard)
   - 点击 "New Project" → "Deploy from GitHub repo"
   - 选择你的仓库
   - Railway 会自动检测 `Dockerfile` 并部署

3. **设置自动部署**
   - 在 Railway 项目设置中启用 "Auto Deploy"
   - 每次推送到 `main` 分支会自动重新部署

### 方法三：GitHub Actions 自动部署

1. **获取 Railway Token**
   - 访问 [Railway Tokens](https://railway.app/account/tokens)
   - 创建新 Token：`notion-cafe-token`
   - 复制 Token 值

2. **设置 GitHub Secrets**
   - 进入 GitHub 仓库 → Settings → Secrets and variables → Actions
   - 添加 `RAILWAY_TOKEN`，值为上一步复制的 Token

3. **推送代码触发部署**
   ```bash
   git push origin main
   ```
   GitHub Actions 会自动构建并部署到 Railway

## 🔑 Notion 配置指南

### 1. 创建 Notion Integration

1. 访问 [Notion Integrations](https://www.notion.so/my-integrations)
2. 点击 "New integration"
3. 填写信息：
   - **Name**: Notion Cafe
   - **Associated workspace**: 你的工作区
4. 点击 "Submit"
5. 复制 `secret_xxx` 开头的 **Internal Integration Token**

### 2. 准备 Notion 数据库

1. 在 Notion 中创建一个新数据库
2. 添加以下属性：
   - **Name** (Title) - 事件标题
   - **Date** (Date) - 事件日期
3. 点击右上角 "..." → "Add connections"
4. 选择 "Notion Cafe" Integration

### 3. 获取 Database ID

从数据库页面 URL 中提取：

```
https://www.notion.so/workspace/xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx?v=...
                └────────────────────────────────┘
                         32 位 ID
```

### 4. 配置应用

1. 打开部署的应用
2. 点击右上角 ⚙️ 设置按钮
3. 粘贴 API Key 和 Database ID
4. 点击 "保存设置"

## 📁 项目结构

```
notion-cafe/
├── Cargo.toml              # Rust 依赖
├── index.html              # HTML 入口
├── Trunk.toml             # Trunk 配置
├── Dockerfile             # Railway 部署配置
├── nginx.conf             # Nginx 配置
├── railway.toml           # Railway 配置
├── src/
│   ├── main.rs            # 应用入口
│   ├── styles.css         # 全局样式
│   ├── api/               # Notion API 客户端
│   ├── components/        # UI 组件
│   ├── types/             # 数据类型
│   └── utils/             # 工具函数
└── .github/workflows/     # GitHub Actions
```

## 🔧 自定义配置

### 修改主题颜色

编辑 `src/utils/mod.rs` 中的 `generate_color` 函数：

```rust
pub fn generate_color(index: usize) -> String {
    let colors = vec![
        "#667eea", // 修改为你喜欢的颜色
        "#f093fb",
        // ...
    ];
    colors[index % colors.len()].to_string()
}
```

### 添加新视图

编辑 `src/types/mod.rs`：

```rust
pub enum ViewMode {
    Month,
    Week,
    Day,
    Year, // 新增
}
```

## 🐛 常见问题

### 1. Notion API 返回 401

- 检查 API Key 是否正确（以 `secret_` 开头）
- 确认 Integration 已连接到数据库

### 2. 数据库 ID 格式错误

- 确保是 32 位十六进制字符串
- 移除 URL 中的连字符

### 3. 部署后页面空白

- 检查浏览器控制台错误
- 确认 WASM 文件已正确加载
- 验证 `nginx.conf` 配置

### 4. CORS 错误

Notion API 不支持浏览器直接调用时会遇到 CORS 问题。解决方案：
- 使用 Railway 的代理服务
- 或设置自定义 API 代理

## 📄 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

## 🙏 致谢

- [Yew](https://yew.rs/) - Rust 前端框架
- [Notion API](https://developers.notion.com/) - 数据源
- [Railway](https://railway.app/) - 托管平台

---

Made with ☕️ by [Shinkai](https://github.com/yourusername)
