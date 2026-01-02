use rust_model::{ModelPipeline, EquityData};
use std::collections::HashMap;

fn main() {
    println!("Financial Forecasting Model Pipeline");
    println!("=====================================\n");
    
    let mut pipeline = ModelPipeline::new();
    
    let data_by_symbol = generate_sample_data();
    println!("Generated data for {} symbols\n", data_by_symbol.len());
    
    println!("Training models...");
    pipeline.train(&data_by_symbol);
    println!("Training complete!\n");
    
    println!("=== Strongest Movers (Alpha Search) ===");
    let strongest_movers = pipeline.get_strongest_movers(&data_by_symbol);
    for result in &strongest_movers {
        let prob_pct = result.probability * 100.0;
        let change_pct = result.change;
        println!("Symbol: {:<8} | Alpha: {:>8.4} | Probability: {:>6.2}% | Change: {:>7.2}%", 
                 result.symbol, result.alpha, prob_pct, change_pct);
    }
    
    println!("\n=== Highest Volume ===");
    let highest_volume = pipeline.get_highest_volume(&data_by_symbol);
    for result in &highest_volume {
        println!("Symbol: {:<8} | Volume: {:>12.0} | Alpha: {:>8.4}", 
                 result.symbol, result.volume, result.alpha);
    }
    
    println!("\n=== Highest Probable Alpha (Gaussian Copula) ===");
    let probable_alpha = pipeline.get_highest_probable_alpha(&data_by_symbol);
    for result in &probable_alpha {
        let prob_pct = result.probability * 100.0;
        let change_pct = result.change;
        println!("Symbol: {:<8} | Alpha: {:>8.4} | Probability: {:>6.2}% | Change: {:>7.2}%", 
                 result.symbol, result.alpha, prob_pct, change_pct);
    }
    
    let corr_analysis = pipeline.get_correlation_analysis();
    println!("\n=== Correlation Analysis ===");
    println!("Matrix Size: {}", corr_analysis.matrix_size);
    println!("Highest Correlation: {:.4}", corr_analysis.highest_correlation);
    println!("Lowest Correlation: {:.4}", corr_analysis.lowest_correlation);
    
    println!("\nPipeline execution complete!");
}

fn generate_sample_data() -> HashMap<String, Vec<EquityData>> {
    let symbols = vec![
        "NQ", "NVDA", "AMD", "WDC", "SLV", 
        "GS", "NET", "EWJ", "EURUSD", "INRJPY", "BRLGBP"
    ];
    
    let mut data_by_symbol: HashMap<String, Vec<EquityData>> = HashMap::new();
    
    for symbol in symbols {
        let mut data = Vec::new();
        let base_price = get_base_price(symbol);
        let current_time = chrono::Utc::now().timestamp_millis();
        let one_day_ms = 24 * 60 * 60 * 1000;
        
        for i in 0..100 {
            let timestamp = current_time - ((99 - i) * one_day_ms as i64);
            let price_change = (fastrand::f64() - 0.5) * base_price * 0.05;
            let open = base_price + price_change;
            let close = open + (fastrand::f64() - 0.5) * base_price * 0.02;
            let high = open.max(close) + fastrand::f64() * base_price * 0.01;
            let low = open.min(close) - fastrand::f64() * base_price * 0.01;
            let volume = 1_000_000.0 + fastrand::f64() * 5_000_000.0;
            
            let equity = EquityData {
                symbol: symbol.to_string(),
                timestamp,
                open,
                high,
                low,
                close,
                volume,
                adjusted_close: close,
                moving_averages: None,
                rsi: None,
                macd: None,
            };
            
            data.push(equity);
        }
        
        data_by_symbol.insert(symbol.to_string(), data);
    }
    
    data_by_symbol
}

fn get_base_price(symbol: &str) -> f64 {
    match symbol {
        "NQ" => 16000.0,
        "NVDA" => 500.0,
        "AMD" => 150.0,
        "WDC" => 60.0,
        "SLV" => 25.0,
        "GS" => 350.0,
        "NET" => 80.0,
        "EWJ" => 65.0,
        "EURUSD" => 1.08,
        "INRJPY" => 1.80,
        "BRLGBP" => 0.252,  // BRL to GBP exchange rate
        _ => 100.0,
    }
}
