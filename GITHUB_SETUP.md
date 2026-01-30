# GitHub 发布检查清单

完成以下步骤将 Bar Tomato 发布到 GitHub：

## 1️⃣ 本地准备

- [x] 项目功能完整
- [x] README.md 已编写
- [x] LICENSE 已添加 (MIT)
- [x] .gitignore 已配置
- [x] CONTRIBUTING.md 已编写
- [x] 版本号在 `src-tauri/Cargo.toml` 中设置 (推荐: 0.1.0)
- [x] 版本号在 `package.json` 中设置

## 2️⃣ Git 初始化

```bash
cd /Users/mariozzj/Documents/project/bar-tomato

# 初始化 Git 仓库（如果还未初始化）
git init

# 添加所有文件
git add .

# 首次提交
git commit -m "feat: initial commit - Bar Tomato v0.1.0

- Pomodoro timer with LifeOS integration
- Tray application for macOS and Windows
- Glassmorphism UI design
- Auto-launch and dark mode support
- Obsidian vault synchronization"

# 创建 main 分支
git branch -M main
```

## 3️⃣ GitHub 仓库创建

1. 访问 https://github.com/new
2. 填写仓库信息：
   - **Repository name**: bar-tomato
   - **Description**: A beautiful Pomodoro timer app with Obsidian LifeOS integration
   - **Visibility**: Public
   - **Initialize**: 不勾选（我们已有本地仓库）

## 4️⃣ 连接远程仓库并推送

```bash
# 添加远程仓库（替换 YOUR_USERNAME）
git remote add origin https://github.com/YOUR_USERNAME/bar-tomato.git

# 推送到 GitHub
git branch -M main
git push -u origin main
```

## 5️⃣ GitHub 仓库配置

### 基本信息
- [ ] 添加仓库描述
- [ ] 添加仓库主题（topics）：
  - pomodoro-timer
  - obsidian
  - lifeos
  - tauri
  - rust
  - svelte

### 关于部分
- [ ] 添加项目主页（Website）: 如有官网
- [ ] 添加文档链接

### 发布和下载
- [ ] 创建 Release 标签 v0.1.0
- [ ] 编写 Release Notes
- [ ] 上传构建的二进制文件（可选）

## 6️⃣ 创建第一个 Release

```bash
# 创建 Git 标签
git tag -a v0.1.0 -m "Release version 0.1.0"

# 推送标签到 GitHub
git push origin v0.1.0

# 或在 GitHub 上手动创建 Release
# 访问: https://github.com/YOUR_USERNAME/bar-tomato/releases/new
```

### Release Notes 模板

```markdown
# 🍅 Bar Tomato v0.1.0

首个正式版本！

## ✨ 功能

- 💼 番茄钟计时（25 分钟可配置）
- 📊 与 Obsidian LifeOS 无缝集成
- 🎨 液态玻璃 UI 设计
- ⏱️ 秒表模式
- 📱 托盘应用
- 🚀 开机自启
- 📝 任务管理和追踪

## 📦 下载

- macOS: [Bar Tomato.app.tar.gz](...)
- Windows: [Bar Tomato.msi](...)

## 🐛 已知问题

暂无

## 📖 文档

- [README](https://github.com/mariozzj/bar-tomato#readme)
- [贡献指南](CONTRIBUTING.md)

---

感谢使用 Bar Tomato！
```

## 7️⃣ 关键文件检查

- [x] README.md - 完整的项目说明
- [x] LICENSE - MIT 许可证
- [x] .gitignore - 排除不必要的文件
- [x] CONTRIBUTING.md - 贡献指南
- [x] src-tauri/Cargo.toml - 版本信息
- [x] package.json - 版本信息

## 8️⃣ 推广

- [ ] 分享到 Obsidian 社区论坛
- [ ] 分享到 Reddit (r/obsidian)
- [ ] 添加到 Awesome Lists
- [ ] 发送到 Obsidian 插件展示

## 📋 快速命令参考

```bash
# 查看当前配置
git remote -v
git log --oneline -5

# 同步更新
git pull origin main

# 提交新更改
git add .
git commit -m "type: description"
git push origin main

# 创建新标签
git tag v0.2.0
git push origin v0.2.0
```

## 🎯 后续步骤

1. **监控 Issues** - 快速响应用户反馈
2. **Releases** - 定期发布新版本
3. **CI/CD** - 考虑使用 GitHub Actions 自动构建
4. **文档** - 不断完善使用文档
5. **社区** - 与用户交流和互动

---

**祝您发布顺利！🚀**
