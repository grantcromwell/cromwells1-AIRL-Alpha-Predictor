use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquityData {
    pub symbol: String,
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub adjusted_close: f64,
    pub moving_averages: Option<HashMap<String, f64>>,
    pub rsi: Option<f64>,
    pub macd: Option<HashMap<String, f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingSample {
    pub features: Vec<f64>,
    pub target: f64,
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prediction {
    pub symbol: String,
    pub timestamp: i64,
    pub predicted_return: f64,
    pub confidence: f64,
    pub features: Vec<f64>,
}

impl EquityData {
    pub fn to_training_sample(&self, future_return: f64) -> TrainingSample {
        let mut features = Vec::new();
        
        features.push(self.close);
        features.push(self.volume);
        features.push((self.high - self.low) / self.close);
        features.push(self.close - self.open);
        
        if let Some(ma) = &self.moving_averages {
            features.push(*ma.get("ma_5").unwrap_or(&0.0));
            features.push(*ma.get("ma_10").unwrap_or(&0.0));
            features.push(*ma.get("ma_20").unwrap_or(&0.0));
            features.push(*ma.get("ma_50").unwrap_or(&0.0));
            
            if let (Some(&ma5), Some(&close)) = (ma.get("ma_5"), Some(&self.close)) {
                features.push((close - ma5) / ma5);
            } else {
                features.push(0.0);
            }
        } else {
            features.extend(&[0.0, 0.0, 0.0, 0.0, 0.0]);
        }
        
        if let Some(rsi) = self.rsi {
            features.push(rsi);
        } else {
            features.push(50.0);
        }
        
        if let Some(macd) = &self.macd {
            features.push(*macd.get("macd").unwrap_or(&0.0));
            features.push(*macd.get("signal").unwrap_or(&0.0));
            features.push(*macd.get("histogram").unwrap_or(&0.0));
        } else {
            features.extend(&[0.0, 0.0, 0.0]);
        }
        
        let vol_20 = self.calculate_volatility(20);
        features.push(vol_20);
        
        features.push(self.high / self.low - 1.0);
        
        TrainingSample {
            features,
            target: future_return,
            symbol: self.symbol.clone(),
        }
    }
    
    pub fn calculate_volatility(&self, period: usize) -> f64 {
        0.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlphaResult {
    pub symbol: String,
    pub alpha: f64,
    pub probability: f64,
    pub volume: f64,
    pub change: f64,
}
