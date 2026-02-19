#[derive(Clone, Debug)]
pub struct Stock {
    pub symbol: String,
    pub name: String,
    pub price: f64,
    pub change: f64,
    pub change_percent: f64,
    pub icon: String,
}

#[derive(Clone, Debug)]
pub struct MarketIndex {
    pub name: String,
    pub value: f64,
    pub change: f64,
    pub change_percent: f64,
}

#[derive(Clone, Debug)]
pub struct CandleData {
    pub open: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Clone, Debug)]
pub struct Order {
    pub action: String,
    pub symbol: String,
    pub name: String,
    pub status: String,
    pub direction: String,
    pub order_type: String,
    pub quantity: u32,
    pub limit_price: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct Trade {
    pub time: String,
    pub price: f64,
    pub volume: u32,
    pub direction: String,
}

#[derive(Clone, Debug)]
pub struct StockDetail {
    pub symbol: String,
    pub price: f64,
    pub change: f64,
    pub change_percent: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub prev_close: f64,
    pub volume: u64,
    pub amount: f64,
    pub bid_price: f64,
    pub ask_price: f64,
}

pub struct TradingData {
    pub stocks: Vec<Stock>,
    pub market_indices: Vec<MarketIndex>,
    pub candle_data: Vec<CandleData>,
    pub orders: Vec<Order>,
    pub trades: Vec<Trade>,
    pub selected_stock: StockDetail,
}

impl TradingData {
    pub fn new() -> Self {
        Self {
            stocks: vec![
                Stock {
                    symbol: "TSLA".to_string(),
                    name: "特斯拉".to_string(),
                    price: 248.50,
                    change: 5.30,
                    change_percent: 2.18,
                    icon: "🚗".to_string(),
                },
                Stock {
                    symbol: "AAPL".to_string(),
                    name: "苹果".to_string(),
                    price: 178.72,
                    change: -1.28,
                    change_percent: -0.71,
                    icon: "🍎".to_string(),
                },
                Stock {
                    symbol: "NVDA".to_string(),
                    name: "英伟达".to_string(),
                    price: 192.60,
                    change: 0.03,
                    change_percent: 0.02,
                    icon: "💚".to_string(),
                },
                Stock {
                    symbol: "MSFT".to_string(),
                    name: "微软".to_string(),
                    price: 378.91,
                    change: 2.45,
                    change_percent: 0.65,
                    icon: "💻".to_string(),
                },
                Stock {
                    symbol: "GOOGL".to_string(),
                    name: "谷歌".to_string(),
                    price: 141.80,
                    change: -0.92,
                    change_percent: -0.64,
                    icon: "🔍".to_string(),
                },
                Stock {
                    symbol: "AMZN".to_string(),
                    name: "亚马逊".to_string(),
                    price: 178.25,
                    change: 1.75,
                    change_percent: 0.99,
                    icon: "📦".to_string(),
                },
            ],
            market_indices: vec![
                MarketIndex {
                    name: "道琼斯".to_string(),
                    value: 38675.68,
                    change: 231.50,
                    change_percent: 0.60,
                },
                MarketIndex {
                    name: "纳斯达克".to_string(),
                    value: 16156.33,
                    change: -45.20,
                    change_percent: -0.28,
                },
                MarketIndex {
                    name: "标普500".to_string(),
                    value: 5123.41,
                    change: 12.80,
                    change_percent: 0.25,
                },
            ],
            candle_data: vec![
                CandleData {
                    open: 192.50,
                    close: 192.80,
                    volume: 1250000.0,
                },
                CandleData {
                    open: 192.80,
                    close: 193.20,
                    volume: 980000.0,
                },
                CandleData {
                    open: 193.20,
                    close: 193.50,
                    volume: 1120000.0,
                },
                CandleData {
                    open: 193.50,
                    close: 193.80,
                    volume: 890000.0,
                },
                CandleData {
                    open: 193.80,
                    close: 193.40,
                    volume: 750000.0,
                },
                CandleData {
                    open: 193.40,
                    close: 193.10,
                    volume: 680000.0,
                },
                CandleData {
                    open: 193.10,
                    close: 192.90,
                    volume: 820000.0,
                },
                CandleData {
                    open: 192.90,
                    close: 192.70,
                    volume: 910000.0,
                },
            ],
            orders: vec![
                Order {
                    action: "买入".to_string(),
                    symbol: "NVDA".to_string(),
                    name: "英伟达".to_string(),
                    status: "已成交".to_string(),
                    direction: "买入".to_string(),
                    order_type: "限价单".to_string(),
                    quantity: 100,
                    limit_price: Some(192.50),
                },
                Order {
                    action: "卖出".to_string(),
                    symbol: "AAPL".to_string(),
                    name: "苹果".to_string(),
                    status: "待成交".to_string(),
                    direction: "卖出".to_string(),
                    order_type: "限价单".to_string(),
                    quantity: 50,
                    limit_price: Some(179.50),
                },
                Order {
                    action: "买入".to_string(),
                    symbol: "TSLA".to_string(),
                    name: "特斯拉".to_string(),
                    status: "已撤单".to_string(),
                    direction: "买入".to_string(),
                    order_type: "市价单".to_string(),
                    quantity: 25,
                    limit_price: None,
                },
                Order {
                    action: "买入".to_string(),
                    symbol: "MSFT".to_string(),
                    name: "微软".to_string(),
                    status: "已成交".to_string(),
                    direction: "买入".to_string(),
                    order_type: "限价单".to_string(),
                    quantity: 75,
                    limit_price: Some(378.00),
                },
            ],
            trades: vec![
                Trade {
                    time: "14:32:15".to_string(),
                    price: 192.60,
                    volume: 500,
                    direction: "买入".to_string(),
                },
                Trade {
                    time: "14:31:48".to_string(),
                    price: 192.58,
                    volume: 1200,
                    direction: "卖出".to_string(),
                },
                Trade {
                    time: "14:30:22".to_string(),
                    price: 192.62,
                    volume: 800,
                    direction: "买入".to_string(),
                },
                Trade {
                    time: "14:29:55".to_string(),
                    price: 192.55,
                    volume: 1500,
                    direction: "买入".to_string(),
                },
                Trade {
                    time: "14:28:18".to_string(),
                    price: 192.50,
                    volume: 2000,
                    direction: "卖出".to_string(),
                },
                Trade {
                    time: "14:27:42".to_string(),
                    price: 192.52,
                    volume: 600,
                    direction: "买入".to_string(),
                },
                Trade {
                    time: "14:26:33".to_string(),
                    price: 192.48,
                    volume: 900,
                    direction: "卖出".to_string(),
                },
                Trade {
                    time: "14:25:10".to_string(),
                    price: 192.45,
                    volume: 1100,
                    direction: "买入".to_string(),
                },
            ],
            selected_stock: StockDetail {
                symbol: "NVDA".to_string(),
                price: 192.60,
                change: 0.03,
                change_percent: 0.02,
                open: 192.50,
                high: 193.80,
                low: 192.30,
                prev_close: 192.57,
                volume: 8542000,
                amount: 1645000000.0,
                bid_price: 192.58,
                ask_price: 192.62,
            },
        }
    }
}
