# 无障碍（a11y）标签与帮助文本（用于屏幕阅读器）。
# Key 前缀 `a11y-`，按文件分组：`a11y-<area>-*`。

# terminal/input.rs
a11y-input-label = 命令输入。
a11y-input-helper = 输入 Shell 命令，按 Enter 执行。按 cmd-up 浏览之前已执行命令的输出。按 cmd-l 重新聚焦命令输入。
a11y-input-workflow-arg-help = 按 shift-tab 选择下一个工作流参数
a11y-input-executed-command = 已执行：{$command}

# input_suggestions.rs
a11y-input-suggestions-suggestion = 建议：{$text}。
a11y-input-suggestions-selected = 已选择：{$text}
a11y-input-suggestions-closed = 已关闭建议。
a11y-input-suggestions-menu-label = 命令建议。
a11y-input-suggestions-menu-help = 用 tab 和 shift-tab 导航，按 Enter 确认。按 cmd + Enter 执行所选命令。按 Esc 离开建议菜单。

# editor/view/model/mod.rs
a11y-editor-action-suffix-selected = ，已选择
a11y-editor-action-suffix-unselected = ，已取消选择
a11y-editor-unselected = 已取消选择
a11y-editor-deleted-suffix = ，已删除

# code/editor/find/view.rs
a11y-code-find-result-of = 第 {$index} 项，共 {$count} 项。
a11y-code-find-no-results = 无结果。
a11y-code-find-replace-success = 替换成功。当前是第 {$index} 项，共 {$remaining} 项
a11y-code-find-replace-help = 继续按 Enter 替换更多匹配，或使用上下方向键浏览。
a11y-code-find-replace-success-last = 已成功替换最后一项匹配。
a11y-code-find-result-help = 用 Enter 与 shift-Enter 在匹配间浏览。按 Esc 退出。
a11y-code-find-bar-empty = 用于在编辑器中搜索文本的查找栏。
a11y-code-find-bar-with-matches = 查找栏共找到 {$count} 项匹配。当前是第 {$index} 项，共 {$count} 项。
a11y-code-find-help-replace = 替换字段已聚焦。输入替换文本，按 Enter 替换当前匹配，按 Tab 返回查找字段。用上下方向键浏览匹配项，Esc 关闭。
a11y-code-find-help-find = 查找字段已聚焦。输入要搜索的文本。用 Enter、Shift-Enter 或上下方向键在匹配间浏览。按 Esc 关闭查找栏。

# search/search_bar.rs
a11y-search-loading-suggestions = 正在加载 {$name} 建议
a11y-search-error-finding-results = 查找结果时出错
a11y-search-selected = 已选择 {$label}

# 终端横幅与命令纠错（terminal/view.rs）
a11y-terminal-banner-recognized = 已识别 {$title}。
a11y-terminal-correction-suggested = 建议的纠正命令：{$command}
a11y-terminal-correction-help = 按右方向键插入，或继续编辑以忽略

# 区块选择 / 选区 a11y（terminal/view.rs）
a11y-terminal-block-status-failed = 失败，状态码 {$code}
a11y-terminal-block-status-background = 后台
a11y-terminal-block-status-succeeded = 成功
a11y-terminal-block-status-in-progress = 进行中
a11y-terminal-block-summary = 区块 {$index}：{$command}，{$status}。
a11y-terminal-block-help = 按 cmd-C 朗读并复制命令与输出，按 cmd-option-shift-C 仅朗读并复制输出。按 cmd-B 收藏区块：使用 option-up 与 option-down 可在收藏的区块间快速导航。

# 区块收藏 / 滚动 / 选择操作
a11y-terminal-toggle-bookmark = 切换收藏区块
a11y-terminal-selected-blocks = 已选择 {$count} 个区块。
a11y-terminal-selected-all-blocks = 已选择全部 {$count} 个区块。
a11y-terminal-scrolled-bottom-selected = 已滚动到所选区块底部
a11y-terminal-scrolled-top-selected = 已滚动到所选区块顶部
a11y-terminal-scrolled-bottom-overhanging = 已滚动到最底部可见区块的底部

# 复制输出 / 区块（在 Rust 端用换行拼接正文）
a11y-terminal-copied-block-outputs-header = 已复制 {$count} 个区块的输出。
a11y-terminal-block-output-entry = 区块 {$index}。
    输出：{$output}
a11y-terminal-copied-blocks-header = 已复制 {$count} 个区块。
a11y-terminal-block-copy-entry = 区块 {$index}：{$command}。输出：{$output}

# 区块过滤 / 初始化 / 设置
a11y-terminal-open-block-filter = 为区块 {$index} 打开过滤编辑器
a11y-terminal-showed-init-block = 已显示初始化区块
a11y-terminal-opened-warpify-settings = 已打开 Warpify 设置
a11y-terminal-opened-files-palette = 已打开文件搜索面板

# AI 区块 / 对话
a11y-terminal-ai-attached-blocks-menu = 打开作为该 AI 查询上下文附加的区块列表。
a11y-terminal-ai-overflow-menu = 打开此 AI 区块的复制选项溢出菜单。
a11y-terminal-ai-rewind-confirm = 显示确认对话框，回退到 AI 对话此点之前的状态。
a11y-terminal-ai-rewind-execute = 执行回退到 AI 对话此点之前的状态。
a11y-terminal-ai-select-attached-block = 点击作为该 AI 查询上下文附加的区块。
a11y-terminal-pick-repo = 使用文件选择器选择 git 仓库

