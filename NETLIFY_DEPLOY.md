# Netlify 部署指南

## 🚀 方式一：拖拽部署（最简单，推荐）

### 步骤 1：本地构建

在你的电脑上：

```bash
cd "/Users/jiyingshe/Desktop/AI学习/21-notioncafe"

# 安装 trunk（如果还没装）
cargo install trunk

# 构建生产版本
trunk build --release
```

### 步骤 2：拖拽部署

1. 访问 **https://app.netlify.com/drop**
2. 把 `dist` 文件夹**拖到网页上**
3. 等待上传完成（约 10 秒）
4. 获得随机域名，如 `https://abundant-cactus-123456.netlify.app`
5. 点击域名查看网站！🎉

### 步骤 3：自定义域名（可选）

1. 在 Netlify 面板点击 "Site settings"
2. 选择 "Domain management"
3. 点击 "Add custom domain"
4. 输入你的域名，按提示配置 DNS

---

## 🚀 方式二：Git 自动部署

### 步骤 1：授权 Netlify 访问 GitHub

1. 访问 **https://app.netlify.com/**
2. 点击 "Add new site" → "Import an existing project"
3. 选择 "GitHub" 授权
4. 找到 `SharingMan/notion` 仓库

### 步骤 2：配置构建设置

Netlify 会自动读取 `netlify.toml` 配置，但请确认：

| 设置项 | 值 |
|--------|-----|
| **Build command** | `curl https://sh.rustup.rs -sSf \| sh -s -- -y && source $HOME/.cargo/env && cargo install trunk && rustup target add wasm32-unknown-unknown && trunk build --release` |
| **Publish directory** | `dist` |

### 步骤 3：部署

点击 "Deploy site"，等待 5-10 分钟构建完成。

---

## 🚀 方式三：Netlify CLI

```bash
# 安装 Netlify CLI
npm install -g netlify-cli

# 登录
netlify login

# 初始化项目
cd "/Users/jiyingshe/Desktop/AI学习/21-notioncafe"
netlify init

# 部署
netlify deploy --prod --dir=dist
```

---

## 🔧 CORS 问题解决方案

和 Vercel 一样，Notion API 会有 CORS 问题。解决方案：

### 方案 1：使用 Netlify Functions（已配置）

`netlify.toml` 已包含 Functions 配置。创建 `netlify/functions/notion.js`：

```javascript
exports.handler = async (event, context) => {
  const path = event.path.replace('/.netlify/functions/notion/', '');
  const notionUrl = `https://api.notion.com/v1/${path}`;

  try {
    const response = await fetch(notionUrl, {
      method: event.httpMethod,
      headers: {
        'Authorization': event.headers.authorization || '',
        'Notion-Version': '2022-06-28',
        'Content-Type': 'application/json',
      },
      body: event.body,
    });

    const data = await response.text();

    return {
      statusCode: response.status,
      headers: {
        'Access-Control-Allow-Origin': '*',
        'Content-Type': 'application/json',
      },
      body: data,
    };
  } catch (error) {
    return {
      statusCode: 500,
      body: JSON.stringify({ error: error.message }),
    };
  }
};
```

### 方案 2：使用第三方代理

在应用设置中输入代理地址，如：
- `https://cors-anywhere.herokuapp.com/https://api.notion.com/v1`

### 方案 3：本地使用（最稳定）

在本地开发使用，数据存储在本地：
```bash
trunk serve --open
```

---

## 📊 Netlify 免费额度

| 资源 | 免费额度 |
|------|---------|
| **带宽** | 100GB/月 |
| **构建时间** | 300分钟/月 |
| **存储** | 无限（Git repo） |
| **Serverless Functions** | 125,000次/月 |

对于个人项目完全够用！

---

## ✅ 推荐方案总结

| 场景 | 推荐方式 |
|------|---------|
| **最快部署** | 方式一：拖拽部署 |
| **自动更新** | 方式二：Git 集成 |
| **命令行爱好者** | 方式三：CLI |

---

## 🆚 Netlify vs Vercel

| 特性 | Netlify | Vercel |
|------|---------|--------|
| 拖拽部署 | ✅ 支持 | ❌ 不支持 |
| 构建速度 | ⚡ 快 | ⚡ 快 |
| 国内访问 | 🟡 一般 | 🟡 一般 |
| Rust 支持 | ✅ 良好 | ✅ 良好 |
| 免费额度 | 100GB | 100GB |

Netlify 的拖拽部署更简单可靠！

---

## 📚 相关链接

- [Netlify Dashboard](https://app.netlify.com/)
- [Netlify Drop](https://app.netlify.com/drop)
- [Netlify CLI 文档](https://docs.netlify.com/cli/get-started/)

---

**推荐先用方式一（拖拽部署）试试，最快看到效果！** 🎉
