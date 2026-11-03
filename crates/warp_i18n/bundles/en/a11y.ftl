# Accessibility (a11y) labels and help text emitted to screen readers.
# Keys MUST start with `a11y-`. Group keys by file: `a11y-<area>-*`.

# terminal/input.rs
a11y-input-label = Command Input.
a11y-input-helper = Input your shell command, press enter to execute. Press cmd-up to navigate to output of previously executed commands. Press cmd-l to re-focus command input.
a11y-input-workflow-arg-help = Press shift-tab to select the next workflow argument
a11y-input-executed-command = Executed: {$command}

# input_suggestions.rs
a11y-input-suggestions-suggestion = Suggestion: {$text}.
a11y-input-suggestions-selected = Selected: {$text}
a11y-input-suggestions-closed = Closed suggestions.
a11y-input-suggestions-menu-label = Command suggestions.
a11y-input-suggestions-menu-help = Navigate with tab and shift-tab, and confirm with enter. Execute selected command with command + enter. Esc leaves the suggestions menu.

# editor/view/model/mod.rs
a11y-editor-action-suffix-selected = , selected
a11y-editor-action-suffix-unselected = , unselected
a11y-editor-unselected = Unselected
a11y-editor-deleted-suffix = , deleted

# code/editor/find/view.rs
a11y-code-find-result-of = Result {$index} of {$count}.
a11y-code-find-no-results = No results.
a11y-code-find-replace-success = Successfully replaced match. Selected match is {$index} of {$remaining}
a11y-code-find-replace-help = Continue pressing Enter to replace more matches, or use up/down arrows to navigate.
a11y-code-find-replace-success-last = Successfully replaced the last match.
a11y-code-find-result-help = Use enter and shift-enter to navigate between matches. Escape to quit.
a11y-code-find-bar-empty = Find bar for searching text in the editor.
a11y-code-find-bar-with-matches = Find bar with {$count} matches found. Currently on match {$index} of {$count}.
a11y-code-find-help-replace = Replace field focused. Type replacement text, press Enter to replace current match, Tab to return to find field. Use up/down arrows to navigate matches, Escape to close.
a11y-code-find-help-find = Find field focused. Type to search text. Use Enter and Shift-Enter or up/down arrows to navigate between matches. Press Escape to close find bar.

# search/search_bar.rs
a11y-search-loading-suggestions = Loading {$name} suggestions
a11y-search-error-finding-results = Error finding results
a11y-search-selected = Selected {$label}

# Terminal banners and command corrections (terminal/view.rs)
a11y-terminal-banner-recognized = {$title} recognized.
a11y-terminal-correction-suggested = Suggested corrected command: {$command}
a11y-terminal-correction-help = Press right arrow to insert or keep editing to ignore

# Block selection / selection a11y (terminal/view.rs)
a11y-terminal-block-status-failed = failed, status code {$code}
a11y-terminal-block-status-background = background
a11y-terminal-block-status-succeeded = succeeded
a11y-terminal-block-status-in-progress = in progress
a11y-terminal-block-summary = Block {$index}: {$command}, {$status}.
a11y-terminal-block-help = Press cmd-C to read and copy both command and output, and cmd-option-shift-C to read and copy output only. Press cmd-B to bookmark the block: you could navigate between bookmarked blocks quickly using option-up and option-down.

# Block bookmark / scroll / selection actions
a11y-terminal-toggle-bookmark = Toggle Bookmark block
a11y-terminal-selected-blocks = Selected {$count} blocks.
a11y-terminal-selected-all-blocks = Selected all {$count} blocks.
a11y-terminal-scrolled-bottom-selected = Scrolled to bottom of selected block
a11y-terminal-scrolled-top-selected = Scrolled to top of selected block
a11y-terminal-scrolled-bottom-overhanging = Scrolled to bottom of bottommost visible block

# Copy outputs / blocks (joined with newline-prefixed body in Rust)
a11y-terminal-copied-block-outputs-header = Copied {$count} block outputs.
a11y-terminal-block-output-entry = Block {$index}.
    Output: {$output}
a11y-terminal-copied-blocks-header = Copied {$count} blocks.
a11y-terminal-block-copy-entry = Block {$index}: {$command}. Output: {$output}

# Block filter / initialization / settings
a11y-terminal-open-block-filter = Open block filter editor for block {$index}
a11y-terminal-showed-init-block = Showed initialization block
a11y-terminal-opened-warpify-settings = Opened Warpify Settings
a11y-terminal-opened-files-palette = Opened file search palette

# AI block / conversation
a11y-terminal-ai-attached-blocks-menu = Open list of blocks attached as context to this AI query.
a11y-terminal-ai-overflow-menu = Open overflow menu with copy options for this AI block.
a11y-terminal-ai-rewind-confirm = Show confirmation dialog to rewind to before this point in the AI conversation.
a11y-terminal-ai-rewind-execute = Execute rewind to before this point in the AI conversation.
a11y-terminal-ai-select-attached-block = Click on a block attached as context to this AI query.
a11y-terminal-pick-repo = Use file picker to select a git repository