# 笔记本编辑器（notebooks/editor/view.rs）
a11y-notebook-pasting = 正在粘贴：{$text}
a11y-notebook-shift-tab = Shift-Tab
a11y-notebook-edit-link = 编辑链接
a11y-notebook-copy-link = 复制链接
a11y-notebook-open-link = 打开链接：{$url}
a11y-notebook-secondary-click-link = 在 {$url} 上次级点击
a11y-notebook-delete-line-left = 向左删除行
a11y-notebook-delete-line-right = 向右删除行
a11y-notebook-delete-word-left = 向左删除单词
a11y-notebook-delete-word-right = 向右删除单词
a11y-notebook-cut-line-left = 向左剪切行
a11y-notebook-cut-line-right = 向右剪切行
a11y-notebook-cut-word-left = 向左剪切单词
a11y-notebook-cut-word-right = 向右剪切单词
a11y-notebook-show-character-palette = 显示字符面板
a11y-notebook-show-find-bar = 显示查找栏
a11y-notebook-open-block-insertion-menu = 打开区块插入菜单
a11y-notebook-open-embedded-object-search = 打开嵌入对象搜索菜单
a11y-notebook-insert-block = 插入 {$kind} 区块
a11y-notebook-deselect-command = 取消选择命令
a11y-notebook-deselect-command-help = 从选择命令切换到选择文本
a11y-notebook-change-code-language = 将代码块语言切换为 {$language}
a11y-notebook-copy-code-block = 复制代码块
a11y-notebook-toggle-task-list = 切换任务列表

# view_components/find.rs（与 code/editor/find/view.rs 共享 result/no-results key）
a11y-find-bar-help-prompt = 输入要搜索的短语。
a11y-find-bar-help = 按 Esc 退出，用 Enter 与 shift-Enter 在匹配间浏览

# terminal/view/open_in_warp.rs
a11y-open-in-warp-target = 在 Warp 中打开 {$path}
a11y-open-in-warp-close-banner = 关闭"在 Warp 中查看"横幅
a11y-open-in-warp-learn-more-label = 了解更多
a11y-open-in-warp-learn-more-help = 进一步了解如何在 Warp 中打开 Markdown 文件

# search/command_search/view.rs
a11y-command-search-label = 命令搜索
a11y-command-search-help = 搜索历史、工作流等。输入后用上下方向键浏览搜索结果。按 Enter 选定结果并插入终端输入。按 Esc 关闭。

# notebooks/editor/omnibar.rs
a11y-omnibar-convert-to = 转换为 {$style}
a11y-omnibar-remove-link = 移除链接

# notebooks/editor/model.rs
a11y-notebooks-editor-selected-workflow = 已选择工作流：{$command}

# editor/view/mod.rs
a11y-editor-view-pasting = 正在粘贴：{$text}

# autoupdate/mod.rs
a11y-autoupdate-update-available-label = 有可用更新。
a11y-autoupdate-update-available-help = 使用命令面板安装并重启 Warp
a11y-autoupdate-no-updates = 暂无可用更新

# workspace/view.rs
a11y-workspace-verbosity-set = 已设置 {$verbosity} 无障碍通报

# themes/theme_chooser.rs
a11y-theme-chooser-label = 主题选择器。当前主题选择器窗口暂不支持屏幕阅读器。
a11y-theme-chooser-help = 按 Esc 关闭。

# reward_view.rs
a11y-reward-title = 恭喜！
a11y-reward-help = 按 Enter 打开主题选择器，或按 Esc 关闭。

# notebooks/notebook.rs + notebooks/file/mod.rs（共享 {$title} 后缀）
a11y-notebook-suffix = {$title} 笔记本

# launch_configs/save_modal.rs
a11y-launch-save-label = 保存配置弹窗
a11y-launch-save-help = 输入要保存当前窗口、标签和窗格配置的文件名。按 Enter 保存启动配置，按 Esc 退出保存配置弹窗。

# auth/auth_view_body.rs
a11y-auth-welcome-label = 欢迎使用 Warp！
a11y-auth-welcome-help = 按 Enter 打开浏览器进行注册或登录。

# auth/auth_override_warning_body.rs
a11y-auth-override-warning-label = 检测到新的登录
a11y-auth-override-warning-help = Warp 检测到来自浏览器的新登录。按 Esc 取消并继续在未登录状态下使用 Warp。

# search/command_search/view.rs（额外）
a11y-command-search-result-executed-label = 结果已执行
a11y-command-search-result-executed-help = 按 Cmd-Up 浏览命令输出。
a11y-command-search-result-accepted-label = 结果已接受。
a11y-command-search-result-accepted-help = 可在执行前编辑此处的命令，按 Enter 执行。

# notebooks/editor/model.rs（额外）
a11y-notebook-style-toggle-on = {$style} 已开启
a11y-notebook-style-toggle-off = {$style} 已关闭

# notebooks/editor/find_bar.rs
a11y-notebook-find-bar-enable-regex = 启用正则搜索
a11y-notebook-find-bar-disable-regex = 禁用正则搜索
a11y-notebook-find-bar-enable-case-sensitive = 启用大小写敏感搜索
a11y-notebook-find-bar-disable-case-sensitive = 禁用大小写敏感搜索
a11y-notebook-find-bar-focus-next = 聚焦下一个匹配
a11y-notebook-find-bar-focus-previous = 聚焦上一个匹配
a11y-notebook-find-bar-close = 关闭查找栏
