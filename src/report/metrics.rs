//use crate::backtester::BacktestResult;
use crate::data_loader::MarketData;
use crate::strategy::Signal;

pub struct PerformanceMetrics {
    pub total_return: f64,
    pub annualized_return: f64,
    pub volatility: f64,
    pub sharpe_ratio: f64,
    pub max_drawdown: f64,
    pub num_trades: usize,
    // Adicione outras métricas conforme necessário
}

pub fn calculate_metrics(
    portfolio_values: &[f64],
    signals: &[Signal],
    market_data: &[MarketData],
    risk_free_rate: f64,
) -> PerformanceMetrics {
    // Implementar cálculos das métricas aqui
    // Por exemplo, calcular retornos diários, volatilidade, Sharpe Ratio, etc.

    // Exemplo simplificado:
    let total_return = (portfolio_values.last().unwrap() - portfolio_values[0]) / portfolio_values[0];
    let annualized_return = total_return / (market_data.len() as f64 / 252.0);

    // Calcular retornos diários
    let daily_returns: Vec<f64> = portfolio_values
        .windows(2)
        .map(|w| (w[1] - w[0]) / w[0])
        .collect();

    // Calcular volatilidade anualizada
    let volatility = standard_deviation(&daily_returns) * (252.0f64).sqrt();

    // Calcular Sharpe Ratio
    let sharpe_ratio = if volatility != 0.0 {
        (annualized_return - risk_free_rate) / volatility
    } else {
        0.0
    };

    // Calcular Drawdown Máximo
    let max_drawdown = calculate_max_drawdown(portfolio_values);

    PerformanceMetrics {
        total_return,
        annualized_return,
        volatility,
        sharpe_ratio,
        max_drawdown,
        num_trades: signals.iter().filter(|&&s| s != Signal::Hold).count(),
        // Preencher outras métricas
    }
}

fn standard_deviation(data: &[f64]) -> f64 {
    let mean = data.iter().copied().sum::<f64>() / data.len() as f64;
    let var = data.iter().map(|value| {
        let diff = mean - *value;
        diff * diff
    }).sum::<f64>() / data.len() as f64;
    var.sqrt()
}

fn calculate_max_drawdown(portfolio_values: &[f64]) -> f64 {
    let mut peak = portfolio_values[0];
    let mut max_drawdown = 0.0;

    for &value in portfolio_values {
        if value > peak {
            peak = value;
        }
        let drawdown = (peak - value) / peak;
        if drawdown > max_drawdown {
            max_drawdown = drawdown;
        }
    }

    max_drawdown
}
