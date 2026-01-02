#[cfg(test)]
mod tests {
    use crate::types::{EquityData, TrainingSample};
    use crate::random_forest::RandomForest;
    use crate::gaussian_copula::GaussianCopula;
    use crate::betafish_search::BetafishSearch;
    use crate::exa_search::ExaSearch;
    use std::collections::HashMap;
    
    fn create_sample_data() -> Vec<EquityData> {
        let mut data = Vec::new();
        for i in 0..100 {
            let close = 100.0 + i as f64;
            data.push(EquityData {
                symbol: "TEST".to_string(),
                timestamp: i as i64 * 86400000,
                open: close - 1.0,
                high: close + 1.0,
                low: close - 2.0,
                close,
                volume: 1000000.0,
                adjusted_close: close,
                moving_averages: None,
                rsi: None,
                macd: None,
            });
        }
        data
    }
    
    #[test]
    fn test_equity_data_to_training_sample() {
        let data = create_sample_data();
        let sample = data[50].to_training_sample(1.5);
        
        assert_eq!(sample.features.len(), 15);
        assert_eq!(sample.target, 1.5);
        assert_eq!(sample.symbol, "TEST");
    }
    
    #[test]
    fn test_random_forest_training() {
        let mut samples = Vec::new();
        
        for i in 0..200 {
            let features = vec![
                100.0 + i as f64,
                1000000.0,
                0.02,
                (i as f64 - 100.0) / 100.0,
                100.0, 100.0, 100.0, 100.0, 0.0,
                50.0, 0.0, 0.0, 0.0, 0.01, 0.02,
            ];
            let target = (i as f64 - 100.0) / 100.0;
            samples.push(TrainingSample {
                features,
                target,
                symbol: "TEST".to_string(),
            });
        }
        
        let mut rf = RandomForest::new(10, 5, 2, 5);
        rf.fit(&samples);
        
        assert!(!rf.predict(&samples[0].features).is_nan());
    }
    
    #[test]
    fn test_random_forest_prediction() {
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
        
        let mut rf = RandomForest::new(5, 5, 2, 3);
        rf.fit(&samples);
        
        let features = vec![150.0, 1000000.0, 0.02, 0.5, 100.0, 100.0, 100.0, 100.0, 0.0, 50.0, 0.0, 0.0, 0.0, 0.01, 0.02];
        let prediction = rf.predict(&features);
        
        assert!(!prediction.is_nan());
        assert!(!prediction.is_infinite());
    }
    
    #[test]
    fn test_random_forest_batch_prediction() {
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
        
        let mut rf = RandomForest::new(5, 5, 2, 3);
        rf.fit(&samples);
        
        let feature_list = vec![samples[0].features.clone(), samples[1].features.clone()];
        let predictions = rf.predict_batch(&feature_list);
        
        assert_eq!(predictions.len(), 2);
    }
    
    #[test]
    fn test_gaussian_copula_creation() {
        let copula = GaussianCopula::new();
        assert_eq!(copula.symbols().len(), 0);
    }
    
    #[test]
    fn test_gaussian_copula_fit() {
        let mut copula = GaussianCopula::new();
        let mut data = HashMap::new();
        data.insert("TEST".to_string(), create_sample_data());
        
        copula.fit(&data);
        
        assert!(!copula.symbols().is_empty());
    }
    
    #[test]
    fn test_gaussian_copula_simulation() {
        let mut copula = GaussianCopula::new();
        let mut data = HashMap::new();
        data.insert("TEST".to_string(), create_sample_data());
        
        copula.fit(&data);
        let simulations = copula.simulate(10);
        
        assert_eq!(simulations.len(), 10);
    }
    
    #[test]
    fn test_gaussian_copula_probability_distribution() {
        let mut copula = GaussianCopula::new();
        let mut data = HashMap::new();
        data.insert("TEST".to_string(), create_sample_data());
        
        copula.fit(&data);
        let distribution = copula.get_probability_distribution("TEST", 100);
        
        assert_eq!(distribution.len(), 100);
        let sum: f64 = distribution.iter().sum();
        assert!((sum - 1.0).abs() < 0.01);
    }
    
    #[test]
    fn test_betafish_search() {
        let search = BetafishSearch::new(30, 5);
        let mut data = HashMap::new();
        data.insert("TEST".to_string(), create_sample_data());
        
        let results = search.find_strongest_movers(&data);
        
        assert!(results.len() <= 5);
    }
    
    #[test]
    fn test_betafish_search_highest_volume() {
        let search = BetafishSearch::new(30, 5);
        let mut data = HashMap::new();
        data.insert("TEST".to_string(), create_sample_data());
        
        let results = search.find_highest_volume(&data);
        
        assert!(results.len() <= 5);
    }
    
    #[test]
    fn test_exa_search() {
        let mut search = ExaSearch::new(0.5, 5);
        let mut data = HashMap::new();
        data.insert("TEST".to_string(), create_sample_data());
        
        search.fit(&data);
        let results = search.find_highest_probable_alpha(&data);
        
        assert!(results.len() <= 5);
    }
}
