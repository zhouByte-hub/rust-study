use gpui::{px, Styled, ParentElement};
use gpui_component::{h_flex, v_flex, StyledExt, scroll::ScrollableElement};

use crate::{data::{StockDetail, Trade}, theme::TradingTheme};

pub fn render_quote_panel(
    stock: StockDetail,
    trades: Vec<Trade>,
) -> impl gpui::IntoElement {
    gpui::div()
        .w(px(320.0))
        .h_full()
        .bg(TradingTheme::panel_background())
        .border_l_1()
        .border_color(TradingTheme::border())
        .flex()
        .flex_col()
        .child(render_price_display(stock.clone()))
        .child(render_trading_info(stock))
        .child(render_bid_ask_bar())
        .child(render_time_and_sales(trades))
        .child(render_trading_stats())
        .child(render_footer())
}

fn render_price_display(stock: StockDetail) -> impl gpui::IntoElement {
    let is_positive = stock.change >= 0.0;
    let change_color = if is_positive {
        TradingTheme::green()
    } else {
        TradingTheme::red()
    };

    gpui::div()
        .w_full()
        .px(px(20.0))
        .py(px(24.0))
        .border_b_1()
        .border_color(TradingTheme::border())
        .child(
            v_flex()
                .gap(px(12.0))
                .child(
                    gpui::div()
                        .text_2xl()
                        .font_bold()
                        .text_color(TradingTheme::text_primary())
                        .child(format!("{:.3}", stock.price)),
                )
                .child(
                    h_flex()
                        .gap(px(12.0))
                        .items_baseline()
                        .child(
                            gpui::div()
                                .text_lg()
                                .font_semibold()
                                .text_color(change_color)
                                .child(format!(
                                    "{}{:.3}",
                                    if is_positive { "+" } else { "" },
                                    stock.change
                                )),
                        )
                        .child(
                            gpui::div()
                                .text_lg()
                                .font_semibold()
                                .text_color(change_color)
                                .child(format!(
                                    "({}{:.2}%)",
                                    if is_positive { "+" } else { "" },
                                    stock.change_percent
                                )),
                        ),
                ),
        )
}

fn render_trading_info(stock: StockDetail) -> impl gpui::IntoElement {
    let items = vec![
        ("开盘价", format!("{:.2}", stock.open)),
        ("最高价", format!("{:.2}", stock.high)),
        ("最低价", format!("{:.2}", stock.low)),
        ("前收盘", format!("{:.2}", stock.prev_close)),
        ("成交量", format!("{:.2}M", stock.volume as f64 / 1_000_000.0)),
        ("成交额", format!("{:.2}B", stock.amount / 1_000_000_000.0)),
    ];

    gpui::div()
        .w_full()
        .px(px(20.0))
        .py(px(16.0))
        .border_b_1()
        .border_color(TradingTheme::border())
        .child(
            v_flex()
                .w_full()
                .gap(px(12.0))
                .children(items.into_iter().map(|(label, value)| {
                    h_flex()
                        .w_full()
                        .justify_between()
                        .items_center()
                        .child(
                            gpui::div()
                                .text_xs()
                                .text_color(TradingTheme::text_muted())
                                .child(label),
                        )
                        .child(
                            gpui::div()
                                .text_xs()
                                .font_semibold()
                                .text_color(TradingTheme::text_primary())
                                .child(value),
                        )
                })),
        )
}

fn render_bid_ask_bar() -> impl gpui::IntoElement {
    gpui::div()
        .w_full()
        .px(px(20.0))
        .py(px(16.0))
        .border_b_1()
        .border_color(TradingTheme::border())
        .child(
            v_flex()
                .w_full()
                .gap(px(12.0))
                .child(
                    gpui::div()
                        .text_xs()
                        .font_semibold()
                        .text_color(TradingTheme::text_secondary())
                        .child("买卖价差"),
                )
                .child(
                    h_flex()
                        .w_full()
                        .h(px(32.0))
                        .rounded_md()
                        .overflow_hidden()
                        .child(
                            gpui::div()
                                .w(px(140.0))
                                .h_full()
                                .bg(TradingTheme::green())
                                .opacity(0.3)
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(
                                    gpui::div()
                                        .text_sm()
                                        .font_semibold()
                                        .text_color(TradingTheme::green())
                                        .child("买 192.58"),
                                ),
                        )
                        .child(
                            gpui::div()
                                .w(px(140.0))
                                .h_full()
                                .bg(TradingTheme::red())
                                .opacity(0.3)
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(
                                    gpui::div()
                                        .text_sm()
                                        .font_semibold()
                                        .text_color(TradingTheme::red())
                                        .child("卖 192.62"),
                                ),
                        ),
                ),
        )
}

