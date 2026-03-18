use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, Runtime, WebviewWindowBuilder,
};

#[cfg(target_os = "macos")]
use cocoa::appkit::NSWindow;
#[cfg(target_os = "macos")]
use cocoa::base::id;
#[cfg(target_os = "macos")]
use objc::{msg_send, sel, sel_impl};

// Window size - increased width for 3-column layout
const TRAY_WINDOW_WIDTH: f64 = 580.0;
const TRAY_WINDOW_HEIGHT: f64 = 530.0;

pub fn create_tray<R: Runtime>(app: &tauri::App<R>) -> tauri::Result<()> {
    // Create the tray window (hidden by default)
    let window = WebviewWindowBuilder::new(app, "tray-window", tauri::WebviewUrl::App("/".into()))
        .title("Prayer Times")
        .inner_size(TRAY_WINDOW_WIDTH, TRAY_WINDOW_HEIGHT)
        .min_inner_size(TRAY_WINDOW_WIDTH, TRAY_WINDOW_HEIGHT)
        .max_inner_size(TRAY_WINDOW_WIDTH, TRAY_WINDOW_HEIGHT)
        .resizable(false)
        .decorations(false)
        .shadow(true)
        .visible(false)
        .skip_taskbar(true)
        .always_on_top(true)
        .build()?;

    #[cfg(target_os = "macos")]
    {
        unsafe {
            let ns_window = window.ns_window().unwrap() as id;
            ns_window.setOpaque_(false);
            ns_window.setBackgroundColor_(cocoa::appkit::NSColor::clearColor(cocoa::base::nil));

            let content_view: id = ns_window.contentView();
            let _: () = msg_send![content_view, setWantsLayer: true];
            let layer: id = msg_send![content_view, layer];
            let _: () = msg_send![layer, setCornerRadius: 12.0];
            let _: () = msg_send![layer, setMasksToBounds: true];
        }
    }

    // Listen for focus loss to hide window
    let window_clone = window.clone();
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::Focused(false) = event {
            let _ = window_clone.hide();
        }
    });

    let settings_item = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit_item = MenuItem::with_id(app, "quit", "Remove", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&settings_item, &separator, &quit_item])?;

    TrayIconBuilder::with_id("prayer-tray")
        .tooltip("Prayer Times")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "settings" => {
                if let Some(window) = app.get_webview_window("tray-window") {
                    let _ = window.show();
                    let _ = window.set_focus();
                    let _ = window.emit("open-settings", ());
                }
            }
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                rect: tray_rect,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("tray-window") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        position_window_near_tray(&window, &tray_rect);
                        let _ = window.show();
                        let _ = window.set_focus();
                        let _ = window.emit("window-shown", ());
                    }
                }
            }
        })
        .build(app)?;

    Ok(())
}

fn position_window_near_tray<R: Runtime>(
    window: &tauri::WebviewWindow<R>,
    tray_rect: &tauri::Rect,
) {
    use tauri::{LogicalPosition, Position};

    // Get scale factor for converting between physical and logical pixels
    let scale_factor = window.scale_factor().unwrap_or(1.0);

    // Get monitor info
    let Ok(Some(monitor)) = window.current_monitor() else {
        return;
    };

    let monitor_size = monitor.size();
    let monitor_pos = monitor.position();

    // Convert monitor to logical coordinates
    let monitor_x = monitor_pos.x as f64 / scale_factor;
    let monitor_y = monitor_pos.y as f64 / scale_factor;
    let monitor_width = monitor_size.width as f64 / scale_factor;
    let monitor_height = monitor_size.height as f64 / scale_factor;

    // Extract tray position in logical pixels
    let (tray_x, tray_y, tray_width, tray_height) = match (tray_rect.position, tray_rect.size) {
        (Position::Physical(pos), tauri::Size::Physical(size)) => (
            pos.x as f64 / scale_factor,
            pos.y as f64 / scale_factor,
            size.width as f64 / scale_factor,
            size.height as f64 / scale_factor,
        ),
        (Position::Logical(pos), tauri::Size::Logical(size)) => {
            (pos.x, pos.y, size.width, size.height)
        }
        // Mixed cases - convert to logical
        (Position::Physical(pos), tauri::Size::Logical(size)) => (
            pos.x as f64 / scale_factor,
            pos.y as f64 / scale_factor,
            size.width,
            size.height,
        ),
        (Position::Logical(pos), tauri::Size::Physical(size)) => (
            pos.x,
            pos.y,
            size.width as f64 / scale_factor,
            size.height as f64 / scale_factor,
        ),
    };

    // Calculate window position: centered horizontally, below the tray icon
    let x = tray_x + (tray_width / 2.0) - (TRAY_WINDOW_WIDTH / 2.0);
    let y = tray_y + tray_height + 4.0; // Small gap below tray

    // Clamp to screen bounds
    let min_x = monitor_x;
    let max_x = monitor_x + monitor_width - TRAY_WINDOW_WIDTH;
    let min_y = monitor_y;
    let max_y = monitor_y + monitor_height - TRAY_WINDOW_HEIGHT;

    let final_x = x.max(min_x).min(max_x);
    let final_y = y.max(min_y).min(max_y);

    // Set position using logical coordinates
    let _ = window.set_position(Position::Logical(LogicalPosition {
        x: final_x,
        y: final_y,
    }));

    // Also set the size again to ensure it's correct
    let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize {
        width: TRAY_WINDOW_WIDTH,
        height: TRAY_WINDOW_HEIGHT,
    }));
}
