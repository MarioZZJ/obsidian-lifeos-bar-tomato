# 贡献指南

感谢您对 Bar Tomato 的关注！欢迎贡献代码、报告问题或提出改进建议。

## 开发环境设置

### 1. Fork 和克隆仓库

```bash
# Fork 仓库后，克隆您的 fork
git clone https://github.com/YOUR_USERNAME/bar-tomato.git
cd bar-tomato

# 添加上游仓库
git remote add upstream https://github.com/mariozzj/bar-tomato.git
```

### 2. 安装依赖

```bash
# 安装 Rust（如果还未安装）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Node.js 依赖
npm install

# 验证 Tauri 环境
cargo tauri info
```

### 3. 开发工作流

```bash
# 开发模式启动应用（自动热重载）
cargo tauri dev

# 运行 Rust 编译检查
cargo check

# 运行 Rust 测试
cargo test

# 构建生产版本
cargo tauri build
```

## 提交 PR 的步骤

### 1. 创建特性分支

```bash
git checkout -b feature/your-feature-name
# 或修复 bug
git checkout -b fix/bug-description
```

### 2. 编写代码

- 遵循现有代码风格
- 添加必要的测试
- 更新相关文档

### 3. 提交代码

```bash
# 添加更改
git add .

# 提交，使用描述性的提交信息
git commit -m "feat: add new feature"
# 或
git commit -m "fix: resolve issue #123"
```

### 4. 推送并创建 PR

```bash
# 推送到您的 fork
git push origin feature/your-feature-name

# 在 GitHub 上创建 Pull Request
# 填写 PR 描述，说明:
# - 做了什么
# - 为什么做
# - 如何测试
```

## 提交规范

采用 Conventional Commits 格式：

```
type(scope): subject

body

footer
```

### 类型（type）
- `feat`: 新功能
- `fix`: 修复 bug
- `docs`: 文档更改
- `style`: 代码风格（不影响功能）
- `refactor`: 重构代码
- `perf`: 性能优化
- `test`: 添加或更新测试
- `chore`: 构建、依赖等杂项

### 示例

```
feat(timer): add overtime support for pomodoro

Allow users to continue timing after pomodoro completes,
displaying overtime in orange color. Users can click
"Rest" button to enter break phase.

Closes #42
```

## 代码风格

### Rust
```bash
# 格式化代码
cargo fmt

# 检查代码风格和潜在问题
cargo clippy
```

### TypeScript/Svelte
```bash
# 代码已通过 Prettier 格式化
# 使用 ESLint 进行检查（如需要配置）
```

## 测试

### Rust 测试
```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

### 手动测试清单
- [ ] 番茄钟正常启动和倒计时
- [ ] 暂停/恢复功能正常
- [ ] 完成后显示加班时间
- [ ] 休息阶段正常工作
- [ ] 任务搜索和选择功能
- [ ] Obsidian 数据同步
- [ ] 托盘菜单和快捷操作
- [ ] 开机自启（Windows/macOS）

## 报告 Bug

提交 issue 时请包含：

1. **详细描述** - 问题是什么？
2. **复现步骤** - 如何重现问题？
3. **预期行为** - 应该如何运行？
4. **实际行为** - 实际发生了什么？
5. **系统信息** - OS 版本、Obsidian 版本等
6. **日志** - 相关的错误日志或截图

示例：
```markdown
### 问题描述
点击选择 Vault 路径后应用闪退

### 复现步骤
1. 打开应用
2. 点击设置按钮
3. 点击"选择" Vault 路径按钮
4. 在文件对话框中选择目录
5. 应用立即关闭

### 预期行为
应该保存选择的路径并显示已连接

### 系统信息
- macOS Sonoma 14.1.2
- Obsidian 1.4.16
- lifeos-pro v1.0.0
```

## 功能请求

提交功能请求时请包含：

1. **功能描述** - 想要什么新功能？
2. **使用场景** - 什么时候会用到？
3. **示例** - 可以参考的类似功能（如果有）

## 文档

- README.md - 项目概述和快速开始
- 代码注释 - 复杂逻辑需要注释
- Commit 消息 - 清晰的提交历史

## 获得帮助

- 📖 查看 [README.md](README.md) 了解项目
- 🐛 查看 [Issues](https://github.com/mariozzj/bar-tomato/issues) 了解已知问题
- 💬 在 Issue 中讨论

## 许可证

所有贡献将遵循 [MIT License](LICENSE)。

---

感谢您的贡献！🎉
