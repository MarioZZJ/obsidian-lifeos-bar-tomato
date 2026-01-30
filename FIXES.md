# bar-tomato 修复说明

## 已修复的问题

### 1. 状态栏双图标问题 ✅
- **问题**: macOS 状态栏出现两个图标
- **原因**: Tauri 创建了默认应用图标 + 手动创建的托盘图标
- **修复**:
  - 给托盘图标添加了 ID (`"main"`)
  - 修改了 `tray_title()` 返回格式，不再包含 🍅（因为托盘图标本身已经显示）
  - 在更新标题时添加 🍅 前缀：`format!("🍅{}", timer.tray_title())`
- **效果**: 现在只显示一个图标，格式为 `🍅 24:37`（计时中）或 `🍅`（空闲）

### 2. 配置持久化问题 ✅
- **问题**: 重启后 Vault 路径丢失
- **原因**: 配置只保存在内存中
- **修复**:
  - 新增 `app_config.rs` 模块
  - 使用 `dirs` crate 获取系统配置目录
  - 配置保存在 `~/.config/bar-tomato/config.json`（macOS）
  - 启动时自动加载保存的配置
  - 设置 Vault 路径时自动保存
- **效果**: 重启应用后自动恢复之前配置的 Vault 路径

### 3. 项目名称绑定问题 ✅
- **问题**: 只记录项目名（如 "DualBasic"），丢失了领域信息
- **原因**: 代码中使用 `pn.split('-').last()` 截取了最后一部分
- **修复**:
  - 移除了所有 `split('-').last()` 的截取逻辑
  - 直接使用完整的项目文件夹名称（如 "科学研究-DualBasic"）
  - 在 `projects.rs` 中保持 `display_name` 为完整名称
- **效果**: 日记中的项目时间记录现在显示完整的"领域-项目"格式

## 测试方法

1. **测试托盘图标**:
   ```bash
   cd /Users/mariozzj/Documents/project/bar-tomato
   cargo tauri dev
   ```
   - 检查状态栏是否只有一个 🍅 图标
   - 开始计时，检查是否显示为 `🍅 24:37` 格式

2. **测试配置持久化**:
   - 设置 Vault 路径
   - 关闭应用
   - 重新启动应用
   - 检查 Vault 路径是否自动恢复
   - 配置文件位置: `~/.config/bar-tomato/config.json`

3. **测试项目名称**:
   - 选择一个任务（如 "科学研究-DualBasic" 项目下的任务）
   - 完成一个番茄钟
   - 检查日记 `## 项目列表` 区域
   - 应该显示: `6. [[1. 项目/科学研究-DualBasic/DualBasic.README.md|科学研究-DualBasic]] 0hr25`

## 文件变更清单

- `src-tauri/src/app_config.rs` - 新增配置持久化模块
- `src-tauri/src/lib.rs` - 修改托盘图标创建和配置加载
- `src-tauri/src/timer/state.rs` - 修改 `tray_title()` 格式
- `src-tauri/src/commands.rs` - 添加配置保存，移除项目名截取
- `src-tauri/src/vault/projects.rs` - 保持完整项目名称
- `src-tauri/Cargo.toml` - 添加 `dirs` 依赖
