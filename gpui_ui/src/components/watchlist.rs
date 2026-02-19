use gpui::{px, Styled, ParentElement, InteractiveElement};
use gpui_component::{h_flex, v_flex, StyledExt, scroll::ScrollableElement};

use crate::{data::{Stock, MarketIndex}, theme::TradingTheme};

pub fn render_watchlist(
    stocks: Vec<Stock>,
    market_indices: Vec<MarketIndex>,
    selected_symbol: &str,
) -> impl gpui::IntoElement {
    gpui::div()
        .w(px(280.0))
        .h_full()
        .bg(TradingTheme::panel_background())
        .border_r_1()
        .border_color(TradingTheme::border())
        .flex()
        .flex_col()
        .child(render_watchlist_header())
        .child(render_filter_tabs())
        .child(render_stock_list(stocks, selected_symbol))
        .child(render_market_indices(market_indices))
}

fn render_watchlist_header() -> impl gpui::IntoElement {
    gpui::div()
        .w_full()
        .h(px(48.0))
        .px(px(16.0))
        .flex()
        .items_center()
        .border_b_1()
        .border_color(TradingTheme::border())
        .child(
            gpui::div()
                .text_base()
                .font_semibold()
                .text_color(TradingTheme::text_primary())
                .child("关注列表"),
        )
}

fn render_filter_tabs() -> impl gpui::IntoElement {
    let tabs = vec!["全部", "EV", "经典", "持有"];
    
    gpui::div()
        .w_full()
        .h(px(40.0))
        .px(px(12.0))
        .flex()
        .items_center()
        .gap(px(8.0))
        .border_b_1()
        .border_color(TradingTheme::border())
        .children(tabs.into_iter().map(|tab| {
            gpui::div()
                .px(px(12.0))
                .py(px(6.0))
                .rounded_md()
                .bg(if tab == "全部" {
                    TradingTheme::blue()
                } else {
                    TradingTheme::card_background()
                })
                .child(
                    gpui::div()
                        .text_xs()
                        .font_medium()
                        .text_color(if tab == "全部" {
                            TradingTheme::text_primary()
                        } else {
                            TradingTheme::text_secondary()
                        })
                        .child(tab),
                )
        }))
}

fn render_stock_list(
    stocks: Vec<Stock>,
    selected_symbol: &str,
) -> impl gpui::IntoElement {
    gpui::div()
        .flex_1()
        .overflow_y_scrollbar()
        .child(
            v_flex()
                .w_full()
                .children(stocks.into_iter().map(|stock| {
                    render_stock_item(stock, selected_symbol)
                })),
        )
}

fn render_stock_item(
    stock: Stock,
    selected_symbol: &str,
) -> impl gpui::IntoElement {
    let is_selected = stock.symbol == selected_symbol;
    let is_positive = stock.change >= 0.0;
    let change_color = if is_positive {
        TradingTheme::green()
    } else {
        TradingTheme::red()
    };

    gpui::div()
        .w_full()
        .h(px(64.0))
        .px(px(16.0))
        .py(px(12.0))
        .bg(if is_selected {
            TradingTheme::selected_background()
        } else {
            TradingTheme::panel_background()
        })
        .border_b_1()
        .border_color(TradingTheme::divider())
        .hover(|style| style.bg(TradingTheme::hover_background()))
        .child(
            h_flex()
                .w_full()
                .h_full()
                .gap(px(12.0))
                .items_center()
                .child(
                    gpui::div()
                        .w(px(32.0))
                        .h(px(32.0))
                        .rounded_md()
                        .bg(TradingTheme::card_background())
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            gpui::div()
                                .text_lg()
                                .child(stock.icon),
                        ),
                )
                .child(
                    v_flex()
                        .flex_1()
                        .gap(px(4.0))
                        .child(
                            gpui::div()
                                .text_sm()
                                .font_semibold()
                                .text_color(TradingTheme::text_primary())
                                .child(stock.name),
                        )
                        .child(
                            gpui::div()
                                .text_xs()
                                .text_color(TradingTheme::text_muted())
                                .child(stock.symbol),
                        ),
                )
                .child(
                    v_flex()
                        .items_end()
                        .gap(px(4.0))
                        .child(
                            gpui::div()
                                .text_sm()
                                .font_semibold()
                                .text_color(TradingTheme::text_primary())
                                .child(format!("{:.2}", stock.price)),
                        )
                        .child(
                            gpui::div()
                                .text_xs()
                                .font_medium()
                                .text_color(change_color)
                                .child(format!(
                                    "{}{:.2}%",
                                    if is_positive { "+" } else { "" },
                                    stock.change_percent
                                )),
                        ),
                ),
        )
}

fn render_market_indices(
    market_indices: Vec<MarketIndex>,
) -> impl gpui::IntoElement {
    gpui::div()
        .w_full()
        .border_t_1()
        .border_color(TradingTheme::border())
        .child(
            v_flex()
                .w_full()
                .px(px(16.0))
                .py(px(12.0))
                .gap(px(12.0))
                .children(market_indices.into_iter().map(|index| {
                    render_market_index(index)
                })),
        )
}

fn render_market_index(index: MarketIndex) -> impl gpui::IntoElement {
    let is_positive = index.change >= 0.0;
    let change_color = if is_positive {
        TradingTheme::green()
    } else {
        TradingTheme::red()
    };

    h_flex()
        .w_full()
        .justify_between()
        .items_center()
        .child(
            gpui::div()
                .text_xs()
                .text_color(TradingTheme::text_secondary())
                .child(index.name),
        )
        .child(
            h_flex()
                .gap(px(8.0))
                .items_center()
                .child(
                    gpui::div()
                        .text_xs()
                        .font_semibold()
                        .text_color(TradingTheme::text_primary())
                        .child(format!("{:.2}", index.value)),
                )
                .child(
                    gpui::div()
                        .text_xs()
                        .font_medium()
                        .text_color(change_color)
                        .child(format!(
                            "{}{:.2}%",
                            if is_positive { "+" } else { "" },
                            index.change_percent
                        )),
                ),
        )
}
