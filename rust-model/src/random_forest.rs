use crate::decision_tree::DecisionTree;
use crate::types::TrainingSample;
use rand::Rng;
use std::collections::HashMap;

pub struct RandomForest {
    trees: Vec<DecisionTree>,
    n_trees: usize,
    max_depth: usize,
    min_samples_split: usize,
    n_features: usize,
    feature_importance: HashMap<usize, f64>,
}

impl RandomForest {
    pub fn new(n_trees: usize, max_depth: usize, min_samples_split: usize, n_features: usize) -> Self {
        RandomForest {
            trees: Vec::with_capacity(n_trees),
            n_trees,
            max_depth,
            min_samples_split,
            n_features,
            feature_importance: HashMap::new(),
        }
    }
    
    pub fn fit(&mut self, samples: &[TrainingSample]) {
        self.trees.clear();
        
        for _ in 0..self.n_trees {
            let bootstrap_samples = self.bootstrap_sample(samples);
            
            let mut tree = DecisionTree::new(
                self.max_depth,
                self.min_samples_split,
                self.n_features,
            );
            
            tree.fit(&bootstrap_samples);
            self.trees.push(tree);
        }
        
        self.calculate_feature_importance(samples);
    }
    
    fn bootstrap_sample(&self, samples: &[TrainingSample]) -> Vec<TrainingSample> {
        let mut rng = rand::thread_rng();
        let mut bootstrap = Vec::with_capacity(samples.len());
        
        for _ in 0..samples.len() {
            let idx = rng.gen_range(0..samples.len());
            bootstrap.push(samples[idx].clone());
        }
        
        bootstrap
    }
    
    pub fn predict(&self, features: &[f64]) -> f64 {
        if self.trees.is_empty() {
            return 0.0;
        }
        
        let predictions: Vec<f64> = self.trees.iter()
            .map(|tree| tree.predict(features))
            .collect();
        
        predictions.iter().sum::<f64>() / predictions.len() as f64
    }
    
    pub fn predict_with_variance(&self, features: &[f64]) -> (f64, f64) {
        if self.trees.is_empty() {
            return (0.0, 0.0);
        }
        
        let predictions: Vec<f64> = self.trees.iter()
            .map(|tree| tree.predict(features))
            .collect();
        
        let mean = predictions.iter().sum::<f64>() / predictions.len() as f64;
        let variance = predictions.iter()
            .map(|&p| (p - mean).powi(2))
            .sum::<f64>() / predictions.len() as f64;
        
        (mean, variance)
    }
    
    pub fn predict_batch(&self, sample_list: &[Vec<f64>]) -> Vec<f64> {
        sample_list.iter()
            .map(|features| self.predict(features))
            .collect()
    }
    
    fn calculate_feature_importance(&mut self, _samples: &[TrainingSample]) {
        for i in 0..15 {
            self.feature_importance.insert(i, 1.0 / 15.0);
        }
    }
    
    pub fn get_feature_importance(&self) -> &HashMap<usize, f64> {
        &self.feature_importance
    }
    
    pub fn score(&self, samples: &[TrainingSample]) -> ModelMetrics {
        let mut predictions = Vec::with_capacity(samples.len());
        let mut actuals = Vec::with_capacity(samples.len());
        
        for sample in samples {
            let pred = self.predict(&sample.features);
            predictions.push(pred);
            actuals.push(sample.target);
        }
        
        let mse = self.calculate_mse(&predictions, &actuals);
        let rmse = mse.sqrt();
        let mae = self.calculate_mae(&predictions, &actuals);
        let r2 = self.calculate_r2(&predictions, &actuals);
        
        ModelMetrics {
            mse,
            rmse,
            mae,
            r2,
        }
    }
    
    fn calculate_mse(&self, predictions: &[f64], actuals: &[f64]) -> f64 {
        predictions.iter()
            .zip(actuals.iter())
            .map(|(&pred, &actual)| (pred - actual).powi(2))
            .sum::<f64>() / predictions.len() as f64
    }
    
    fn calculate_mae(&self, predictions: &[f64], actuals: &[f64]) -> f64 {
        predictions.iter()
            .zip(actuals.iter())
            .map(|(&pred, &actual)| (pred - actual).abs())
            .sum::<f64>() / predictions.len() as f64
    }
    
    fn calculate_r2(&self, predictions: &[f64], actuals: &[f64]) -> f64 {
        let mean_actual = actuals.iter().sum::<f64>() / actuals.len() as f64;
        
        let ss_res = predictions.iter()
            .zip(actuals.iter())
            .map(|(&pred, &actual)| (actual - pred).powi(2))
            .sum::<f64>();
        
        let ss_tot = actuals.iter()
            .map(|&actual| (actual - mean_actual).powi(2))
            .sum::<f64>();
        
        if ss_tot == 0.0 {
            1.0
        } else {
            1.0 - (ss_res / ss_tot)
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModelMetrics {
    pub mse: f64,
    pub rmse: f64,
    pub mae: f64,
    pub r2: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_random_forest_creation() {
        let rf = RandomForest::new(10, 10, 2, 5);
        assert_eq!(rf.n_trees, 10);
    }
    
    #[test]
    fn test_prediction() {
        let samples = create_test_samples();
        let mut rf = RandomForest::new(5, 5, 2, 3);
        rf.fit(&samples);
        
        let features = vec![100.0, 1000000.0, 0.02, 0.01, 100.0, 100.0, 100.0, 100.0, 0.0, 50.0, 0.0, 0.0, 0.0, 0.01, 0.02];
        let pred = rf.predict(&features);
        
        assert!(!pred.is_nan());
    }
    
    fn create_test_samples() -> Vec<TrainingSample> {
        let mut samples = Vec::new();
        for i in 0..100 {
            let features = vec![
                100.0 + i as f64,
                1000000.0,
                0.02,
                (i as f64 - 50.0) / 100.0,
                100.0, 100.0, 100.0, 100.0, 0.0,
                50.0, 0.0, 0.0, 0.0, 0.01, 0.02,
            ];
            let target = (i as f64 - 50.0) / 100.0;
            samples.push(TrainingSample {
                features,
                target,
                symbol: "TEST".to_string(),
            });
        }
        samples
    }
}
