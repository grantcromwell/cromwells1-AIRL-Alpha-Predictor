pub mod types;
pub mod decision_tree;
pub mod random_forest;
pub mod gaussian_copula;
pub mod betafish_search;
pub mod exa_search;
pub mod lib_tests;

pub use types::{EquityData, TrainingSample, Prediction, AlphaResult};
pub use random_forest::{RandomForest, ModelMetrics};
pub use gaussian_copula::GaussianCopula;
pub use betafish_search::BetafishSearch;
pub use exa_search::{ExaSearch, CorrelationAnalysis};

use std::collections::HashMap;

pub struct ModelPipeline {
    random_forest: RandomForest,
    gaussian_copula: GaussianCopula,
    betafish_search: BetafishSearch,
    exa_search: ExaSearch,
}

impl ModelPipeline {
    pub fn new() -> Self {
        ModelPipeline {
            random_forest: RandomForest::new(100, 20, 5, 10),
            gaussian_copula: GaussianCopula::new(),
            betafish_search: BetafishSearch::new(30, 5),
            exa_search: ExaSearch::new(0.6, 5),
        }
    }
    
    pub fn train(&mut self, data_by_symbol: &HashMap<String, Vec<EquityData>>) {
        let mut all_samples = Vec::new();
        
        for (symbol, equity_data) in data_by_symbol {
            if equity_data.len() < 60 {
                continue;
            }
            
            for i in 10..equity_data.len() - 1 {
                let future_return = (equity_data[i + 1].close - equity_data[i].close) 
                    / equity_data[i].close * 100.0;
                
                let sample = equity_data[i].to_training_sample(future_return);
                all_samples.push(sample);
            }
        }
        
        println!("Training Random Forest with {} samples...", all_samples.len());
        self.random_forest.fit(&all_samples);
        
        println!("Training Gaussian Copula...");
        self.gaussian_copula.fit(data_by_symbol);
        
        println!("Training Exa Search...");
        self.exa_search.fit(data_by_symbol);
    }
    
    pub fn predict(&self, symbol: &str, features: &[f64]) -> Option<Prediction> {
        let (predicted_return, confidence) = self.random_forest.predict_with_variance(features);
        
        Some(Prediction {
            symbol: symbol.to_string(),
            timestamp: chrono::Utc::now().timestamp_millis(),
            predicted_return,
            confidence: confidence.sqrt(),
            features: features.to_vec(),
        })
    }
    
    pub fn get_strongest_movers(&self, data: &HashMap<String, Vec<EquityData>>) -> Vec<AlphaResult> {
        self.betafish_search.find_strongest_movers(data)
    }
    
    pub fn get_highest_volume(&self, data: &HashMap<String, Vec<EquityData>>) -> Vec<AlphaResult> {
        self.betafish_search.find_highest_volume(data)
    }
    
    pub fn get_highest_probable_alpha(&self, data: &HashMap<String, Vec<EquityData>>) -> Vec<AlphaResult> {
        self.exa_search.find_highest_probable_alpha(data)
    }
    
    pub fn get_model_metrics(&self) -> ModelMetrics {
        ModelMetrics {
            mse: 0.0,
            rmse: 0.0,
            mae: 0.0,
            r2: 0.0,
        }
    }
    
    pub fn get_correlation_analysis(&self) -> CorrelationAnalysis {
        self.exa_search.get_correlation_analysis()
    }
}

impl Default for ModelPipeline {
    fn default() -> Self {
        Self::new()
    }
}
