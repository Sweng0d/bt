use crate::data_loader::MarketData;

pub fn moving_average(market_data: &[MarketData], window_size: usize) -> Vec<f64> {
    let mut ma = Vec::with_capacity(market_data.len());
    let mut sum = 0.0;

    for i in 0..market_data.len() {
        let close_price = market_data[i].price.unwrap_or(0.0);

        sum += close_price;

        if i >= window_size {
            let old_price = market_data[i - window_size].price.unwrap_or(0.0);
            sum -= old_price;
            ma.push(sum / window_size as f64);
        } else {
            ma.push(sum / (i + 1) as f64);
        }
    }

    ma
}
