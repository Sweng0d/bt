pub mod indicators;

use crate::data_loader::MarketData;

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

    pub fn generate_signals(&self, market_data: &[MarketData]) -> Vec<Signal> {
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
}
