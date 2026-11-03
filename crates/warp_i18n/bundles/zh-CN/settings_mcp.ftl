# MCP 服务器设置页字符串。
# Keys MUST start with `settings-mcp-`.

settings-mcp-title = MCP 服务器
settings-mcp-no-server-selected = 未选择 MCP 服务器
settings-mcp-no-tools-available = 暂无可用工具
settings-mcp-no-updates-available = 暂无可用更新

# Page-level toasts and errors emitted from mcp_servers_page.rs
settings-mcp-logout-success-named = 已成功登出 {$name} MCP 服务器
settings-mcp-logout-success-generic = 已成功登出 MCP 服务器
settings-mcp-install-modal-busy = 请先完成当前 MCP 安装，再打开新的安装链接。
settings-mcp-unknown-server = 未知 MCP 服务器 “{$name}”
settings-mcp-cannot-install-from-link = MCP 服务器 “{$name}” 无法通过此链接安装。

# update_modal.rs
settings-mcp-update-server-fallback = 服务器
settings-mcp-update-title = 更新 {$name}
settings-mcp-update-modal-esc = ESC
settings-mcp-update-modal-description = 该服务器有 {$count} 个可用更新，请选择要执行的更新。
settings-mcp-update-cancel = 取消
settings-mcp-update-confirm = 更新
settings-mcp-update-publisher-another-device = 另一台设备
settings-mcp-update-publisher-team-member = 团队成员
settings-mcp-update-from = 来自 {$publisher} 的更新
settings-mcp-update-version = 版本 {$version}

# installation_modal.rs
settings-mcp-install-title = 安装 {$name}
settings-mcp-install-modal-esc = ESC
settings-mcp-install-source-shared-team = 团队共享
settings-mcp-install-source-another-device = 来自另一台设备
settings-mcp-install-cancel = 取消
settings-mcp-install-confirm = 安装

# destructive_mcp_confirmation_dialog.rs
settings-mcp-confirm-delete-local-title = 删除该 MCP 服务器？
settings-mcp-confirm-delete-local-desc = 此操作将卸载并移除你所有设备上的此 MCP 服务器。
settings-mcp-confirm-delete-shared-title = 删除共享的 MCP 服务器？
settings-mcp-confirm-delete-shared-desc = 此操作不仅会从你账户中删除此 MCP 服务器，还会从 Warp 中卸载并移除所有团队成员设备上的此 MCP 服务器。
settings-mcp-confirm-unshare-title = 从团队中移除共享的 MCP 服务器？
settings-mcp-confirm-unshare-desc = 此操作将从 Warp 中卸载并移除所有团队成员设备上的此 MCP 服务器。
settings-mcp-confirm-delete-button = 删除 MCP
settings-mcp-confirm-remove-from-team-button = 从团队移除
settings-mcp-confirm-cancel-button = 取消

# server_card.rs
settings-mcp-card-status-offline = 离线
settings-mcp-card-status-starting = 服务器启动中…
settings-mcp-card-status-authenticating = 认证中…
settings-mcp-card-status-shutting-down = 正在关闭…
settings-mcp-card-tooltip-show-logs = 查看日志
settings-mcp-card-tooltip-log-out = 登出
settings-mcp-card-tooltip-share-server = 共享服务器
settings-mcp-card-tooltip-edit = 编辑
settings-mcp-card-tooltip-update-available = 有可用的服务器更新
settings-mcp-card-button-view-logs = 查看日志
settings-mcp-card-button-edit-config = 编辑配置
settings-mcp-card-button-set-up = 设置

# edit_page.rs
settings-mcp-edit-save = 保存
settings-mcp-edit-edit-variables = 编辑变量
settings-mcp-edit-delete-mcp = 删除 MCP
settings-mcp-edit-remove-from-team = 从团队移除
settings-mcp-edit-disabled-banner = 仅团队管理员和该 MCP 服务器的创建者可编辑此 MCP 服务器。
settings-mcp-edit-add-new-title = 添加新 MCP 服务器
settings-mcp-edit-edit-title-named = 编辑 {$name} MCP 服务器
settings-mcp-edit-edit-title-generic = 编辑 MCP 服务器
settings-mcp-edit-tooltip-log-out = 登出
settings-mcp-edit-error-contains-secrets = 此 MCP 服务器包含敏感信息。请前往 设置 > 隐私 修改敏感信息脱敏设置。
settings-mcp-edit-error-no-server-specified = 未指定 MCP 服务器。
settings-mcp-edit-error-multiple-servers = 编辑单个服务器时无法同时添加多个 MCP 服务器。

# list_page.rs
settings-mcp-list-page-description = 添加 MCP 服务器以扩展 Warp Agent 的能力。MCP 服务器通过标准化接口为 Agent 提供数据源或工具，本质上类似插件。你可以添加自定义服务器，或使用预设快速接入热门服务器，也可在此查看团队成员共享给你的服务器。
settings-mcp-list-empty-state = 添加 MCP 服务器后，会在此处显示。
settings-mcp-list-no-search-results = 未找到匹配结果
settings-mcp-list-search-placeholder = 搜索 MCP 服务器
settings-mcp-list-available-to-install = 可安装
settings-mcp-list-detected-from-config = 从配置文件检测到
settings-mcp-list-auto-spawn-toggle = 自动启动第三方 Agent 的服务器
settings-mcp-list-file-based-desc = 从全局作用域的第三方 AI Agent 配置文件（例如位于主目录中的文件）自动检测并启动 MCP 服务器。在仓库内部检测到的服务器不会自动启动，必须在下方“检测自”区块中单独启用。
settings-mcp-list-see-providers = 查看支持的提供方。
settings-mcp-list-learn-more = 了解更多。
settings-mcp-list-section-my-mcps = 我的 MCP
settings-mcp-list-section-shared-by-warp-and-team = 由 Warp 与 {$team} 共享
settings-mcp-list-section-shared-by-warp-and-others = 由 Warp 与其他设备共享
settings-mcp-list-section-shared-from-warp = Warp 共享
settings-mcp-list-section-detected-from = 检测自 {$provider}
settings-mcp-list-chip-shared-by = 由 {$name} 共享
settings-mcp-list-chip-shared-by-team-member = 由团队成员共享
settings-mcp-list-chip-from-another-device = 来自另一台设备
