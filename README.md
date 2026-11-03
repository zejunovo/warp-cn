# Warp CN

本仓库是 Warp 的非官方 fork。`cn-dev` 开发分支为桌面客户端加入了简体中文和 English 两种界面语言，涵盖设置、菜单、引导和常用操作提示。自定义模型接入仍在规划中。

[本仓库](https://github.com/zejunovo/warp-cn) · [上游仓库](https://github.com/warpdotdev/warp) · [官方文档](https://docs.warp.dev/)

<a href="https://www.warp.dev">
  <img width="1024" alt="Warp 官方产品预览，非本 fork 的定制效果" src="https://github.com/user-attachments/assets/9976b2da-2edd-4604-a36c-8fd53719c6d4" />
</a>

## 项目说明

Warp 是一个从终端发展而来的 AI 开发环境。本仓库包含其开源客户端代码，主要使用 Rust 开发，并使用自研的 WarpUI 界面框架。

上游提供桌面终端、内置 AI 编程助手，以及对 Claude Code、Codex、Gemini CLI 等命令行工具的支持。有关上游产品的功能和使用方法，请查阅[官方文档](https://docs.warp.dev/)。

## 切换界面语言

桌面客户端默认使用简体中文。在 **设置 → 外观 → 语言**（**Settings → Appearance → Language**）选择 **English** 或 **简体中文**，完全退出并重新打开 Warp 后生效。每台设备单独保存语言偏好。终端中的命令、程序输出及用户输入按原文显示。

## 获取源码

需要先安装 Git 和 Git LFS，然后克隆本仓库：

```bash
git clone https://github.com/zejunovo/warp-cn.git
cd warp-cn
```

后续命令均在仓库根目录执行。构建本 fork 的定制版本前，先用 `git branch -a` 查看分支，再切换到 `cn-dev`：

- 本地已有 `cn-dev`：执行 `git switch cn-dev`。
- 仅远端已有 `origin/cn-dev`：执行 `git switch --track origin/cn-dev`，接着使用已有的定制代码。
- 本地和远端均无 `cn-dev`：先同步 `master`，再执行 `git switch -c cn-dev master` 创建开发分支。

分支约定和上游更新方法见下文。

## 本地构建与运行

Rust 工具链版本由 [rust-toolchain.toml](rust-toolchain.toml) 固定，请以该文件为准。
初始化脚本会检查或安装依赖，可能需要管理员权限、下载开发工具或完成登录，请先查看脚本说明并按提示操作。

### Windows（PowerShell）

准备 Git for Windows 和 Git LFS 后，在 PowerShell 中执行：

```powershell
git lfs install --local
git lfs pull
.\script\windows\bootstrap.ps1
cargo run --bin warp-oss --features gui
```

Windows 初始化脚本会准备 Rust、Visual Studio Build Tools、Windows SDK、CMake、Protobuf、LLVM 等依赖。
如果安装后提示重新打开终端，请在新终端中回到仓库目录，再继续初始化和构建。

初始化细节见 [Windows 初始化脚本](script/windows/bootstrap.ps1)；制作安装包另见 [Windows 安装包说明](script/windows/README.md)。

### macOS / Linux（Bash）

```bash
./script/bootstrap --skip-common-skills
WARP_SKIP_COMMON_SKILLS_INSTALL=1 ./script/run
```

这里跳过的是可选的共享代理技能安装，平台构建依赖仍会正常准备。需要这些技能时，可按 [AGENTS.md](AGENTS.md) 选择安装方式。

macOS 需要 Xcode。Linux 初始化脚本使用 `apt` 安装系统依赖，其他发行版需要根据脚本自行准备对应依赖。
Windows 的 Git Bash 也可以使用上述统一入口，脚本会调用 Windows 平台的初始化流程。

有关工程约定、测试命令和 GUI / TUI 的区别，参见 [AGENTS.md](AGENTS.md)。

## 主要目录

| 路径 | 内容 |
| --- | --- |
| `app/` | 桌面应用及终端、AI、设置等功能模块 |
| `crates/` | 核心库、UI 框架、编辑器及其他公共组件 |
| `crates/warpui/`、`crates/warpui_core/` | WarpUI 界面框架 |
| `crates/warp_i18n/` | 桌面界面翻译资源和 Fluent 运行时 |
| `crates/warp_tui/` | 无图形界面的 TUI 前端 |
| `resources/` | 应用资源 |
| `script/` | 初始化、构建、运行与打包脚本 |
| `specs/` | 功能设计与技术方案 |

## 分支约定与上游同步

| 分支 | 用途 |
| --- | --- |
| `master` | 跟随上游 `warpdotdev/warp` 的 `master`，不放本 fork 的定制改动 |
| `cn-dev` | 保存中文文档、本地化和模型接入等二次开发改动 |

### 首次配置上游

`origin` 指向自己的 fork，`upstream` 指向官方仓库。先用 `git remote -v` 检查；尚未配置 `upstream` 时执行：

```bash
git remote add upstream https://github.com/warpdotdev/warp.git
```

### 接收上游更新

先妥善保存当前改动并确保工作区干净，再依次执行。任一步报错或出现冲突时，先处理完成，再执行下一步。

```bash
git fetch upstream
git switch master
git merge --ff-only upstream/master
git push origin master

git switch cn-dev
git merge master
```

`master` 使用快进合并，防止不经检查就混入分叉历史；如果无法快进，应先检查本地提交与上游的差异。
`cn-dev` 通过合并接收更新，保留自己的定制提交。冲突处理完成后，仍需编译并验证受影响的功能；没有文本冲突也不代表功能一定兼容。

若上游新增或调整了界面文案，在对应组件使用 `warp_i18n::t!` 读取词条，并在[英文资源](crates/warp_i18n/bundles/en)和[简体中文资源](crates/warp_i18n/bundles/zh-CN)的对应 `.ftl` 文件中维护相同的键名和参数名。动态内容（如目录名）作为参数传入。运行 `cargo nextest run -p warp_i18n --lib` 检查词条，再分别使用两种语言查看改动涉及的界面。更多示例和约定见[本地化说明](crates/warp_i18n/README.md)。

完成合并和验证后，再把二次开发分支推送到自己的 fork：

```bash
git push -u origin cn-dev
```

也可以在 GitHub 页面选择 `master`，通过 **Sync fork → Update branch** 同步该远端分支。
网页同步不会更新本地代码或 `cn-dev`：本地还需要拉取 `origin/master`，再合并到 `cn-dev`。

## 下载与使用

[Warp 官方下载页](https://www.warp.dev/download)提供的是官方版本，不包含本 fork 的定制改动。
要使用这里的修改，需要构建对应分支，或使用本仓库后续明确发布的安装包。

## 参与开发与反馈

- 修改本 fork 前，先明确功能范围、方案和验证方式，尽量将定制改动集中在独立模块或配置中，方便后续同步。
- 向上游贡献前，请阅读 [CONTRIBUTING.md](CONTRIBUTING.md)，按其要求讨论 Issue、设计方案和 Pull Request。
- 安全问题请按 [SECURITY.md](SECURITY.md) 的指引私下报告。
- 交流与协作请遵守 [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)。

## 开源许可与致谢

本 fork 保留上游的许可划分：

- `crates/warpui/` 和 `crates/warpui_core/` 使用 [MIT 许可证](LICENSE-MIT)。
- 仓库其余代码使用 [AGPL v3 许可证](LICENSE-AGPL)。

界面翻译资源、Fluent 运行时和编译期宏移植自 [Heartcoolman/warp-cn](https://github.com/Heartcoolman/warp-cn)，并针对本分支的客户端调整。感谢 [Warp 团队](https://github.com/warpdotdev/warp)及社区贡献者。
