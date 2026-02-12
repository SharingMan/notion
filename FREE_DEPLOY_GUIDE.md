# 🚀 免费部署平台指南

除了 Railway，还有很多优秀的**完全免费**的部署平台！

---

## 🥇 推荐方案对比

| 平台 | 免费额度 | 优点 | 缺点 | 难度 |
|------|---------|------|------|------|
| **Cloudflare Pages** | 无限请求 | 全球CDN、完全免费 | 构建时间限制 | ⭐ 简单 |
| **Vercel** | 100GB/月 | 功能强大、生态好 | 国内访问慢 | ⭐⭐ 中等 |
| **Netlify** | 100GB/月 | 简单易用 | 国内访问慢 | ⭐ 简单 |
| **GitHub Pages** | 1GB存储 | 完全免费、无流量限制 | 仅静态、国内慢 | ⭐⭐ 中等 |
| **Render** | 750小时/月 | 支持后端 | 免费版慢 | ⭐⭐ 中等 |

---

## 🥇 方案一：Cloudflare Pages（强烈推荐）

**完全免费，无限流量，全球CDN加速！**

### 部署步骤

1. **访问 Cloudflare**
   - 打开 https://dash.cloudflare.com/
   - 注册/登录账号

2. **创建 Pages 项目**
   - 点击左侧菜单 "Pages"
   - 点击 "Create a project"
   - 选择 "Connect to Git"

3. **连接 GitHub**
   - 授权 Cloudflare 访问 GitHub
   - 选择 `SharingMan/notion` 仓库
   - 点击 "Begin setup"

4. **配置构建设置**

   | 设置项 | 值 |
   |--------|-----|
   | **Project name** | notion-cafe |
   | **Production branch** | main |
   | **Build command** | `curl https://sh.rustup.rs -sSf \| sh -s -- -y && source $HOME/.cargo/env && cargo install trunk && rustup target add wasm32-unknown-unknown && trunk build --release` |
   | **Build output directory** | `dist` |

5. **添加环境变量（可选）**
   - 点击 "Environment variables (advanced)"
   - 添加：`NODE_VERSION` = `18`

6. **保存并部署**
   - 点击 "Save and Deploy"
   - 等待构建完成（约 5-10 分钟）

7. **访问网站**
   - Cloudflare 会分配一个域名：`notion-cafe.pages.dev`
   - 你也可以绑定自定义域名

### 自定义域名（可选）

1. 在 Pages 项目设置中找到 "Custom domains"
2. 点击 "Set up a custom domain"
3. 输入你的域名（如 `notion.yourdomain.com`）
4. 按提示添加 DNS 记录

---

## 🥈 方案二：Vercel

**前端开发首选，功能强大**

### 方式 A：网页部署（推荐）

1. 访问 https://vercel.com/
2. 用 GitHub 登录
3. 点击 "Add New..." → "Project"
4. 导入 `SharingMan/notion` 仓库
5. 配置：
   - **Framework Preset**: Other
   - **Build Command**: `curl https://sh.rustup.rs -sSf \| sh -s -- -y && source $HOME/.cargo/env && cargo install trunk && rustup target add wasm32-unknown-unknown && trunk build --release`
   - **Output Directory**: `dist`
6. 点击 "Deploy"

### 方式 B：Vercel CLI

```bash
# 安装 Vercel CLI
npm i -g vercel

# 登录
vercel login

# 部署
cd "/Users/jiyingshe/Desktop/AI学习/21-notioncafe"
vercel --prod
```

---

## 🥉 方案三：Netlify

**极简部署体验**

1. 访问 https://www.netlify.com/
2. 用 GitHub 登录
3. 点击 "Add new site" → "Import an existing project"
4. 选择 GitHub → `SharingMan/notion`
5. 构建设置已配置在 `netlify.toml` 中，无需修改
6. 点击 "Deploy site"

