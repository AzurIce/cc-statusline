# cc-statusline 设计草稿

## claude-hud 功能分析

### HUD 显示内容 → 实现方式

#### 1. 模型信息行
**显示内容：**
- 模型名称徽章：`[Opus]` / `[Sonnet]` / `[Haiku]`
- 提供商标签：`Bedrock` / `API`（特定情况）

**实现方式：**
- 从 stdin JSON 读取 `model.display_name` 或 `model.id`
- Bedrock 模型检测：`model.id` 包含 `anthropic.claude-`
- 模型 ID 规范化：解析版本号（如 `Claude Sonnet 3.5`）

---

#### 2. 项目路径
**显示内容：**
- 1-3 层目录：`my-project` / `apps/my-project` / `dev/apps/my-project`

**实现方式：**
- 从 stdin JSON 读取 `cwd`
- 根据配置 `pathLevels` (1/2/3) 截取路径层级
- 路径分隔符处理（跨平台）

---

#### 3. Git 状态
**显示内容：**
- 基础：`git:(main)` / `git:(main*)`
- 远程同步：`git:(main ↑2 ↓1)`
- 文件统计：`git:(main* !3 +1 ✘2 ?1)`

**实现方式：**
- 执行 git 命令或使用 git2 库
- `git rev-parse --abbrev-ref HEAD` - 分支名
- `git status --porcelain` - 文件状态
- `git rev-list --left-right --count @{u}...HEAD` - 领先/落后数
- 脏标记：检查是否有未提交更改

---

#### 4. 上下文窗口
**显示内容：**
- 进度条：`Context █████░░░░░ 45%`
- Token 详情：`45k/200k` / `55% remaining` / `45% (45k/200k)`
- 高使用率详细分解（85%+）：显示 input/cache_creation/cache_read tokens

**实现方式：**
- **优先**：从 stdin JSON 读取 `context_window.used_percentage`（v2.1.6+）
- **回退**：手动计算
  - 总 tokens = `input_tokens + cache_creation_input_tokens + cache_read_input_tokens`
  - 百分比 = `(总tokens / context_window_size) * 100`
  - Autocompact buffer：根据使用率动态添加 5%-50% 缓冲
- 颜色动态变化：绿色 → 黄色 → 红色（根据阈值）

---

#### 5. 使用限制（订阅用户）
**显示内容：**
- 5小时窗口：`Usage ██░░░░░░░░ 25% (1h 30m / 5h)`
- 7天窗口：`██████████ 85% (2d / 7d)`（超过阈值时显示）
- 重置倒计时

**实现方式：**
- 从 stdin JSON 读取 `rate_limits.five_hour` 和 `rate_limits.seven_day`
- 百分比：`used_percentage` (0-100)
- 重置时间：`resets_at`（Unix 时间戳）转换为倒计时
- 特殊处理：
  - Bedrock 模型不显示
  - API 用户不显示
  - 免费账户只显示 7 天窗口

---

#### 6. 配置文件统计
**显示内容：**
- `📄 2 CLAUDE.md | 🔧 3 rules | 🔌 5 MCPs | ⚡ 2 hooks`

**实现方式：**
- 扫描 `.claude/` 目录
- 统计文件数量：
  - CLAUDE.md：递归查找 `CLAUDE.md` 文件
  - Rules：`.claude/rules/` 目录下文件
  - MCPs：解析 `.claude/mcp.json` 中的 `mcpServers` 数量
  - Hooks：解析 `.claude/hooks.json` 中的 hooks 数量

---

#### 7. 会话信息
**显示内容：**
- 会话时长：`5m` / `1h 30m`
- 会话名称：slug 或自定义标题
- 输出速度：`out: 42.1 tok/s`

**实现方式：**
- 会话开始时间：从 transcript 第一行的 `timestamp` 提取
- 时长计算：`当前时间 - 会话开始时间`
- 格式化：`<1m` / `Nm` / `Nh Nm`
- 会话名称：从 transcript 解析 `slug` 或 `customTitle`
- 输出速度：需要追踪 output tokens 和时间

---

#### 8. 内存使用
**显示内容：**
- `Memory: 8.2 GB / 16 GB (51%)`

