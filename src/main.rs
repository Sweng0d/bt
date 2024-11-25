mod data_loader;
mod strategy;
mod backtester;
mod report;

use data_loader::load_csv_data_from_file;
use strategy::{MovingAverageCrossover, BuyAndHold, Strategy};
use backtester::run_backtest;
use report::{calculate_metrics, plot_portfolio_value};
use rayon::prelude::*;
use std::cmp::Ordering;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Carregar os dados do mercado e envolver em Arc
    let file_path = "data/bitcoin_historical_data.csv";
    let market_data = Arc::new(load_csv_data_from_file(file_path)?);

    println!("Total de registros carregados: {}", market_data.len());

    // Estratégia de Buy and Hold (Exemplo Adicional)
    let buy_and_hold_strategy = BuyAndHold::new();
    let (portfolio_bh, _) = run_backtest(&buy_and_hold_strategy, &market_data[..]);
    let total_return_bh = (portfolio_bh.last().unwrap() - 10000.0) / 10000.0;
    println!(
        "\nEstratégia Buy and Hold:\nRetorno Total: {:.2}%",
        total_return_bh * 100.0
    );

    // Definir os intervalos das médias móveis
    let short_ma_range = 5..=200;
    let long_ma_range = 5..=200;

    // Paralelizar o processamento usando rayon
    let results: Vec<_> = short_ma_range
        .into_par_iter()
        .flat_map(|short_window| {
            let market_data = Arc::clone(&market_data);
            long_ma_range.clone().into_par_iter().filter_map(move |long_window| {
                if long_window > short_window {
                    let strategy = MovingAverageCrossover::new(short_window, long_window);
                    let (portfolio_values, _) = run_backtest(&strategy, &market_data[..]);
                    let total_return = (portfolio_values.last().unwrap() - 10000.0) / 10000.0;

                    Some((short_window, long_window, total_return))
                } else {
                    None
                }
            })
        })
        .collect();

    // Ordenar os resultados pelo retorno total
    let mut sorted_results = results.clone();
    sorted_results.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(Ordering::Equal));

    // Exibir as duas melhores combinações
    println!("\nAs duas melhores combinações de Moving Average Crossover são:");
    for (i, (short_ma, long_ma, total_return)) in sorted_results.iter().take(2).enumerate() {
        println!(
            "{}º: Short MA: {}, Long MA: {}, Retorno Total: {:.2}%",
            i + 1,
            short_ma,
            long_ma,
            total_return * 100.0
        );
    }

    // Gerar relatório para a melhor combinação
    let (best_short_ma, best_long_ma, _) = sorted_results[0];

    let strategy = MovingAverageCrossover::new(best_short_ma, best_long_ma);
    let (portfolio_values, signals) = run_backtest(&strategy, &market_data[..]);

    // Calcular métricas
    let metrics = calculate_metrics(&portfolio_values, &signals, &market_data[..], 0.02);

    // Gerar gráfico da curva de capital
    plot_portfolio_value(&portfolio_values, "portfolio_value.png")?;

    // Exibir métricas
    println!("\nMétricas de Desempenho:");
    println!("Retorno Total: {:.2}%", metrics.total_return * 100.0);
    println!("Retorno Anualizado: {:.2}%", metrics.annualized_return * 100.0);
    println!("Volatilidade: {:.2}%", metrics.volatility * 100.0);
    println!("Sharpe Ratio: {:.2}", metrics.sharpe_ratio);
    println!("Drawdown Máximo: {:.2}%", metrics.max_drawdown * 100.0);
    println!("Número de Trades: {}", metrics.num_trades);

    Ok(())
}
