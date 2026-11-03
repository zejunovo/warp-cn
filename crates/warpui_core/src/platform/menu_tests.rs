use super::Menu;

#[test]
fn system_window_menu_identity_is_independent_of_its_translated_title() {
    for title in ["Window", "窗口"] {
        let menu = Menu::new_window(title, vec![]);
        assert!(menu.is_window_menu());
        assert_eq!(menu.title, title);
    }
}

#[test]
fn ordinary_menus_do_not_become_system_window_menus_by_name() {
    for title in ["File", "Window", "窗口"] {
        assert!(!Menu::new(title, vec![]).is_window_menu());
    }
}

#[test]
fn changing_the_display_title_preserves_system_window_menu_identity() {
    let mut menu = Menu::new_window("Window", vec![]);
    menu.title = "窗口".to_owned();
    assert!(menu.is_window_menu());
}
