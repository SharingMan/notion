# Railway 部署指南

## 方式一：通过 Railway 面板（推荐）

### 1. 访问 Railway 网站
- 打开 https://railway.app/
- 使用 GitHub 账号登录

### 2. 创建新项目
1. 点击 "New Project"
2. 选择 "Deploy from GitHub repo"
3. 搜索并选择 `SharingMan/notion` 仓库
4. Railway 会自动检测 Dockerfile 并开始构建

### 3. 等待部署完成
- 构建过程大约 5-10 分钟
- 可以在 Deployments 标签页查看进度

### 4. 生成域名
1. 部署成功后，点击 "Settings" 标签
2. 找到 "Public Networking" 部分
3. 点击 "Generate Domain"
4. 复制生成的域名（如 `notion-cafe.up.railway.app`）

### 5. 完成！
访问生成的域名即可使用你的 Notion Cafe 应用。

---

## 方式二：通过 Railway CLI

### 1. 安装 Railway CLI
```bash
npm install -g @railway/cli
```

### 2. 登录 Railway
```bash
railway login
```
浏览器会弹出登录页面，使用 GitHub 账号登录。

### 3. 链接项目
```bash
cd "/Users/jiyingshe/Desktop/AI学习/21-notioncafe"
railway link
```
选择 "Create a new project" 或选择现有项目。

### 4. 部署
```bash
railway up
```

### 5. 生成域名
```bash
railway domain
```

---

## 方式三：GitHub Actions 自动部署

### 1. 获取 Railway Token
1. 访问 https://railway.app/account/tokens
2. 点击 "New Token"
3. 名称：notion-cafe-token
4. 复制生成的 Token

### 2. 设置 GitHub Secrets
1. 打开 https://github.com/SharingMan/notion/settings/secrets/actions
2. 点击 "New repository secret"
3. 名称：`RAILWAY_TOKEN`
4. 值：粘贴刚才复制的 Token
5. 点击 "Add secret"

### 3. 触发自动部署
每次推送到 main 分支时，GitHub Actions 会自动部署：
```bash
git push origin main
```

---

## 🔔 重要提示

### CORS 问题
由于 Notion API 的 CORS 限制，部署后可能会遇到 API 调用失败。解决方法：

1. **使用代理**（推荐）
   - 在 Railway 中创建一个额外的服务作为 API 代理
   - 或者使用 Cloudflare Workers 作为代理

2. **本地开发模式**
   - 开发时使用 `trunk serve`
   - 生产环境需要配置代理

### 环境变量（可选）
如果需要通过环境变量配置 Notion API Key：

1. 在 Railway 面板点击 "Variables" 标签
2. 添加变量：
   - `NOTION_API_KEY`: your-secret-key
   - `NOTION_DATABASE_ID`: your-database-id
3. 修改代码读取环境变量

---

## 📋 故障排除

### 构建失败
```bash
# 查看日志
railway logs
```

### 检查服务状态
```bash
railway status
```

### 重新部署
```bash
railway up --detach
```

### 访问部署的控制台
```bash
railway connect
```

---

## ✅ 部署清单

- [ ] GitHub 仓库已创建
- [ ] 代码已推送到 GitHub
- [ ] Railway 项目已创建
- [ ] 连接到 GitHub 仓库
- [ ] 首次部署成功
- [ ] 已生成公开域名
- [ ] 配置了 Notion API 连接
- [ ] 测试了事件同步功能

---

## 🔗 相关链接

- Railway Dashboard: https://railway.app/dashboard
- GitHub 仓库: https://github.com/SharingMan/notion
- Notion API 文档: https://developers.notion.com/
