use crate::types::EquityData;
use ndarray::{Array2, Array1};
use rand_distr::{Normal, Distribution};
use std::collections::HashMap;

pub struct GaussianCopula {
    correlation_matrix: Array2<f64>,
    symbols: Vec<String>,
    marginal_distributions: HashMap<String, MarginalDist>,
}

#[derive(Debug, Clone)]
pub struct MarginalDist {
    mean: f64,
    std: f64,
    dist_type: DistType,
}

#[derive(Debug, Clone)]
enum DistType {
    Normal,
    StudentT(f64),
    Empirical(Vec<f64>),
}

impl GaussianCopula {
    pub fn new() -> Self {
        GaussianCopula {
            correlation_matrix: Array2::eye(2),
            symbols: Vec::new(),
            marginal_distributions: HashMap::new(),
        }
    }
    
    pub fn fit(&mut self, data_by_symbol: &HashMap<String, Vec<EquityData>>) {
        self.symbols = data_by_symbol.keys().cloned().collect();
        self.symbols.sort();
        
        for (symbol, data) in data_by_symbol {
            if data.len() < 30 {
                continue;
            }
            
            let returns: Vec<f64> = data.windows(2)
                .map(|w| (w[1].close - w[0].close) / w[0].close)
                .collect();
            
            let mean = returns.iter().sum::<f64>() / returns.len() as f64;
            let std = (returns.iter()
                .map(|&r| (r - mean).powi(2))
                .sum::<f64>() / returns.len() as f64)
                .sqrt();
            
            self.marginal_distributions.insert(
                symbol.clone(),
                MarginalDist {
                    mean,
                    std,
                    dist_type: DistType::Normal,
                }
            );
        }
        
        self.estimate_correlation(data_by_symbol);
    }
    
    fn estimate_correlation(&mut self, data_by_symbol: &HashMap<String, Vec<EquityData>>) {
        let n = self.symbols.len();
        
        if n < 2 {
            return;
        }
        
        self.correlation_matrix = Array2::zeros((n, n));
        
        let mut returns_by_symbol: HashMap<String, Vec<f64>> = HashMap::new();
        
        for (symbol, data) in data_by_symbol {
            let returns: Vec<f64> = data.windows(2)
                .map(|w| (w[1].close - w[0].close) / w[0].close)
                .collect();
            returns_by_symbol.insert(symbol.clone(), returns);
        }
        
        for i in 0..n {
            for j in 0..n {
                let symbol_i = &self.symbols[i];
                let symbol_j = &self.symbols[j];
                
                if let (Some(returns_i), Some(returns_j)) = (
                    returns_by_symbol.get(symbol_i),
                    returns_by_symbol.get(symbol_j)
                ) {
                    let min_len = returns_i.len().min(returns_j.len());
                    let corr = self.calculate_correlation(
                        &returns_i[..min_len],
                        &returns_j[..min_len]
                    );
                    self.correlation_matrix[[i, j]] = corr;
                } else {
                    self.correlation_matrix[[i, j]] = if i == j { 1.0 } else { 0.0 };
                }
            }
        }
    }
    
    fn calculate_correlation(&self, x: &[f64], y: &[f64]) -> f64 {
        let n = x.len() as f64;
        let mean_x = x.iter().sum::<f64>() / n;
        let mean_y = y.iter().sum::<f64>() / n;
        
        let mut numerator = 0.0;
        let mut denom_x = 0.0;
        let mut denom_y = 0.0;
        
        for (&xi, &yi) in x.iter().zip(y.iter()) {
            let dx = xi - mean_x;
            let dy = yi - mean_y;
            numerator += dx * dy;
            denom_x += dx * dx;
            denom_y += dy * dy;
        }
        
        let denominator = (denom_x * denom_y).sqrt();
        if denominator == 0.0 {
            0.0
        } else {
            numerator / denominator
        }
    }
    