**实现方式：**
- 读取系统内存信息
- Linux: `/proc/meminfo`
- macOS: `sysctl` 命令或系统 API
- Windows: Windows API
- 计算：`(used / total) * 100`

---

#### 9. Claude Code 版本
**显示内容：**
- `CC v2.1.81`

**实现方式：**
- 多种检测方式（按优先级）：
  1. 查找 Claude Code 可执行文件并执行 `--version`
  2. 读取 `package.json`（如果是 npm 安装）
  3. 检查环境变量
- 版本号提取和格式化

---

#### 10. 工具活动行
**显示内容：**
- `◐ Edit: auth.ts | ✓ Read ×3 | ✓ Grep ×2`
- 状态图标：`◐` 运行中 / `✓` 完成 / `✗` 错误

**实现方式：**
- 解析 transcript JSONL 文件
- 识别 `tool_use` 类型的 content block
- 提取：
  - 工具名称：`name` 字段
  - 工具 ID：`id` 字段
  - 目标：从 `input` 提取（file_path, pattern, command 等）
  - 开始时间：当前行的 `timestamp`
- 匹配 `tool_result` 更新状态和结束时间
- 保留最近 20 个工具调用
- 聚合相同工具的调用次数

---

#### 11. Agent 活动行
**显示内容：**
- `◐ explore [haiku]: Finding auth code (2m 15s)`
- 显示 agent 类型、模型、描述、运行时长

**实现方式：**
- 识别 `Task` 工具调用（subagent 启动标志）
- 从 `input` 提取：
  - `subagent_type`：agent 类型
  - `model`：使用的模型
  - `description`：任务描述
- 计算运行时长：`当前时间 - startTime`
- 保留最近 10 个 agent

---

#### 12. Todo 任务行
**显示内容：**
- `▸ Fix authentication bug (2/5)`
- 显示当前任务和完成进度

**实现方式：**
- 识别 todo 相关工具：
  - `TodoWrite`：完整替换 todo 列表
  - `TaskCreate`：创建新任务
  - `TaskUpdate`：更新任务状态
- 从 `input` 提取：
  - `subject` / `description`：任务内容
  - `status`：pending / in_progress / completed
  - `taskId`：任务 ID（用于更新）
- 维护任务 ID 到索引的映射
- 计算完成进度：`completed / total`

---

#### 13. 自定义命令输出
**显示内容：**
- 用户自定义的额外信息行

**实现方式：**
- 通过命令行参数 `--extra-cmd` 传入命令
- 执行命令并捕获 stdout
- 显示在独立行

---

### 渲染系统

#### 布局模式
1. **Compact 模式**：单行显示核心信息
2. **Expanded 模式**：多行显示，可配置元素顺序

#### 文本处理
- **终端宽度检测**：`stdout.columns` → `stderr.columns` → `$COLUMNS`
- **智能换行**：按 ` | ` 或 ` │ ` 分隔符拆分
- **截断**：超长内容添加 `...`
- **Unicode 处理**：
  - Grapheme cluster 分割（处理组合字符、emoji）
  - 宽字符检测（CJK、emoji 占 2 格）
  - ANSI 转义序列保留

#### 颜色系统
- 11 种可配置颜色（context, usage, warning, critical, model, project, git, gitBranch, label, custom, usageWarning）
- 支持：命名颜色 / 256色索引 (0-255) / hex (#rrggbb)
- 动态颜色：根据阈值自动变化

---

### 配置系统
- **配置文件**：`~/.claude/plugins/claude-hud/config.json`
- **缓存文件**：`~/.claude/plugins/claude-hud/transcript-cache/<hash>.json`
- **配置项**：
  - 布局模式、路径层级、元素顺序
  - Git 显示选项
  - 各元素显示开关
  - 颜色自定义
  - 阈值设置

---

### Transcript 缓存机制
- 基于文件 `mtime` 和 `size` 的缓存键
- 缓存内容：解析后的 tools/agents/todos 数据
- 缓存失效：文件修改时间或大小变化
- 性能优化：避免重复解析大型 JSONL 文件
