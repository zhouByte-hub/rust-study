use gpui::{px, Styled, ParentElement, prelude::FluentBuilder};

use crate::theme::TradingTheme;

pub fn render_sidebar() -> impl gpui::IntoElement {
    let icons = vec![
        ("📋".to_string(), "关注列表".to_string(), true),
        ("❤️".to_string(), "收藏".to_string(), false),
        ("🧭".to_string(), "发现".to_string(), false),
        ("🕐".to_string(), "历史".to_string(), false),
        ("🚗".to_string(), "".to_string(), false),
        ("💚".to_string(), "".to_string(), false),
        ("🍎".to_string(), "".to_string(), false),
        ("💬".to_string(), "".to_string(), false),
    ];

    gpui::div()
        .w(px(64.0))
        .h_full()
        .bg(TradingTheme::panel_background())
        .border_r_1()
        .border_color(TradingTheme::border())
        .flex()
        .flex_col()
        .child(render_sidebar_top(icons))
        .child(render_sidebar_bottom())
}

fn render_sidebar_top(icons: Vec<(String, String, bool)>) -> impl gpui::IntoElement {
    gpui::div()
        .flex_1()
        .w_full()
        .py(px(16.0))
        .flex()
        .flex_col()
        .gap(px(8.0))
        .children(icons.into_iter().map(|(icon, label, active)| {
            render_sidebar_icon(icon, label, active)
        }))
}

fn render_sidebar_icon(icon: String, label: String, active: bool) -> impl gpui::IntoElement {
    gpui::div()
        .w_full()
        .h(px(48.0))
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(4.0))
        .bg(if active {
            TradingTheme::hover_background()
        } else {
            TradingTheme::panel_background()
        })
        .border_l_2()
        .border_color(if active {
            TradingTheme::blue()
        } else {
            TradingTheme::panel_background()
        })
        .child(
            gpui::div()
                .text_xl()
                .child(icon),
        )
        .when(!label.is_empty(), |this| {
            this.child(
                gpui::div()
                    .text_xs()
                    .text_color(if active {
                        TradingTheme::text_primary()
                    } else {
                        TradingTheme::text_muted()
                    })
                    .child(label),
            )
        })
}

fn render_sidebar_bottom() -> impl gpui::IntoElement {
    gpui::div()
        .w_full()
        .h(px(64.0))
        .flex()
        .items_center()
        .justify_center()
        .border_t_1()
        .border_color(TradingTheme::border())
        .child(
            gpui::div()
                .w(px(40.0))
                .h(px(40.0))
                .rounded_full()
                .bg(TradingTheme::card_background())
                .border_1()
                .border_color(TradingTheme::border())
                .flex()
                .items_center()
                .justify_center()
                .child(
                    gpui::div()
                        .text_lg()
                        .child("👤"),
                ),
        )
}
