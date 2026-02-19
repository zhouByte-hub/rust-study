use gpui::{px, Styled, ParentElement};
use gpui_component::h_flex;

use crate::theme::TradingTheme;

pub fn render_header() -> impl gpui::IntoElement {
    gpui::div()
        .w_full()
        .h(px(48.0))
        .bg(TradingTheme::panel_background())
        .border_b_1()
        .border_color(TradingTheme::border())
        .flex()
        .items_center()
        .justify_between()
        .px(px(16.0))
        .child(render_window_controls())
        .child(render_search_bar())
        .child(render_right_controls())
}

fn render_window_controls() -> impl gpui::IntoElement {
    h_flex()
        .gap(px(8.0))
        .items_center()
        .child(
            gpui::div()
                .w(px(12.0))
                .h(px(12.0))
                .rounded_full()
                .bg(TradingTheme::red()),
        )
        .child(
            gpui::div()
                .w(px(12.0))
                .h(px(12.0))
                .rounded_full()
                .bg(TradingTheme::yellow()),
        )
        .child(
            gpui::div()
                .w(px(12.0))
                .h(px(12.0))
                .rounded_full()
                .bg(TradingTheme::green()),
        )
}

fn render_search_bar() -> impl gpui::IntoElement {
    gpui::div()
        .w(px(400.0))
        .h(px(32.0))
        .bg(TradingTheme::card_background())
        .border_1()
        .border_color(TradingTheme::border())
        .rounded_md()
        .px(px(12.0))
        .flex()
        .items_center()
        .gap(px(8.0))
        .child(
            gpui::div()
                .text_sm()
                .text_color(TradingTheme::text_muted())
                .child("🔍"),
        )
        .child(
            gpui::div()
                .text_sm()
                .text_color(TradingTheme::text_muted())
                .child("输入 / 进行搜索"),
        )
}

fn render_right_controls() -> impl gpui::IntoElement {
    h_flex()
        .gap(px(16.0))
        .items_center()
        .child(
            gpui::div()
                .px(px(12.0))
                .py(px(6.0))
                .rounded_md()
                .border_1()
                .border_color(TradingTheme::border())
                .child(
                    gpui::div()
                        .text_xs()
                        .text_color(TradingTheme::text_secondary())
                        .child("预览模式"),
                ),
        )
        .child(
            gpui::div()
                .w(px(32.0))
                .h(px(32.0))
                .rounded_full()
                .bg(TradingTheme::blue())
                .flex()
                .items_center()
                .justify_center()
                .child(
                    gpui::div()
                        .text_sm()
                        .child("🔔"),
                ),
        )
}
