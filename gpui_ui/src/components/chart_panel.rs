use gpui::{px, Styled, ParentElement};
use gpui_component::{h_flex, v_flex, StyledExt, scroll::ScrollableElement};

use crate::{data::{CandleData, Order, StockDetail}, theme::TradingTheme};

pub fn render_chart_panel(
    stock: StockDetail,
    candle_data: Vec<CandleData>,
    orders: Vec<Order>,
) -> impl gpui::IntoElement {
    gpui::div()
        .flex_1()
        .h_full()
        .bg(TradingTheme::panel_background())
        .flex()
        .flex_col()
        .child(render_stock_header(stock))
        .child(render_timeframe_selector())
        .child(render_chart_area(candle_data))
        .child(render_orders_panel(orders))
}

fn render_stock_header(stock: StockDetail) -> impl gpui::IntoElement {
    let is_positive = stock.change >= 0.0;
    let change_color = if is_positive {
        TradingTheme::green()
    } else {
        TradingTheme::red()
    };

    gpui::div()
        .w_full()
        .h(px(60.0))
        .px(px(20.0))
        .flex()
        .items_center()
        .justify_between()
        .border_b_1()
        .border_color(TradingTheme::border())
        .child(
            h_flex()
                .gap(px(16.0))
                .items_baseline()
                .child(
                    gpui::div()
                        .text_xl()
                        .font_bold()
                        .text_color(TradingTheme::text_primary())
                        .child(format!("{}.US", stock.symbol)),
                )
                .child(
                    gpui::div()
                        .text_2xl()
                        .font_bold()
                        .text_color(TradingTheme::text_primary())
                        .child(format!("{:.3}", stock.price)),
                )
                .child(
                    h_flex()
                        .gap(px(8.0))
                        .items_center()
                        .child(
                            gpui::div()
                                .text_sm()
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
                                .text_sm()
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
        .child(
            h_flex()
                .gap(px(24.0))
                .items_center()
                .child(
                    v_flex()
                        .gap(px(4.0))
                        .child(
                            gpui::div()
                                .text_xs()
                                .text_color(TradingTheme::text_muted())
                                .child("买价"),
                        )
                        .child(
                            gpui::div()
                                .text_sm()
                                .font_semibold()
                                .text_color(TradingTheme::green())
                                .child(format!("{:.2}", stock.bid_price)),
                        ),
                )
                .child(
                    v_flex()
                        .gap(px(4.0))
                        .child(
                            gpui::div()
                                .text_xs()
                                .text_color(TradingTheme::text_muted())
                                .child("卖价"),
                        )
                        .child(
                            gpui::div()
                                .text_sm()
                                .font_semibold()
                                .text_color(TradingTheme::red())
                                .child(format!("{:.2}", stock.ask_price)),
                        ),
                ),
        )
}

fn render_timeframe_selector() -> impl gpui::IntoElement {
    let timeframes = vec![
        "日内", "5D", "W", "M", "Q", "Y", "5分钟", "15分钟", "30分钟",
    ];

    gpui::div()
        .w_full()
        .h(px(40.0))
        .px(px(20.0))
        .flex()
        .items_center()
        .gap(px(8.0))
        .border_b_1()
        .border_color(TradingTheme::border())
        .children(timeframes.into_iter().map(|tf| {
            gpui::div()
                .px(px(12.0))
                .py(px(6.0))
                .rounded_md()
                .bg(if tf == "日内" {
                    TradingTheme::blue()
                } else {
                    TradingTheme::card_background()
                })
                .child(
                    gpui::div()
                        .text_xs()
                        .font_medium()
                        .text_color(if tf == "日内" {
                            TradingTheme::text_primary()
                        } else {
                            TradingTheme::text_secondary()
                        })
                        .child(tf),
                )
        }))
}

fn render_chart_area(candle_data: Vec<CandleData>) -> impl gpui::IntoElement {
    gpui::div()
        .flex_1()
        .w_full()
        .px(px(20.0))
        .py(px(16.0))
        .child(
            gpui::div()
                .w_full()
                .h_full()
                .bg(TradingTheme::card_background())
                .rounded_lg()
                .border_1()
                .border_color(TradingTheme::border())
                .p(px(16.0))
                .child(
                    v_flex()
                        .w_full()
                        .h_full()
                        .gap(px(16.0))
                        .child(render_chart_placeholder())
                        .child(render_volume_bars(candle_data)),
                ),
        )
}

fn render_chart_placeholder() -> impl gpui::IntoElement {
    gpui::div()
        .flex_1()
        .w_full()
        .flex()
        .items_center()
        .justify_center()
        .child(
            gpui::div()
                .text_sm()
                .text_color(TradingTheme::text_muted())
                .child("K线图区域 (需要图表库支持)"),
        )
}

fn render_volume_bars(candle_data: Vec<CandleData>) -> impl gpui::IntoElement {
    let max_volume = candle_data
        .iter()
        .map(|d| d.volume)
        .fold(0.0_f64, |a, b| a.max(b));

    gpui::div()
        .w_full()
        .h(px(80.0))
        .flex()
        .items_end()
        .gap(px(2.0))
        .children(candle_data.into_iter().map(|candle| {
            let height = (candle.volume / max_volume * 80.0) as f32;
            let is_positive = candle.close >= candle.open;
            let color = if is_positive {
                TradingTheme::green()
            } else {
                TradingTheme::red()
            };

            gpui::div()
                .flex_1()
                .h(px(height))
                .bg(color)
                .opacity(0.6)
        }))
}

fn render_orders_panel(orders: Vec<Order>) -> impl gpui::IntoElement {
    gpui::div()
        .w_full()
        .h(px(240.0))
        .border_t_1()
        .border_color(TradingTheme::border())
        .child(
            v_flex()
                .w_full()
                .h_full()
                .child(render_orders_header())
                .child(render_orders_table(orders)),
        )
}

fn render_orders_header() -> impl gpui::IntoElement {
    gpui::div()
        .w_full()
        .h(px(40.0))
        .px(px(20.0))
        .flex()
        .items_center()
        .justify_between()
        .border_b_1()
        .border_color(TradingTheme::border())
        .child(
            h_flex()
                .gap(px(16.0))
                .items_center()
                .child(
                    gpui::div()
                        .text_sm()
                        .font_semibold()
                        .text_color(TradingTheme::text_primary())
                        .child("今日订单"),
                )
                .child(
                    gpui::div()
                        .text_sm()
                        .text_color(TradingTheme::text_muted())
                        .child("历史订单"),
                )
                .child(
                    gpui::div()
                        .text_sm()
                        .text_color(TradingTheme::text_muted())
                        .child("持仓"),
                ),
        )
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
                        .child("筛选 ▼"),
                ),
        )
}

fn render_orders_table(orders: Vec<Order>) -> impl gpui::IntoElement {
    gpui::div()
        .flex_1()
        .w_full()
        .px(px(20.0))
        .overflow_x_scrollbar()
        .child(
            v_flex()
                .w_full()
                .gap(px(8.0))
                .child(render_table_header())
                .children(orders.into_iter().map(|order| {
                    render_order_row(order)
                })),
        )
}

fn render_table_header() -> impl gpui::IntoElement {
    h_flex()
        .w_full()
        .gap(px(16.0))
        .px(px(12.0))
        .py(px(8.0))
        .bg(TradingTheme::card_background())
        .rounded_md()
        .child(gpui::div().w(px(60.0)).text_xs().font_semibold().text_color(TradingTheme::text_muted()).child("操作"))
        .child(gpui::div().w(px(60.0)).text_xs().font_semibold().text_color(TradingTheme::text_muted()).child("代码"))
        .child(gpui::div().w(px(80.0)).text_xs().font_semibold().text_color(TradingTheme::text_muted()).child("名称"))
        .child(gpui::div().w(px(80.0)).text_xs().font_semibold().text_color(TradingTheme::text_muted()).child("状态"))
        .child(gpui::div().w(px(60.0)).text_xs().font_semibold().text_color(TradingTheme::text_muted()).child("方向"))
        .child(gpui::div().w(px(80.0)).text_xs().font_semibold().text_color(TradingTheme::text_muted()).child("类型"))
        .child(gpui::div().w(px(60.0)).text_xs().font_semibold().text_color(TradingTheme::text_muted()).child("数量"))
        .child(gpui::div().w(px(80.0)).text_xs().font_semibold().text_color(TradingTheme::text_muted()).child("限价"))
}

fn render_order_row(order: Order) -> impl gpui::IntoElement {
    let action_color = if order.action == "买入" {
        TradingTheme::green()
    } else {
        TradingTheme::red()
    };

    let status_color = match order.status.as_str() {
        "已成交" => TradingTheme::status_success(),
        "待成交" => TradingTheme::status_warning(),
        "已撤单" => TradingTheme::status_error(),
        _ => TradingTheme::text_muted(),
    };

    h_flex()
        .w_full()
        .gap(px(16.0))
        .px(px(12.0))
        .py(px(8.0))
        .border_b_1()
        .border_color(TradingTheme::divider())
        .child(gpui::div().w(px(60.0)).text_xs().font_semibold().text_color(action_color).child(order.action))
        .child(gpui::div().w(px(60.0)).text_xs().text_color(TradingTheme::text_primary()).child(order.symbol))
        .child(gpui::div().w(px(80.0)).text_xs().text_color(TradingTheme::text_primary()).child(order.name))
        .child(gpui::div().w(px(80.0)).text_xs().font_medium().text_color(status_color).child(order.status))
        .child(gpui::div().w(px(60.0)).text_xs().text_color(TradingTheme::text_secondary()).child(order.direction))
        .child(gpui::div().w(px(80.0)).text_xs().text_color(TradingTheme::text_secondary()).child(order.order_type))
        .child(gpui::div().w(px(60.0)).text_xs().text_color(TradingTheme::text_primary()).child(format!("{}", order.quantity)))
        .child(gpui::div().w(px(80.0)).text_xs().text_color(TradingTheme::text_primary()).child(
            order.limit_price.map(|p| format!("{:.2}", p)).unwrap_or_else(|| "-".to_string())
        ))
}