    pub fn simulate(&self, n_simulations: usize) -> Vec<HashMap<String, f64>> {
        if self.symbols.is_empty() {
            return Vec::new();
        }
        
        let normal = Normal::new(0.0, 1.0).unwrap();
        let mut rng = rand::thread_rng();
        
        let standard_normals: Vec<Vec<f64>> = (0..n_simulations)
            .map(|_| {
                (0..self.symbols.len())
                    .map(|_| normal.sample(&mut rng))
                    .collect()
            })
            .collect();
        
        let correlated_normals = self.apply_correlation(&standard_normals);
        
        correlated_normals.iter()
            .map(|row| {
                self.symbols.iter()
                    .enumerate()
                    .filter_map(|(i, symbol)| {
                        self.marginal_distributions.get(symbol)
                            .map(|marginal| {
                                let z = row[i];
                                let value = marginal.mean + marginal.std * z;
                                (symbol.clone(), value)
                            })
                    })
                    .collect()
            })
            .collect()
    }
    
    fn apply_correlation(&self, standard_normals: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let L = self.cholesky_decomposition();
        
        standard_normals.iter()
            .map(|row| {
                let row_array = Array1::from(row.clone());
                let correlated = L.dot(&row_array);
                correlated.to_vec()
            })
            .collect()
    }
    
    fn cholesky_decomposition(&self) -> Array2<f64> {
        let n = self.correlation_matrix.shape()[0];
        let mut L = Array2::zeros((n, n));
        
        for i in 0..n {
            for j in 0..=i {
                let sum = (0..j)
                    .map(|k| L[[i, k]] * L[[j, k]])
                    .sum::<f64>();
                
                if i == j {
                    L[[i, j]] = (self.correlation_matrix[[i, i]] - sum).sqrt();
                } else {
                    L[[i, j]] = (self.correlation_matrix[[i, j]] - sum) / L[[j, j]];
                }
            }
        }
        
        L
    }
    
    pub fn get_probability_distribution(&self, symbol: &str, n_bins: usize) -> Vec<f64> {
        let simulations = self.simulate(10000);
        
        let values: Vec<f64> = simulations.iter()
            .filter_map(|sim| sim.get(symbol).copied())
            .collect();
        
        if values.is_empty() {
            return vec![0.0; n_bins];
        }
        
        let min_val = *values.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let max_val = *values.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        
        if (max_val - min_val).abs() < 1e-10 {
            let mut dist = vec![0.0; n_bins];
            dist[n_bins / 2] = 1.0;
            return dist;
        }
        
        let bin_width = (max_val - min_val) / n_bins as f64;
        let mut distribution = vec![0.0; n_bins];
        
        for value in values {
            let bin = ((value - min_val) / bin_width) as usize;
            let bin_idx = bin.min(n_bins - 1);
            distribution[bin_idx] += 1.0;
        }
        
        let sum: f64 = distribution.iter().sum();
        if sum > 0.0 {
            for prob in distribution.iter_mut() {
                *prob /= sum;
            }
        }
        
        distribution
    }
    
    pub fn get_joint_probability(&self, symbols: &[&str], thresholds: &[f64]) -> f64 {
        let simulations = self.simulate(10000);
        
        let count = simulations.iter()
            .filter(|sim| {
                symbols.iter().zip(thresholds.iter())
                    .all(|(sym, thresh)| {
                        sim.get(*sym)
                            .map(|&val| val > *thresh)
                            .unwrap_or(false)
                    })
            })
            .count();
        
        count as f64 / simulations.len() as f64
    }
    
    pub fn get_conditional_probability(&self, symbol: &str, condition: &str, 
                                        threshold: f64, cond_threshold: f64) -> f64 {
        let simulations = self.simulate(10000);
        
        let count_cond = simulations.iter()
            .filter(|sim| {
                sim.get(condition)
                    .map(|&val| val > cond_threshold)
                    .unwrap_or(false)
            })
            .count();
        
        if count_cond == 0 {
            return 0.0;
        }
        
        let count_joint = simulations.iter()
            .filter(|sim| {
                let sym_val = sim.get(symbol).map(|&v| v > threshold).unwrap_or(false);
                let cond_val = sim.get(condition).map(|&v| v > cond_threshold).unwrap_or(false);
                sym_val && cond_val
            })
            .count();
        
        count_joint as f64 / count_cond as f64
    }
    
    pub fn get_correlation_matrix(&self) -> &Array2<f64> {
        &self.correlation_matrix
    }
    
    pub fn symbols(&self) -> &Vec<String> {
        &self.symbols
    }
}
