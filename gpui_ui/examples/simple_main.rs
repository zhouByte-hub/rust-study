use gpui::{AppContext, Application, SharedString, TitlebarOptions};
use gpui_component::{Root, theme::Theme};

mod simple_search;

use simple_search::SimpleApp;

fn main() {
    Application::new().run(|cx| {
        cx.set_global(Theme::default());
        let mut option = gpui::WindowOptions::default();
        let mut titlebar = TitlebarOptions::default();
        titlebar.title = Some(SharedString::new("简单搜索示例"));
        option.titlebar = Some(titlebar);
        
        cx.open_window(option, |window, cx| {
            cx.new(|cx| {
                let app = cx.new(|_cx| SimpleApp::new());
                Root::new(app, window, cx)
            })
        })
        .unwrap();
    });
}
