mod data_loader;
mod strategy;
mod backtester;
mod report;

use data_loader::load_csv_data_from_file;
use strategy::{MovingAverageCrossover, BuyAndHold, process_strategy};
use rayon::prelude::*;
use std::cmp::Ordering;
use std::path::Path;
use std::sync::Arc;
use crate::backtester::run_backtest;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Carregar os dados do mercado e envolver em Arc
    let file_path = "data/bitcoin_historical_data.csv";
    let market_data = Arc::new(load_csv_data_from_file(file_path)?);

    println!("Total de registros carregados: {}", market_data.len());

    // Criar diretório de saída para os relatórios
    let output_dir = Path::new("reports");
    std::fs::create_dir_all(&output_dir)?;

    // Estratégia de Buy and Hold
    let buy_and_hold_strategy = BuyAndHold::new();
    process_strategy(
        &buy_and_hold_strategy,
        &market_data,
        "buy_and_hold",
        &output_dir,
        0.02,
    )?;

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
                    let (portfolio_values, _, _) = run_backtest(&strategy, &market_data[..]);
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

    // Processar a melhor combinação de Moving Average Crossover
    let (best_short_ma, best_long_ma, _) = sorted_results[0];
    let best_strategy = MovingAverageCrossover::new(best_short_ma, best_long_ma);
    process_strategy(
        &best_strategy,
        &market_data,
        "best_moving_average_crossover",
        &output_dir,
        0.02,
    )?;

    println!("\nRelatórios gerados em {:?}", output_dir);

    Ok(())
}
