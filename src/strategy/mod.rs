pub mod indicators;
pub mod process_strategy;
pub use process_strategy::process_strategy;


use crate::data_loader::MarketData;


/// Define o comportamento comum para todas as estratégias
pub trait Strategy {
    fn generate_signals(&self, market_data: &[MarketData]) -> Vec<Signal>;
}

/// Estratégia de Moving Average Crossover
pub struct MovingAverageCrossover {
    pub short_window: usize,
    pub long_window: usize,
}

impl MovingAverageCrossover {
    pub fn new(short_window: usize, long_window: usize) -> Self {
        Self {
            short_window,
            long_window,
        }
    }
}

/// Implementa o trait Strategy para Moving Average Crossover
impl Strategy for MovingAverageCrossover {
    fn generate_signals(&self, market_data: &[MarketData]) -> Vec<Signal> {
        let short_ma = indicators::moving_average(market_data, self.short_window);
        let long_ma = indicators::moving_average(market_data, self.long_window);

        let mut signals = Vec::new();

        for i in 1..market_data.len() {
            let prev_short_ma = short_ma[i - 1];
            let prev_long_ma = long_ma[i - 1];
            let current_short_ma = short_ma[i];
            let current_long_ma = long_ma[i];

            let signal = if prev_short_ma <= prev_long_ma && current_short_ma > current_long_ma {
                Signal::Buy
            } else if prev_short_ma >= prev_long_ma && current_short_ma < current_long_ma {
                Signal::Sell
            } else {
                Signal::Hold
            };

            signals.push(signal);
        }

        signals
    }
}

/// Estratégia de Buy and Hold
pub struct BuyAndHold;

impl BuyAndHold {
    pub fn new() -> Self {
        Self {}
    }
}

/// Implementa o trait Strategy para Buy and Hold
impl Strategy for BuyAndHold {
    fn generate_signals(&self, market_data: &[MarketData]) -> Vec<Signal> {
        // Apenas compra no início e mantém posição
        let mut signals = vec![Signal::Buy];
        signals.extend(vec![Signal::Hold; market_data.len() - 1]);
        signals
    }
}

/// Enum que representa os sinais de compra, venda ou manutenção
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
}
