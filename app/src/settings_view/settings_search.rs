const SEARCH_ALIASES: &[(&[&str], &[&str])] = &[
    (&["字体"], &["font", "typeface"]),
    (&["字体大小", "字号"], &["font size", "fontsize"]),
    (&["大小", "尺寸"], &["size", "sizing"]),
    (&["字体粗细", "字重"], &["font weight"]),
    (&["行高", "行间距"], &["line height", "line spacing"]),
    (&["主题", "配色"], &["theme", "color scheme"]),
    (&["外观"], &["appearance", "style"]),
    (&["颜色", "色彩"], &["color", "colour"]),
    (&["背景"], &["background"]),
    (&["终端"], &["terminal", "shell"]),
    (&["终端字体"], &["terminal font"]),
    (
        &["语言", "界面语言", "中英文", "中文", "英文"],
        &["language", "locale"],
    ),
    (
        &["快捷键", "键盘快捷键", "按键绑定"],
        &["shortcut", "keybinding"],
    ),
    (&["键盘"], &["keyboard", "keybinding"]),
    (&["热键"], &["hotkey", "shortcut"]),
    (&["剪贴板"], &["clipboard", "copy", "paste"]),
    (&["复制"], &["copy"]),
    (&["粘贴", "贴入"], &["paste"]),
    (&["隐私"], &["privacy"]),
    (&["安全"], &["security", "secure"]),
    (&["遥测", "使用数据"], &["telemetry", "analytics"]),
    (&["通知", "提醒"], &["notification", "notify", "bell"]),
    (
        &["自动补全", "补全"],
        &["completion", "autocomplete", "autosuggestion"],
    ),
    (&["自动建议", "智能建议"], &["autosuggestion", "suggestion"]),
    (&["人工智能", "智能助手"], &["ai"]),
    (&["代理", "智能体"], &["agent", "proxy"]),
    (&["网络代理"], &["proxy"]),
    (&["模型"], &["model"]),
    (&["ai模型", "人工智能模型"], &["ai model"]),
    (&["默认模型"], &["default model"]),
    (&["索引", "代码索引"], &["index", "indexing"]),
    (&["项目"], &["project"]),
    (
        &["工作目录"],
        &["working directory", "current directory", "cwd"],
    ),
    (
        &["默认工作目录", "默认目录"],
        &["default directory", "default cwd"],
    ),
    (&["目录", "文件夹"], &["directory", "folder", "cwd"]),
    (&["文件"], &["file"]),
    (&["滚动", "滚屏"], &["scroll", "scrolling", "scrollback"]),
    (&["滚动历史", "回滚缓冲区"], &["scrollback"]),
    (
        &["透明度", "不透明度", "透明"],
        &["opacity", "transparency", "transparent"],
    ),
    (&["模糊"], &["blur"]),
    (&["光标", "插入符"], &["cursor", "caret"]),
    (&["提示符"], &["prompt"]),
    (&["输入"], &["input"]),
    (&["输出"], &["output"]),
    (&["窗口"], &["window"]),
    (&["标签页", "标签"], &["tab"]),
    (&["面板"], &["pane", "panel"]),
    (&["分屏"], &["split"]),
    (&["编辑器"], &["editor"]),
    (&["代码"], &["code"]),
    (&["搜索", "查找"], &["search", "find"]),
    (&["历史", "历史记录"], &["history"]),
    (&["命令"], &["command"]),
    (&["账户", "账号"], &["account"]),
    (&["登录"], &["login", "sign in"]),
    (&["同步"], &["sync", "synchronization"]),
    (&["默认", "默认值"], &["default"]),
    (&["自动"], &["auto", "automatic"]),
    (&["启动"], &["startup", "launch"]),
    (&["恢复"], &["restore", "recovery"]),
    (&["内边距", "边距"], &["padding", "margin"]),
    (&["连字"], &["ligature"]),
    (&["语音", "声音"], &["voice", "audio", "sound"]),
];

pub(super) fn matches(terms: &str, query: &str) -> bool {
    let terms = terms.to_lowercase();
    let query = query.to_lowercase();
    query.split_whitespace().all(|word| {
        terms.contains(word)
            || SEARCH_ALIASES.iter().any(|(aliases, alternatives)| {
                aliases.contains(&word)
                    && alternatives.iter().any(|alternative| {
                        alternative
                            .split_whitespace()
                            .all(|keyword| contains_keyword(&terms, keyword))
                    })
            })
    })
}

fn contains_keyword(terms: &str, keyword: &str) -> bool {
    // Token boundaries keep aliases such as AI from matching unrelated words like "painting".
    terms
        .split(|character: char| !character.is_alphanumeric())
        .any(|term| term == keyword || term.strip_suffix('s') == Some(keyword))
}

#[cfg(test)]
#[path = "settings_search_tests.rs"]
mod tests;
