use crate::gaussian_copula::GaussianCopula;
use crate::types::{EquityData, AlphaResult};
use std::collections::HashMap;

pub struct ExaSearch {
    gaussian_copula: GaussianCopula,
    confidence_threshold: f64,
    top_n: usize,
}

impl ExaSearch {
    pub fn new(confidence_threshold: f64, top_n: usize) -> Self {
        ExaSearch {
            gaussian_copula: GaussianCopula::new(),
            confidence_threshold,
            top_n,
        }
    }
    
    pub fn fit(&mut self, data: &HashMap<String, Vec<EquityData>>) {
        self.gaussian_copula.fit(data);
    }
    
    pub fn find_highest_probable_alpha(&self, data: &HashMap<String, Vec<EquityData>>) -> Vec<AlphaResult> {
        let mut alpha_results: Vec<AlphaResult> = Vec::new();
        
        for (symbol, equity_data) in data {
            if equity_data.is_empty() {
                continue;
            }
            
            let distribution = self.gaussian_copula.get_probability_distribution(symbol, 100);
            
            let (expected_alpha, probability) = self.calculate_expected_alpha(&distribution);
            
            let latest = equity_data.last().unwrap();
            let volume = latest.volume;
            let change = self.calculate_price_change(equity_data);
            
            if probability >= self.confidence_threshold {
                alpha_results.push(AlphaResult {
                    symbol: symbol.clone(),
                    alpha: expected_alpha,
                    probability,
                    volume,
                    change,
                });
            }
        }
        
        alpha_results.sort_by(|a, b| {
            let score_a = a.alpha * a.probability;
            let score_b = b.alpha * b.probability;
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        alpha_results.into_iter()
            .take(self.top_n)
            .collect()
    }
    
    fn calculate_expected_alpha(&self, distribution: &[f64]) -> (f64, f64) {
        let n = distribution.len();
        
        let mut expected_alpha = 0.0f64;
        let mut max_prob = 0.0f64;
        
        for (i, &prob) in distribution.iter().enumerate() {
            let alpha_value = (i as f64 / n as f64) * 2.0 - 1.0;
            expected_alpha += alpha_value * prob;
            if prob > max_prob {
                max_prob = prob;
            }
        }
        
        (expected_alpha, max_prob)
    }
    
    fn calculate_price_change(&self, data: &[EquityData]) -> f64 {
        if data.len() < 2 {
            return 0.0;
        }
        
        (data.last().unwrap().close - data.first().unwrap().close) / data.first().unwrap().close * 100.0
    }
    
    pub fn get_correlation_analysis(&self) -> CorrelationAnalysis {
        let correlation_matrix = self.gaussian_copula.get_correlation_matrix();
        
        let mut highest_correlation = 0.0f64;
        let mut lowest_correlation = 1.0f64;
        
        for i in 0..correlation_matrix.shape()[0] {
            for j in 0..correlation_matrix.shape()[1] {
                if i != j {
                    let val = correlation_matrix[[i, j]];
                    if val > highest_correlation {
                        highest_correlation = val;
                    }
                    if val < lowest_correlation {
                        lowest_correlation = val;
                    }
                }
            }
        }
        
        CorrelationAnalysis {
            highest_correlation,
            lowest_correlation,
            matrix_size: correlation_matrix.shape()[0],
        }
    }
}

#[derive(Debug, Clone)]
pub struct CorrelationAnalysis {
    pub highest_correlation: f64,
    pub lowest_correlation: f64,
    pub matrix_size: usize,
}
