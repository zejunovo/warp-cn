use warpui::elements::Empty;
use warpui::{App, AppContext, Element, Entity, View};

use super::super::{FilteredPageType, PageTitle, PageType, SettingsWidget};
use super::matches;
use crate::appearance::Appearance;

#[test]
fn english_queries_keep_case_insensitive_all_words_substring_matching() {
    assert!(matches("active AI autosuggestions prompt", "ACTIVE auto"));
    assert!(matches("file search fuzzy opener", "search file"));
    assert!(!matches("font size terminal", "font opacity"));
    assert!(!matches("font size terminal", "unknown"));
}

#[test]
fn chinese_aliases_match_only_widget_search_metadata() {
    let cases = [
        ("font size terminal", "字体"),
        ("font size terminal", "字体大小"),
        ("font size terminal", "终端 字体"),
        ("keyboard shortcuts", "快捷键"),
        ("clipboard copy on selection", "剪贴板"),
        ("privacy telemetry analytics", "隐私"),
        ("desktop notifications", "通知"),
        ("input autosuggestions", "自动补全"),
        ("agent profiles", "代理"),
        ("ai preferred models", "AI模型"),
        ("project code indexing", "索引"),
        ("new tab working directory", "工作目录"),
        ("maximum scrollback lines", "滚动"),
        ("background opacity", "透明度"),
        ("language locale english chinese", "界面语言"),
        ("sync with os themes", "主题"),
    ];
    for (terms, query) in cases {
        assert!(matches(terms, query), "{query:?} should match {terms:?}");
    }
}

#[test]
fn mixed_language_and_phrase_queries_require_every_concept() {
    assert!(matches("terminal font size", "字体 SIZE"));
    assert!(matches("default ai model", "默认 模型"));
    assert!(matches("default ai model", "默认模型"));
    assert!(!matches("terminal font family", "字体大小"));
    assert!(!matches("terminal font size", "字体 隐私"));
    assert!(!matches("font size", "终端字体"));
}

#[test]
fn chinese_aliases_do_not_match_unrelated_english_word_fragments() {
    assert!(!matches("painting", "人工智能"));
    assert!(!matches("against", "代理"));
    assert!(!matches("profile", "文件"));
    assert!(!matches("font size", "不存在的设置"));
}

#[test]
fn empty_or_whitespace_queries_match_all_widgets() {
    assert!(matches("font size", ""));
    assert!(matches("font size", " \t\n "));
    assert!(matches("", ""));
}

struct SearchTestView;

impl Entity for SearchTestView {
    type Event = ();
}

impl View for SearchTestView {
    fn ui_name() -> &'static str {
        "SearchTestView"
    }

    fn render(&self, _: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

struct SearchTestWidget {
    terms: &'static str,
    visible: bool,
}

impl SettingsWidget for SearchTestWidget {
    type View = SearchTestView;

    fn search_terms(&self) -> &str {
        self.terms
    }

    fn should_render(&self, _: &AppContext) -> bool {
        self.visible
    }

    fn render(&self, _: &Self::View, _: &Appearance, _: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

fn search_page() -> PageType<SearchTestView> {
    let widgets: Vec<Box<dyn SettingsWidget<View = SearchTestView>>> = vec![
        Box::new(SearchTestWidget {
            terms: "terminal font size",
            visible: true,
        }),
        Box::new(SearchTestWidget {
            terms: "terminal font family",
            visible: true,
        }),
        Box::new(SearchTestWidget {
            terms: "window background opacity",
            visible: true,
        }),
        Box::new(SearchTestWidget {
            terms: "hidden font size",
            visible: false,
        }),
    ];
    PageType::new_uncategorized(widgets, Some(PageTitle::new("Appearance")))
}

fn visible_terms(page: &PageType<SearchTestView>) -> Vec<&str> {
    let FilteredPageType::Uncategorized { widgets, title, .. } = page.get_filtered() else {
        panic!("expected Uncategorized page");
    };
    assert!(title.is_some());
    widgets
        .into_iter()
        .map(|widget| widget.search_terms())
        .collect()
}

#[test]
fn chinese_search_isolates_one_row_and_clearing_restores_visible_rows() {
    App::test((), |mut app| async move {
        app.update(|ctx| {
            let mut page = search_page();
            let match_data = page.update_filter("字体 大小", ctx);
            assert!(match_data.is_truthy());
            assert_eq!(visible_terms(&page), ["terminal font size"]);

            page.update_filter("字体", ctx);
            assert_eq!(
                visible_terms(&page),
                ["terminal font size", "terminal font family"]
            );

            page.update_filter("", ctx);
            assert_eq!(
                visible_terms(&page),
                [
                    "terminal font size",
                    "terminal font family",
                    "window background opacity"
                ]
            );
        });
    });
}

#[test]
fn english_and_mixed_queries_keep_widget_scope_and_visibility() {
    App::test((), |mut app| async move {
        app.update(|ctx| {
            let mut page = search_page();
            page.update_filter("FONT SIZE", ctx);
            assert_eq!(visible_terms(&page), ["terminal font size"]);

            page.update_filter("字体 SIZE", ctx);
            assert_eq!(visible_terms(&page), ["terminal font size"]);

            let match_data = page.update_filter("字体 透明度", ctx);
            assert!(!match_data.is_truthy());
            assert!(visible_terms(&page).is_empty());
        });
    });
}
