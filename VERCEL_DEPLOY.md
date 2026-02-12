# Vercel 部署指南

## 🚀 部署步骤

### 1. 准备
- 代码已推送到 GitHub: `https://github.com/SharingMan/notion`
- 注册/登录 [Vercel](https://vercel.com)（用 GitHub 登录）

### 2. 创建项目
1. 访问 [Vercel Dashboard](https://vercel.com/dashboard)
2. 点击 **"Add New..."** → **"Project"**
3. 找到 `SharingMan/notion` 仓库，点击 **"Import"**

### 3. 配置构建设置 ⚙️

**重要：在 Configure Project 页面，修改以下设置：**

| 设置项 | 值 |
|--------|-----|
| **Framework Preset** | `Other` |
| **Root Directory** | `./` (默认) |
| **Build Command** | `sh vercel-build.sh` |
| **Output Directory** | `dist` |
| **Install Command** | `echo "No npm install needed"` |

**然后点击 Environment Variables，添加：**
- 无需添加，所有配置都在前端本地存储

### 4. 部署
1. 点击 **"Deploy"**
2. 等待构建完成（约 5-10 分钟，首次较慢）
3. 部署成功后获得域名：`notion-cafe.vercel.app`

### 5. 解决 CORS 问题（关键！）

Vercel 部署后会遇到 Notion API CORS 错误，需要配置代理：

#### 方法 A：Vercel CLI 重新配置（推荐）

安装 Vercel CLI 后，API 路由会自动工作：

```bash
npm i -g vercel
vercel login

# 链接项目
cd "/Users/jiyingshe/Desktop/AI学习/21-notioncafe"
vercel

# 部署
vercel --prod
```

#### 方法 B：Vercel 面板配置（如果方法 A 不行）

1. 在项目 Settings → Functions 中，确保 API 路由已启用
2. 访问 `https://你的域名/api/notion/databases` 测试代理是否工作

#### 方法 C：使用 GitHub Actions 预构建（最稳定）

查看 `.github/workflows/vercel.yml`，配置自动部署。

---

## 🔧 常见问题

### 问题 1：Build Failed "Command failed"
这是正常的，因为构建时间较长。查看详细日志，等待 5-10 分钟。

### 问题 2："Runtimes" 错误
这是因为 Vercel 自动检测框架失败。确保：
1. Framework Preset 选择 `Other`
2. Build Command 填写 `sh vercel-build.sh`

### 问题 3：CORS 错误（部署后无法同步 Notion）
这是预期行为，Notion API 不支持浏览器直接调用。解决方案：

**方案 1：使用 Vercel Edge Function（已配置）**
- 代码在 `api/notion.js`
- 部署后自动生效

**方案 2：使用第三方 CORS 代理**
在应用设置中修改 API 端点：
```
https://cors-anywhere.herokuapp.com/https://api.notion.com/v1
```

**方案 3：本地开发模式**
在本地使用 `trunk serve`，然后导出数据到 Notion。

### 问题 4：WASM 文件 404
确保 `Output Directory` 是 `dist`，不是 `dist/` 或其他。

---

## 📊 Vercel 免费额度

| 资源 | 免费额度 |
|------|---------|
| **带宽** | 100GB/月 |
| **构建时间** | 6000分钟/月 |
| **Serverless Functions** | 100GB-Hrs/月 |

对于个人日历应用完全够用！

---

## 🎯 简化部署方案（如果上述失败）

如果自动构建总是失败，使用本地构建 + 拖拽部署：

```bash
# 本地构建
cd "/Users/jiyingshe/Desktop/AI学习/21-notioncafe"
trunk build --release

# 然后访问 https://vercel.com/new
# 选择 "Import Git Repository" 旁的 "Continue with Template"
# 或者直接访问 https://vercel.com/drag-and-drop
# 拖拽 dist 文件夹
```

---

## 📚 相关链接

- [Vercel Dashboard](https://vercel.com/dashboard)
- [Vercel CLI 文档](https://vercel.com/docs/cli)
- [GitHub 仓库](https://github.com/SharingMan/notion)

---

**仍然遇到问题？** 告诉我具体的错误信息，我帮你解决！
