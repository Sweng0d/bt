use crate::strategy::{Strategy, Signal};
use crate::data_loader::MarketData;

#[derive(Debug)]
pub struct Trade {
    pub date: String,
    pub signal: Signal,
    pub price: f64,
    pub quantity: f64,
}

pub fn run_backtest<S: Strategy>(
    strategy: &S,
    market_data: &[MarketData],
) -> (Vec<f64>, Vec<Signal>, Vec<Trade>) {
    let signals = strategy.generate_signals(market_data);

    let mut position = 0.0; // Quantidade de ativos
    let mut cash = 10000.0; // Capital inicial
    let mut portfolio_values = Vec::new();
    let mut trades = Vec::new(); // Registro de trades

    for (i, signal) in signals.iter().enumerate() {
        let price = market_data[i].price.unwrap_or(0.0);

        match signal {
            Signal::Buy => {
                if position == 0.0 {
                    let quantity = cash / price;
                    trades.push(Trade {
                        date: market_data[i].date.to_string(),
                        signal: Signal::Buy,
                        price,
                        quantity,
                    });
                    position = quantity;
                    cash = 0.0;
                }
            }
            Signal::Sell => {
                if position > 0.0 {
                    trades.push(Trade {
                        date: market_data[i].date.to_string(),
                        signal: Signal::Sell,
                        price,
                        quantity: position,
                    });
                    cash = position * price;
                    position = 0.0;
                }
            }
            Signal::Hold => {}
        }

        let total_value = cash + position * price;
        portfolio_values.push(total_value);
    }

    (portfolio_values, signals, trades)
}