或者拖放部署（最简单）：
```bash
# 本地构建
cd "/Users/jiyingshe/Desktop/AI学习/21-notioncafe"
trunk build --release

# 然后访问 https://app.netlify.com/drop
# 拖放 dist 文件夹即可！
```

---

## 🏅 方案四：GitHub Pages

**完全免费，无需第三方平台**

### 启用 GitHub Pages

1. 打开仓库设置：https://github.com/SharingMan/notion/settings/pages
2. **Source** 选择 "GitHub Actions"
3. GitHub Actions 会自动部署（已配置 `.github/workflows/pages.yml`）

### 访问地址

部署完成后访问：
```
https://sharingman.github.io/notion/
```

> ⚠️ 注意：GitHub Pages 默认域名在国内访问较慢，建议绑定自定义域名。

---

## 🏅 方案五：Render

**支持后端服务的免费平台**

1. 访问 https://dashboard.render.com/
2. 用 GitHub 登录
3. 点击 "New" → "Static Site"
4. 选择 `SharingMan/notion` 仓库
5. 配置：
   - **Name**: notion-cafe
   - **Build Command**: `curl https://sh.rustup.rs -sSf \| sh -s -- -y && source $HOME/.cargo/env && cargo install trunk && rustup target add wasm32-unknown-unknown && trunk build --release`
   - **Publish Directory**: `dist`
6. 点击 "Create Static Site"

---

## 📊 平台选择建议

### 如果你...

| 需求 | 推荐平台 | 原因 |
|------|---------|------|
| 想要最简单 | **Cloudflare Pages** | 一键部署，全球加速 |
| 想要最快 | **Cloudflare Pages** | 全球 CDN，国内可访问 |
| 不想注册新账号 | **GitHub Pages** | 用现有 GitHub 账号 |
| 需要预览部署 | **Vercel** | 每个 PR 自动生成预览链接 |
| 需要团队协作 | **Netlify** | 审核工作流完善 |

---

## ⚠️ 重要：CORS 问题

所有平台都会遇到 Notion API 的 CORS 限制！解决方案：

### 方案 1：使用代理（推荐）

创建一个 Cloudflare Worker 作为代理：

```javascript
// worker.js
export default {
  async fetch(request, env) {
    const url = new URL(request.url);

    if (url.pathname.startsWith('/api/notion/')) {
      const notionPath = url.pathname.replace('/api/notion/', '');
      const notionUrl = `https://api.notion.com/v1/${notionPath}`;

      const response = await fetch(notionUrl, {
        method: request.method,
        headers: {
          'Authorization': request.headers.get('Authorization'),
          'Notion-Version': '2022-06-28',
          'Content-Type': 'application/json',
        },
        body: request.body,
      });

      return new Response(response.body, {
        status: response.status,
        headers: {
          'Access-Control-Allow-Origin': '*',
          'Access-Control-Allow-Methods': 'GET, POST, PATCH, DELETE, OPTIONS',
          'Access-Control-Allow-Headers': 'Content-Type, Authorization, Notion-Version',
        },
      });
    }

    return env.ASSETS.fetch(request);
  },
};
```

### 方案 2：使用第三方代理

修改代码中的 API 地址为：
```rust
const NOTION_API_BASE: &str = "https://your-proxy.workers.dev/api/notion";
```

---

## 🚀 快速开始清单

选择 **Cloudflare Pages**（最简单）：

- [ ] 注册 Cloudflare 账号
- [ ] 打开 dash.cloudflare.com
- [ ] 点击 Pages → Create a project
- [ ] 连接 GitHub 仓库
- [ ] 配置构建命令（已提供）
- [ ] 点击 Deploy
- [ ] 访问生成的域名
- [ ] 配置 Notion API Key

---

## 📞 需要帮助？

如果在部署过程中遇到问题，可以：

1. 查看平台官方文档
2. 检查构建日志
3. 在 GitHub Issues 中提问

祝部署顺利！☕️
