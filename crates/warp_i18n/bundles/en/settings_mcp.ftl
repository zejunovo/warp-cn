# MCP Servers settings page strings.
# Keys MUST start with `settings-mcp-`.

settings-mcp-title = MCP Servers
settings-mcp-no-server-selected = No MCP server selected
settings-mcp-no-tools-available = No tools available
settings-mcp-no-updates-available = No updates available

# Page-level toasts and errors emitted from mcp_servers_page.rs
settings-mcp-logout-success-named = Successfully logged out of {$name} MCP server
settings-mcp-logout-success-generic = Successfully logged out of MCP server
settings-mcp-install-modal-busy = Finish the current MCP install before opening another install link.
settings-mcp-unknown-server = Unknown MCP server '{$name}'
settings-mcp-cannot-install-from-link = MCP server '{$name}' cannot be installed from this link.

# update_modal.rs
settings-mcp-update-server-fallback = Server
settings-mcp-update-title = Update {$name}
settings-mcp-update-modal-esc = ESC
settings-mcp-update-modal-description = This server has {$count} updates available, which would you like to proceed with?
settings-mcp-update-cancel = Cancel
settings-mcp-update-confirm = Update
settings-mcp-update-publisher-another-device = another device
settings-mcp-update-publisher-team-member = a team member
settings-mcp-update-from = Update from {$publisher}
settings-mcp-update-version = Version {$version}

# installation_modal.rs
settings-mcp-install-title = Install {$name}
settings-mcp-install-modal-esc = ESC
settings-mcp-install-source-shared-team = Shared from team
settings-mcp-install-source-another-device = From another device
settings-mcp-install-cancel = Cancel
settings-mcp-install-confirm = Install

# destructive_mcp_confirmation_dialog.rs
settings-mcp-confirm-delete-local-title = Delete MCP server?
settings-mcp-confirm-delete-local-desc = This will uninstall and remove this MCP server from all your devices.
settings-mcp-confirm-delete-shared-title = Delete shared MCP server?
settings-mcp-confirm-delete-shared-desc = This will not only delete this MCP server for yourself, but also uninstall and remove this MCP server from Warp and across all of your teammates' devices.
settings-mcp-confirm-unshare-title = Remove shared MCP server from team?
settings-mcp-confirm-unshare-desc = This will uninstall and remove this MCP server from Warp and across all of your teammates' devices.
settings-mcp-confirm-delete-button = Delete MCP
settings-mcp-confirm-remove-from-team-button = Remove from team
settings-mcp-confirm-cancel-button = Cancel

# server_card.rs
settings-mcp-card-status-offline = Offline
settings-mcp-card-status-starting = Starting server...
settings-mcp-card-status-authenticating = Authenticating...
settings-mcp-card-status-shutting-down = Shutting down...
settings-mcp-card-tooltip-show-logs = Show logs
settings-mcp-card-tooltip-log-out = Log out
settings-mcp-card-tooltip-share-server = Share server
settings-mcp-card-tooltip-edit = Edit
settings-mcp-card-tooltip-update-available = Server update available
settings-mcp-card-button-view-logs = View logs
settings-mcp-card-button-edit-config = Edit config
settings-mcp-card-button-set-up = Set up

# edit_page.rs
settings-mcp-edit-save = Save
settings-mcp-edit-edit-variables = Edit Variables
settings-mcp-edit-delete-mcp = Delete MCP
settings-mcp-edit-remove-from-team = Remove from team
settings-mcp-edit-disabled-banner = Only team admins and the creator of the MCP server can edit the MCP server.
settings-mcp-edit-add-new-title = Add New MCP Server
settings-mcp-edit-edit-title-named = Edit {$name} MCP Server
settings-mcp-edit-edit-title-generic = Edit MCP Server
settings-mcp-edit-tooltip-log-out = Log out
settings-mcp-edit-error-contains-secrets = This MCP server contains secrets. Visit Settings > Privacy to modify your secret redaction settings.
settings-mcp-edit-error-no-server-specified = No MCP Server specified.
settings-mcp-edit-error-multiple-servers = Cannot add multiple MCP servers while editing a single server.

# list_page.rs
settings-mcp-list-page-description = Add MCP servers to extend the Warp Agent's capabilities. MCP servers expose data sources or tools to agents through a standardized interface, essentially acting like plugins. Add a custom server, or use the presets to get started with popular servers. You can also find team servers that have been shared with you here.
settings-mcp-list-empty-state = Once you add a MCP server, it will be shown here.
settings-mcp-list-no-search-results = No search results found
settings-mcp-list-search-placeholder = Search MCP Servers
settings-mcp-list-available-to-install = Available to install
settings-mcp-list-detected-from-config = Detected from config file
settings-mcp-list-auto-spawn-toggle = Auto-spawn servers from third-party agents
settings-mcp-list-file-based-desc = Automatically detect and spawn MCP servers from globally-scoped third-party AI agent configuration files (e.g. in your home directory). Servers detected inside a repository are never spawned automatically and must be enabled individually in the "Detected from" sections below.
settings-mcp-list-see-providers = See supported providers.
settings-mcp-list-learn-more = Learn more.
settings-mcp-list-section-my-mcps = My MCPs
settings-mcp-list-section-shared-by-warp-and-team = Shared by Warp and {$team}
settings-mcp-list-section-shared-by-warp-and-others = Shared by Warp and from other devices
settings-mcp-list-section-shared-from-warp = Shared from Warp
settings-mcp-list-section-detected-from = Detected from {$provider}
settings-mcp-list-chip-shared-by = Shared by: {$name}
settings-mcp-list-chip-shared-by-team-member = Shared by a team member
settings-mcp-list-chip-from-another-device = From another device
