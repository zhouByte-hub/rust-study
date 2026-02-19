use gpui::{AppContext, Application, SharedString, TitlebarOptions};
use gpui_component::{Root, theme::Theme};

mod data;
mod theme;
mod components;
mod dashboard;

use dashboard::TradingPlatform;

fn main() {
    Application::new().run(|cx| {
        cx.set_global(Theme::default());
        let mut option = gpui::WindowOptions::default();
        let mut titlebar = TitlebarOptions::default();
        titlebar.title = Some(SharedString::new("Gpui Demo"));
        option.titlebar = Some(titlebar);
        
        cx.open_window(option, |window, cx| {
            cx.new(|cx| {
                let platform = cx.new(|_cx| TradingPlatform::new());
                Root::new(platform, window, cx)
            })
        })
        .unwrap();
    });
}
