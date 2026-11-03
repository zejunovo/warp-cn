# Core UI strings: app shell, common buttons, generic dialogs.
# These keys must always exist in the en bundle. zh-CN may shadow them; missing zh-CN
# entries fall back here (loader.rs fallback chain).

ui-app-name = Warp
ui-button-ok = OK
ui-button-cancel = Cancel
ui-button-save = Save
ui-button-discard = Discard
ui-button-close = Close
ui-button-confirm = Confirm
ui-button-retry = Retry
ui-button-default = Default
tabs-close-confirm = { $n ->
    [one] Close 1 tab
   *[other] Close { $n } tabs
}
