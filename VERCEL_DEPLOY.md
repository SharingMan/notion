# Vercel 部署指南

## 🚀 方式一：Vercel 官网部署（推荐，最简单）

### 步骤 1：准备
- 确保代码已推送到 GitHub: `https://github.com/SharingMan/notion`
- 注册/登录 [Vercel](https://vercel.com)（可用 GitHub 账号直接登录）

### 步骤 2：导入项目
1. 访问 [Vercel Dashboard](https://vercel.com/dashboard)
2. 点击 **"Add New..."** → **"Project"**
3. 在 "Import Git Repository" 中找到 `SharingMan/notion`
4. 点击 **"Import"**

### 步骤 3：配置构建设置

Vercel 会自动检测配置，请确认以下设置：

| 设置项 | 值 |
|--------|-----|
| **Framework Preset** | `Other` |
| **Root Directory** | `./` (默认) |
| **Build Command** | `curl https://sh.rustup.rs -sSf \| sh -s -- -y && source $HOME/.cargo/env && cargo install trunk && rustup target add wasm32-unknown-unknown && trunk build --release` |
| **Output Directory** | `dist` |
| **Install Command** | `echo "No npm install needed"` |

> 💡 **提示**: 如果 Build Command 太长，可以使用简化版：
> ```bash
> sh vercel-build.sh
> ```

### 步骤 4：部署
1. 点击 **"Deploy"**
2. 等待构建完成（约 5-8 分钟）
3. 部署成功后，Vercel 会分配一个域名，如：`notion-cafe.vercel.app`

### 步骤 5：自定义域名（可选）
1. 在项目页面点击 **"Settings"** → **"Domains"**
2. 输入你的域名，如 `notion.yourdomain.com`
3. 按提示添加 DNS 记录

---

## 🖥️ 方式二：Vercel CLI 部署

### 1. 安装 Vercel CLI
```bash
npm i -g vercel
# 或
pnpm i -g vercel
# 或
yarn global add vercel
```

### 2. 登录
```bash
vercel login
```
浏览器会弹出登录页面，选择 GitHub 登录。

### 3. 链接项目
```bash
cd "/Users/jiyingshe/Desktop/AI学习/21-notioncafe"
vercel
```
按提示操作：
- ? Set up and deploy "~/notion"? **Yes**
- ? Which scope do you want to deploy to? **你的用户名**
- ? Link to existing project? **No**
- ? What's your project name? **notion-cafe**
- ? In which directory is your code located? **./** (当前目录)

### 4. 配置环境变量（如需要）
```bash
vercel env add NOTION_API_KEY
```

### 5. 部署
```bash
# 开发预览（每次修改后自动更新）
vercel

# 生产部署
vercel --prod
```

---

## ⚠️ 重要：CORS 问题解决方案

由于浏览器安全限制，直接调用 Notion API 会遇到 **CORS 错误**！

### 解决方案：使用 Vercel Edge Function 代理

Vercel 支持免费的 Edge Functions，我们可以创建一个代理来解决 CORS：

#### 步骤 1：创建 API 目录

```bash
mkdir -p api
```

#### 步骤 2：创建代理函数

创建文件 `api/notion.js`：

```javascript
export default async function handler(request) {
  const url = new URL(request.url);
  const path = url.pathname.replace('/api/notion/', '');

  const notionUrl = `https://api.notion.com/v1/${path}${url.search}`;

  try {
    const response = await fetch(notionUrl, {
      method: request.method,
      headers: {
        'Authorization': request.headers.get('Authorization'),
        'Notion-Version': '2022-06-28',
        'Content-Type': 'application/json',
      },
      body: request.method !== 'GET' && request.method !== 'HEAD'
        ? request.body
        : undefined,
    });

    const data = await response.text();

    return new Response(data, {
      status: response.status,
      headers: {
        'Content-Type': 'application/json',
        'Access-Control-Allow-Origin': '*',
        'Access-Control-Allow-Methods': 'GET, POST, PATCH, DELETE, OPTIONS',
        'Access-Control-Allow-Headers': 'Authorization, Notion-Version, Content-Type',
      },
    });
  } catch (error) {
    return new Response(JSON.stringify({ error: error.message }), {
      status: 500,
      headers: {
        'Content-Type': 'application/json',
        'Access-Control-Allow-Origin': '*',
      },
    });
  }
}

export const config = {
  runtime: 'edge',
};
```

#### 步骤 3：修改前端代码使用代理

编辑 `src/api/mod.rs`，修改 API base URL：

```rust
// 开发环境使用直接 API
#[cfg(debug_assertions)]
const NOTION_API_BASE: &str = "https://api.notion.com/v1";

// 生产环境使用 Vercel 代理
#[cfg(not(debug_assertions))]
const NOTION_API_BASE: &str = "/api/notion";
```

或者更简单，添加一个配置选项让用户选择：

在 `src/types/mod.rs` 的 `AppState` 中添加：
```rust
pub struct AppState {
    // ... 其他字段
    pub use_proxy: bool,
    pub proxy_url: String,
}
```

#### 步骤 4：重新部署

```bash
git add .
git commit -m "Add Vercel Edge Function proxy for Notion API"
git push
```

Vercel 会自动重新部署。

---

## 🔧 进阶配置

### 环境变量

在 Vercel Dashboard → Project Settings → Environment Variables 中添加：

| 变量名 | 说明 | 示例 |
|--------|------|------|
| `NOTION_API_KEY` | Notion API Key | `secret_xxx` |
| `NOTION_DATABASE_ID` | 默认数据库 ID | `xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx` |

> ⚠️ 注意：这些变量只在服务端（Edge Functions）可用，前端代码无法直接访问！

### 自定义构建脚本

如果 Build Command 太长，创建 `vercel-build.sh`：

```bash
#!/bin/bash
set -e

# 安装 Rust
curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env

# 安装 Trunk
cargo install trunk --locked

# 添加 WASM 目标
rustup target add wasm32-unknown-unknown

# 构建
trunk build --release

echo "Build complete!"
```

然后修改 Vercel 的 Build Command 为：`sh vercel-build.sh`

### 缓存优化

创建 `vercel.json`（已创建）：

```json
{
  "headers": [
    {
      "source": "/(.*).wasm",
      "headers": [
        {
          "key": "Content-Type",
          "value": "application/wasm"
        },
        {
          "key": "Cache-Control",
          "value": "public, max-age=31536000, immutable"
        }
      ]
    }
  ]
}
```

---

## 📊 Vercel 免费额度

| 资源 | 免费额度 |
|------|---------|
| **带宽** | 100GB/月 |
| **构建时间** | 6000分钟/月 |
| **Serverless Functions** | 100GB-Hrs/月 |
| **Edge Functions** | 100万次调用/月 |

对于个人项目完全够用！

---

## 🐛 常见问题

### 1. 构建失败：Rust 未找到
确保 Build Command 中包含了 Rust 安装：
```bash
curl https://sh.rustup.rs -sSf | sh -s -- -y && source $HOME/.cargo/env
```

### 2. 构建超时（>45分钟）
Vercel 免费版构建限制 45 分钟。解决方案：
- 本地构建后上传 `dist` 文件夹
- 或使用 GitHub Actions 预编译

### 3. WASM 文件 MIME 类型错误
确保 `vercel.json` 中配置了正确的 Content-Type。

### 4. 刷新页面 404
SPA 路由问题，已配置 `vercel.json` 重写规则。

---

## ✅ 部署清单

- [ ] GitHub 仓库已更新
- [ ] Vercel 项目已创建
- [ ] Build Command 配置正确
- [ ] 首次部署成功
- [ ] 配置了 CORS 代理（如需要）
- [ ] 测试了 Notion API 连接
- [ ] 绑定了自定义域名（可选）

---

## 📚 相关链接

- [Vercel Dashboard](https://vercel.com/dashboard)
- [Vercel CLI 文档](https://vercel.com/docs/cli)
- [Vercel Edge Functions](https://vercel.com/docs/concepts/functions/edge-functions)
- [Notion API 文档](https://developers.notion.com/)

---

遇到问题？查看 [Vercel 官方文档](https://vercel.com/docs) 或在 GitHub Issues 提问！