fn render_time_and_sales(trades: Vec<Trade>) -> impl gpui::IntoElement {
    gpui::div()
        .w_full()
        .flex_1()
        .px(px(20.0))
        .py(px(16.0))
        .border_b_1()
        .border_color(TradingTheme::border())
        .child(
            v_flex()
                .w_full()
                .h_full()
                .gap(px(12.0))
                .child(
                    gpui::div()
                        .text_xs()
                        .font_semibold()
                        .text_color(TradingTheme::text_secondary())
                        .child("时间与销售 (日内)"),
                )
                .child(
                    gpui::div()
                        .flex_1()
                        .overflow_y_scrollbar()
                        .child(
                            v_flex()
                                .w_full()
                                .gap(px(8.0))
                                .children(trades.into_iter().map(|trade| {
                                    render_trade_item(trade)
                                })),
                        ),
                ),
        )
}

fn render_trade_item(trade: Trade) -> impl gpui::IntoElement {
    let is_buy = trade.direction == "买入";
    let direction_color = if is_buy {
        TradingTheme::green()
    } else {
        TradingTheme::red()
    };

    h_flex()
        .w_full()
        .gap(px(12.0))
        .items_center()
        .child(
            gpui::div()
                .w(px(8.0))
                .h(px(8.0))
                .rounded_full()
                .bg(direction_color),
        )
        .child(
            gpui::div()
                .w(px(60.0))
                .text_xs()
                .font_medium()
                .text_color(TradingTheme::text_muted())
                .child(trade.time),
        )
        .child(
            gpui::div()
                .w(px(60.0))
                .text_xs()
                .font_semibold()
                .text_color(TradingTheme::text_primary())
                .child(format!("{:.2}", trade.price)),
        )
        .child(
            gpui::div()
                .flex_1()
                .text_xs()
                .text_color(TradingTheme::text_secondary())
                .child(format!("{}", trade.volume)),
        )
}

fn render_trading_stats() -> impl gpui::IntoElement {
    gpui::div()
        .w_full()
        .px(px(20.0))
        .py(px(16.0))
        .border_b_1()
        .border_color(TradingTheme::border())
        .child(
            v_flex()
                .w_full()
                .gap(px(12.0))
                .child(
                    h_flex()
                        .w_full()
                        .justify_between()
                        .items_center()
                        .child(
                            gpui::div()
                                .text_xs()
                                .font_semibold()
                                .text_color(TradingTheme::text_secondary())
                                .child("交易统计"),
                        )
                        .child(
                            gpui::div()
                                .text_xs()
                                .text_color(TradingTheme::text_muted())
                                .child("单位: 10K"),
                        ),
                )
                .child(
                    gpui::div()
                        .w_full()
                        .h(px(120.0))
                        .bg(TradingTheme::card_background())
                        .rounded_md()
                        .border_1()
                        .border_color(TradingTheme::border())
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            gpui::div()
                                .text_sm()
                                .text_color(TradingTheme::text_muted())
                                .child("净流出 -2012.31"),
                        ),
                ),
        )
}

fn render_footer() -> impl gpui::IntoElement {
    gpui::div()
        .w_full()
        .h(px(48.0))
        .px(px(20.0))
        .flex()
        .items_center()
        .justify_between()
        .child(
            h_flex()
                .gap(px(8.0))
                .items_center()
                .child(
                    gpui::div()
                        .text_xs()
                        .text_color(TradingTheme::text_muted())
                        .child("🔄"),
                )
                .child(
                    gpui::div()
                        .text_xs()
                        .text_color(TradingTheme::text_muted())
                        .child("2025/10/10 21:34:47"),
                ),
        )
}
