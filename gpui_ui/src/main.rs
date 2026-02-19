use gpui::{Application, AppContext};
use gpui_component::{Root, theme::Theme};

mod data;
mod theme;
mod components;
mod dashboard;

use dashboard::TradingPlatform;

fn main() {
    Application::new().run(|cx| {
        cx.set_global(Theme::default());
        
        cx.open_window(gpui::WindowOptions::default(), |window, cx| {
            cx.new(|cx| {
                let platform = cx.new(|_cx| TradingPlatform::new());
                Root::new(platform, window, cx)
            })
        })
        .unwrap();
    });
}
