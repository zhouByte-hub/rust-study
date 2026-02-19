use gpui::{Context, Render, Window, Styled, ParentElement};
use gpui_component::h_flex;

use crate::data::TradingData;
use crate::theme::TradingTheme;
use crate::components::{
    render_watchlist,
    render_chart_panel,
    render_quote_panel,
    render_sidebar,
    render_header,
};

pub struct TradingPlatform {
    pub data: TradingData,
}

impl TradingPlatform {
    pub fn new() -> Self {
        Self {
            data: TradingData::new(),
        }
    }
}

impl Render for TradingPlatform {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        gpui::div()
            .size_full()
            .bg(TradingTheme::background())
            .flex()
            .flex_col()
            .child(render_header())
            .child(
                h_flex()
                    .flex_1()
                    .w_full()
                    .child(render_sidebar())
                    .child(render_watchlist(
                        self.data.stocks.clone(),
                        self.data.market_indices.clone(),
                        &self.data.selected_stock.symbol,
                    ))
                    .child(render_chart_panel(
                        self.data.selected_stock.clone(),
                        self.data.candle_data.clone(),
                        self.data.orders.clone(),
                    ))
                    .child(render_quote_panel(
                        self.data.selected_stock.clone(),
                        self.data.trades.clone(),
                    )),
            )
    }
}
