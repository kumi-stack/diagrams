pub mod commands;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    App, AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder,
};

pub const MAIN_WINDOW: &str = "main";
pub const QUICK_ADD_WINDOW: &str = "quick-add";
pub const QUICK_ADD_OPENED_EVENT: &str = "quick-add-opened";

const MENU_NEW_DIAGRAM: &str = "new-diagram";
const MENU_SHOW_APP: &str = "show-app";
const MENU_QUIT: &str = "quit";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TrayAction {
    NewDiagram,
    ShowApp,
    Quit,
}

#[derive(Default)]
pub struct AppState {
    current_project: Mutex<Option<String>>,
    exiting: AtomicBool,
}

impl AppState {
    pub fn current_project(&self) -> Option<String> {
        self.current_project
            .lock()
            .ok()
            .and_then(|value| value.clone())
    }

    pub fn set_current_project(&self, project: Option<String>) {
        if let Ok(mut current) = self.current_project.lock() {
            *current = project;
        }
    }

    pub fn is_exiting(&self) -> bool {
        self.exiting.load(Ordering::SeqCst)
    }

    fn begin_exit(&self) {
        self.exiting.store(true, Ordering::SeqCst);
    }
}

pub fn setup(app: &mut App) -> tauri::Result<()> {
    WebviewWindowBuilder::new(app, QUICK_ADD_WINDOW, WebviewUrl::App("quick-add".into()))
        .title("New diagram")
        .inner_size(480.0, 560.0)
        .min_inner_size(420.0, 480.0)
        .resizable(false)
        .minimizable(false)
        .maximizable(false)
        .skip_taskbar(true)
        .visible(false)
        .center()
        .build()?;

    let new_diagram = MenuItemBuilder::with_id(MENU_NEW_DIAGRAM, "New diagram").build(app)?;
    let show_app = MenuItemBuilder::with_id(MENU_SHOW_APP, "Show application").build(app)?;
    let quit = MenuItemBuilder::with_id(MENU_QUIT, "Quit").build(app)?;
    let menu = MenuBuilder::new(app)
        .items(&[&new_diagram, &show_app, &quit])
        .build()?;

    let mut tray = TrayIconBuilder::new()
        .tooltip("Diagram Studio")
        .menu(&menu)
        .on_menu_event(|app, event| {
            if let Some(action) = tray_action(event.id().as_ref()) {
                handle_tray_action(app, action);
            }
        });

    if let Some(icon) = app.default_window_icon() {
        tray = tray.icon(icon.clone());
    }

    tray.build(app)?;
    Ok(())
}

pub fn show_window(app: &AppHandle, label: &str) {
    if let Some(window) = app.get_webview_window(label) {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

pub fn hide_window(app: &AppHandle, label: &str) {
    if let Some(window) = app.get_webview_window(label) {
        let _ = window.hide();
    }
}

fn show_quick_add(app: &AppHandle) {
    show_window(app, QUICK_ADD_WINDOW);
    let _ = app.emit_to(QUICK_ADD_WINDOW, QUICK_ADD_OPENED_EVENT, ());
}

pub fn should_hide_on_close(label: &str, exiting: bool) -> bool {
    !exiting && matches!(label, MAIN_WINDOW | QUICK_ADD_WINDOW)
}

fn tray_action(id: &str) -> Option<TrayAction> {
    match id {
        MENU_NEW_DIAGRAM => Some(TrayAction::NewDiagram),
        MENU_SHOW_APP => Some(TrayAction::ShowApp),
        MENU_QUIT => Some(TrayAction::Quit),
        _ => None,
    }
}

fn handle_tray_action(app: &AppHandle, action: TrayAction) {
    match action {
        TrayAction::NewDiagram => show_quick_add(app),
        TrayAction::ShowApp => show_window(app, MAIN_WINDOW),
        TrayAction::Quit => {
            app.state::<AppState>().begin_exit();
            app.exit(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_last_selected_project() {
        let state = AppState::default();
        assert_eq!(state.current_project(), None);

        state.set_current_project(Some("platform".into()));
        assert_eq!(state.current_project().as_deref(), Some("platform"));

        state.set_current_project(None);
        assert_eq!(state.current_project(), None);
    }

    #[test]
    fn maps_known_tray_menu_ids() {
        assert_eq!(tray_action(MENU_NEW_DIAGRAM), Some(TrayAction::NewDiagram));
        assert_eq!(tray_action(MENU_SHOW_APP), Some(TrayAction::ShowApp));
        assert_eq!(tray_action(MENU_QUIT), Some(TrayAction::Quit));
        assert_eq!(tray_action("unknown"), None);
    }

    #[test]
    fn hides_application_windows_unless_exit_is_in_progress() {
        assert!(should_hide_on_close(MAIN_WINDOW, false));
        assert!(should_hide_on_close(QUICK_ADD_WINDOW, false));
        assert!(!should_hide_on_close("other", false));
        assert!(!should_hide_on_close(MAIN_WINDOW, true));
    }
}
