use crate::types::{EquityData, TrainingSample};
use rand::prelude::IteratorRandom;
use rand::thread_rng;
use std::collections::HashMap;

pub struct DecisionTree {
    root: Option<Node>,
    max_depth: usize,
    min_samples_split: usize,
    n_features: usize,
}

#[derive(Debug, Clone)]
struct Node {
    feature_index: Option<usize>,
    threshold: Option<f64>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    value: Option<f64>,
    is_leaf: bool,
}

impl DecisionTree {
    pub fn new(max_depth: usize, min_samples_split: usize, n_features: usize) -> Self {
        DecisionTree {
            root: None,
            max_depth,
            min_samples_split,
            n_features,
        }
    }
    
    pub fn fit(&mut self, samples: &[TrainingSample]) {
        let n_samples = samples.len();
        let n_features = samples[0].features.len();
        
        if n_samples == 0 {
            return;
        }
        
        let mut X: Vec<Vec<f64>> = samples.iter()
            .map(|s| s.features.clone())
            .collect();
        
        let y: Vec<f64> = samples.iter()
            .map(|s| s.target)
            .collect();
        
        let n_select_features = self.n_features.min(n_features);
        let feature_indices: Vec<usize> = (0..n_features)
            .choose_multiple(&mut thread_rng(), n_select_features);
        
        self.root = Some(self.build_tree(&X, &y, 0, &feature_indices));
    }
    
    fn build_tree(&self, X: &[Vec<f64>], y: &[f64], depth: usize, 
                  feature_indices: &[usize]) -> Node {
        let n_samples = X.len();
        let n_labels = y.len();
        
        if depth >= self.max_depth || n_samples < self.min_samples_split || n_labels == 0 {
            let value = if y.is_empty() { 0.0 } else { y.iter().sum::<f64>() / y.len() as f64 };
            return Node {
                feature_index: None,
                threshold: None,
                left: None,
                right: None,
                value: Some(value),
                is_leaf: true,
            };
        }
        
        let best_split = self.find_best_split(X, y, feature_indices);
        
        if best_split.feature_index.is_none() {
            let value = if y.is_empty() { 0.0 } else { y.iter().sum::<f64>() / y.len() as f64 };
            return Node {
                feature_index: None,
                threshold: None,
                left: None,
                right: None,
                value: Some(value),
                is_leaf: true,
            };
        }
        
        let left_indices = &best_split.left_indices;
        let right_indices = &best_split.right_indices;
        
        let left_X: Vec<Vec<f64>> = left_indices.iter()
            .map(|&i| X[i].clone())
            .collect();
        let left_y: Vec<f64> = left_indices.iter()
            .map(|&i| y[i])
            .collect();
        let right_X: Vec<Vec<f64>> = right_indices.iter()
            .map(|&i| X[i].clone())
            .collect();
        let right_y: Vec<f64> = right_indices.iter()
            .map(|&i| y[i])
            .collect();
        
        let left_subtree = self.build_tree(&left_X, &left_y, depth + 1, feature_indices);
        let right_subtree = self.build_tree(&right_X, &right_y, depth + 1, feature_indices);
        
        Node {
            feature_index: best_split.feature_index,
            threshold: best_split.threshold,
            left: Some(Box::new(left_subtree)),
            right: Some(Box::new(right_subtree)),
            value: None,
            is_leaf: false,
        }
    }
    
    fn find_best_split(&self, X: &[Vec<f64>], y: &[f64], 
                      feature_indices: &[usize]) -> SplitResult {
        let mut best_gain = f64::NEG_INFINITY;
        let mut best_feature = None;
        let mut best_threshold = None;
        let mut best_left_indices = Vec::new();
        let mut best_right_indices = Vec::new();
        
        let parent_variance = self.variance(y);
        
        for &feature_idx in feature_indices {
            let feature_values: Vec<f64> = X.iter()
                .map(|row| row[feature_idx])
                .collect();
            let thresholds = self.get_unique_thresholds(&feature_values);
            
            for &threshold in &thresholds {
                let mut left_indices = Vec::new();
                let mut right_indices = Vec::new();
                
                for (i, &val) in feature_values.iter().enumerate() {
                    if val <= threshold {
                        left_indices.push(i);
                    } else {
                        right_indices.push(i);
                    }
                }
                
                if left_indices.is_empty() || right_indices.is_empty() {
                    continue;
                }
                
                let left_y: Vec<f64> = left_indices.iter().map(|&i| y[i]).collect();
                let right_y: Vec<f64> = right_indices.iter().map(|&i| y[i]).collect();
                
                let n = y.len() as f64;
                let n_left = left_y.len() as f64;
                let n_right = right_y.len() as f64;
                
                let gain = parent_variance 
                    - (n_left / n) * self.variance(&left_y)
                    - (n_right / n) * self.variance(&right_y);
                
                if gain > best_gain {
                    best_gain = gain;
                    best_feature = Some(feature_idx);
                    best_threshold = Some(threshold);
                    best_left_indices = left_indices;
                    best_right_indices = right_indices;
                }
            }
        }
        
        SplitResult {
            feature_index: best_feature,
            threshold: best_threshold,
            left_indices: best_left_indices,
            right_indices: best_right_indices,
        }
    }
    
    fn get_unique_thresholds(&self, values: &[f64]) -> Vec<f64> {
        let mut sorted = values.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        if sorted.len() <= 1 {
            return sorted;
        }
        
        let mut thresholds = Vec::new();
        for i in 0..sorted.len() - 1 {
            if (sorted[i] - sorted[i + 1]).abs() >= 1e-10 {
                thresholds.push((sorted[i] + sorted[i + 1]) / 2.0);
            }
        }
        
        thresholds
    }
    
    fn variance(&self, values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        values.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / values.len() as f64
    }
    
    pub fn predict(&self, features: &[f64]) -> f64 {
        match &self.root {
            Some(node) => self.predict_node(node, features),
            None => 0.0,
        }
    }
    
    fn predict_node(&self, node: &Node, features: &[f64]) -> f64 {
        if node.is_leaf {
            return node.value.unwrap_or(0.0);
        }
        
        let feature_idx = node.feature_index.unwrap();
        let feature_val = features[feature_idx];
        let threshold = node.threshold.unwrap();
        
        if feature_val <= threshold {
            match &node.left {
                Some(left) => self.predict_node(left, features),
                None => node.value.unwrap_or(0.0),
            }
        } else {
            match &node.right {
                Some(right) => self.predict_node(right, features),
                None => node.value.unwrap_or(0.0),
            }
        }
    }
}

#[derive(Debug, Clone)]
struct SplitResult {
    feature_index: Option<usize>,
    threshold: Option<f64>,
    left_indices: Vec<usize>,
    right_indices: Vec<usize>,
}
