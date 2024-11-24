use std::error::Error;
use std::path::Path;
use std::fs::File;
use csv::ReaderBuilder;

pub mod types;
pub use types::MarketData;

pub fn load_csv_data_from_file<P: AsRef<Path>>(file_path: P) -> Result<Vec<MarketData>, Box<dyn Error>> {
    let file = File::open(file_path)?;

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',') // Delimitador é vírgula
        .quote(b'"') // Caracter de aspas é "
        .from_reader(file);

    let mut records = Vec::new();

    for result in rdr.deserialize() {
        let record: MarketData = result?;
        records.push(record);
    }

    // Opcional: inverter a ordem dos registros se necessário
    records.reverse();

    Ok(records)
}
