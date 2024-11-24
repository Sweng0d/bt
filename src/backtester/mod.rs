use crate::strategy::{MovingAverageCrossover, Signal};
use crate::data_loader::MarketData;

pub fn run_backtest(
    strategy: &MovingAverageCrossover,
    market_data: &[MarketData],
) -> (Vec<f64>, Vec<Signal>) {
    let signals = strategy.generate_signals(market_data);

    let mut position = 0.0; // Quantidade de ativos
    let mut cash = 10000.0; // Capital inicial
    let mut portfolio_values = Vec::new();

    for (i, signal) in signals.iter().enumerate() {
        let price = market_data[i + 1].price.unwrap_or(0.0);

        match signal {
            Signal::Buy => {
                if position == 0.0 {
                    position = cash / price;
                    cash = 0.0;
                }
            }
            Signal::Sell => {
                if position > 0.0 {
                    cash = position * price;
                    position = 0.0;
                }
            }
            Signal::Hold => {}
        }

        let total_value = cash + position * price;
        portfolio_values.push(total_value);
    }

    // Retornar os valores da carteira e os sinais
    (portfolio_values, signals)
}
