use std::path::Path;
use crate::data_loader::MarketData;
use crate::strategy::Strategy;
use crate::backtester::run_backtest;
use crate::report::{calculate_metrics, plot_portfolio_value};

pub fn process_strategy<S: Strategy>(
    strategy: &S,
    market_data: &[MarketData],
    strategy_name: &str,
    output_dir: &Path,
    risk_free_rate: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    // Executar o backtest
    let (portfolio_values, signals, trades) = run_backtest(strategy, market_data);

    // Calcular métricas de desempenho
    let metrics = calculate_metrics(&portfolio_values, &signals, market_data, risk_free_rate);

    // Exibir métricas no console
    println!("\nRelatório da Estratégia: {}", strategy_name);
    println!("Retorno Total: {:.2}%", metrics.total_return * 100.0);
    println!("Retorno Anualizado: {:.2}%", metrics.annualized_return * 100.0);
    println!("Volatilidade: {:.2}%", metrics.volatility * 100.0);
    println!("Sharpe Ratio: {:.2}", metrics.sharpe_ratio);
    println!("Drawdown Máximo: {:.2}%", metrics.max_drawdown * 100.0);
    println!("Número de Trades: {}", metrics.num_trades);

    // Gerar gráfico do portfólio
    let file_path = output_dir.join(format!("{}_portfolio_value.png", strategy_name));
    plot_portfolio_value(&portfolio_values, file_path.to_str().unwrap())?;

    Ok(())
}