# Notebook editor (notebooks/editor/view.rs)
a11y-notebook-pasting = Pasting: {$text}
a11y-notebook-shift-tab = Shift-tab
a11y-notebook-edit-link = Edit Link
a11y-notebook-copy-link = Copy Link
a11y-notebook-open-link = Open link: {$url}
a11y-notebook-secondary-click-link = Secondary click on {$url}
a11y-notebook-delete-line-left = Delete line left
a11y-notebook-delete-line-right = Delete line right
a11y-notebook-delete-word-left = Delete word left
a11y-notebook-delete-word-right = Delete word right
a11y-notebook-cut-line-left = Cut line left
a11y-notebook-cut-line-right = Cut line right
a11y-notebook-cut-word-left = Cut word left
a11y-notebook-cut-word-right = Cut word right
a11y-notebook-show-character-palette = Show character palette
a11y-notebook-show-find-bar = Show find bar
a11y-notebook-open-block-insertion-menu = Open block-insertion menu
a11y-notebook-open-embedded-object-search = Open embedded object search menu
a11y-notebook-insert-block = Insert {$kind} block
a11y-notebook-deselect-command = De-select command
a11y-notebook-deselect-command-help = Switch from selecting commands to selecting text
a11y-notebook-change-code-language = Change code block language to {$language}
a11y-notebook-copy-code-block = Copy code block
a11y-notebook-toggle-task-list = Toggle task list

# view_components/find.rs (shares result/no-results keys with code/editor/find/view.rs)
a11y-find-bar-help-prompt = Type searched phrase.
a11y-find-bar-help = Press escape to quit, use enter and shift-enter to navigate between matches

# terminal/view/open_in_warp.rs
a11y-open-in-warp-target = Open {$path} in Warp
a11y-open-in-warp-close-banner = Close View in Warp banner
a11y-open-in-warp-learn-more-label = Learn more
a11y-open-in-warp-learn-more-help = Learn more about opening Markdown files in Warp

# search/command_search/view.rs
a11y-command-search-label = Command Search
a11y-command-search-help = Search your history, workflows, and more.  Use the Up and Down arrows to browse search results after typing.  Press Enter to accept a selected result, inserting it into the terminal input.  Press Escape to close.

# notebooks/editor/omnibar.rs
a11y-omnibar-convert-to = Convert to {$style}
a11y-omnibar-remove-link = Remove link

# notebooks/editor/model.rs
a11y-notebooks-editor-selected-workflow = Selected workflow: {$command}

# editor/view/mod.rs
a11y-editor-view-pasting = Pasting: {$text}

# autoupdate/mod.rs
a11y-autoupdate-update-available-label = Update available.
a11y-autoupdate-update-available-help = Use the command palette to install and relaunch Warp
a11y-autoupdate-no-updates = No updates available

# workspace/view.rs
a11y-workspace-verbosity-set = {$verbosity} accessibility announcements set

# themes/theme_chooser.rs
a11y-theme-chooser-label = Theme chooser. Unfortunately, theme chooser window isn't compatible with screen readers yet.
a11y-theme-chooser-help = Press escape to close.

# reward_view.rs
a11y-reward-title = Congrats!
a11y-reward-help = Press enter to open the theme chooser or escape to dismiss.

# notebooks/notebook.rs + notebooks/file/mod.rs (shared {$title} suffix)
a11y-notebook-suffix = {$title} notebook

# launch_configs/save_modal.rs
a11y-launch-save-label = Save Config Modal
a11y-launch-save-help = Type the name of the file to which you want to save your current configuration of windows, tabs, and panes. Use enter to save the launch configuration, esc to quit the save configuration modal.

# auth/auth_view_body.rs
a11y-auth-welcome-label = Welcome to Warp!
a11y-auth-welcome-help = Press enter to open your browser to Sign Up or Sign In.

# auth/auth_override_warning_body.rs
a11y-auth-override-warning-label = New login detected
a11y-auth-override-warning-help = Warp has detected a new login from a web browser. Press escape to cancel and continue using Warp without login.

# search/command_search/view.rs (extra)
a11y-command-search-result-executed-label = Result executed
a11y-command-search-result-executed-help = Press Cmd-Up to navigate to the command's output.
a11y-command-search-result-accepted-label = Result accepted.
a11y-command-search-result-accepted-help = You can edit the command here before pressing Enter to execute it.

# notebooks/editor/model.rs (extra)
a11y-notebook-style-toggle-on = {$style} on
a11y-notebook-style-toggle-off = {$style} off

# notebooks/editor/find_bar.rs
a11y-notebook-find-bar-enable-regex = Enable regex search
a11y-notebook-find-bar-disable-regex = Disable regex search
a11y-notebook-find-bar-enable-case-sensitive = Enable case-sensitive search
a11y-notebook-find-bar-disable-case-sensitive = Disable case-sensitive search
a11y-notebook-find-bar-focus-next = Focus next match
a11y-notebook-find-bar-focus-previous = Focus previous match
a11y-notebook-find-bar-close = Close find bar
