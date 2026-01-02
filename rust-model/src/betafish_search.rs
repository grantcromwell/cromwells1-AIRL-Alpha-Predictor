use crate::types::{EquityData, AlphaResult};
use std::collections::HashMap;

pub struct BetafishSearch {
    window_size: usize,
    top_n: usize,
}

impl BetafishSearch {
    pub fn new(window_size: usize, top_n: usize) -> Self {
        BetafishSearch {
            window_size,
            top_n,
        }
    }
    
    pub fn find_strongest_movers(&self, data: &HashMap<String, Vec<EquityData>>) -> Vec<AlphaResult> {
        let mut alpha_results: Vec<AlphaResult> = Vec::new();
        
        for (symbol, equity_data) in data {
            if equity_data.len() < self.window_size {
                continue;
            }
            
            let window_data = &equity_data[equity_data.len() - self.window_size..];
            
            let alpha = self.calculate_alpha(window_data);
            let volume = self.calculate_average_volume(window_data);
            let change = self.calculate_price_change(window_data);
            let probability = self.calculate_probability(alpha, volume);
            
            alpha_results.push(AlphaResult {
                symbol: symbol.clone(),
                alpha,
                probability,
                volume,
                change,
            });
        }
        
        alpha_results.sort_by(|a, b| {
            b.alpha.partial_cmp(&a.alpha).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        alpha_results.into_iter()
            .take(self.top_n)
            .collect()
    }
    
    fn calculate_alpha(&self, data: &[EquityData]) -> f64 {
        if data.len() < 2 {
            return 0.0;
        }
        
        let mut returns: Vec<f64> = data.windows(2)
            .map(|w| (w[1].close - w[0].close) / w[0].close)
            .collect();
        
        let mean_return = returns.iter().sum::<f64>() / returns.len() as f64;
        let std_return = (returns.iter()
            .map(|&r| (r - mean_return).powi(2))
            .sum::<f64>() / returns.len() as f64)
            .sqrt();
        
        if std_return == 0.0 {
            return 0.0;
        }
        
        let sharpe = mean_return / std_return;
        let alpha = sharpe * (returns.len() as f64).sqrt();
        
        alpha
    }
    
    fn calculate_average_volume(&self, data: &[EquityData]) -> f64 {
        data.iter().map(|d| d.volume).sum::<f64>() / data.len() as f64
    }
    
    fn calculate_price_change(&self, data: &[EquityData]) -> f64 {
        if data.len() < 2 {
            return 0.0;
        }
        
        (data.last().unwrap().close - data.first().unwrap().close) / data.first().unwrap().close * 100.0
    }
    
    fn calculate_probability(&self, alpha: f64, volume: f64) -> f64 {
        let alpha_score = alpha.abs().min(10.0) / 10.0;
        let volume_score = (volume / 10000000.0).min(1.0).max(0.0);
        let combined = 0.7 * alpha_score + 0.3 * volume_score;
        
        combined.max(0.0).min(1.0)
    }
    
    pub fn find_highest_volume(&self, data: &HashMap<String, Vec<EquityData>>) -> Vec<AlphaResult> {
        let mut volume_results: Vec<AlphaResult> = Vec::new();
        
        for (symbol, equity_data) in data {
            if equity_data.is_empty() {
                continue;
            }
            
            let recent_data = &equity_data[equity_data.len().saturating_sub(self.window_size)..];
            
            let alpha = self.calculate_alpha(recent_data);
            let volume = self.calculate_average_volume(recent_data);
            let change = self.calculate_price_change(recent_data);
            let probability = self.calculate_probability(alpha, volume);
            
            volume_results.push(AlphaResult {
                symbol: symbol.clone(),
                alpha,
                probability,
                volume,
                change,
            });
        }
        
        volume_results.sort_by(|a, b| {
            b.volume.partial_cmp(&a.volume).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        volume_results.into_iter()
            .take(self.top_n)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_betafish_search() {
        let search = BetafishSearch::new(30, 5);
        assert_eq!(search.window_size, 30);
    }
}
