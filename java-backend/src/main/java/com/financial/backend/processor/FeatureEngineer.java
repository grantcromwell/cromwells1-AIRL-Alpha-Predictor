package com.financial.backend.processor;

import com.financial.backend.model.EquityData;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class FeatureEngineer {
    private static final Logger logger = LoggerFactory.getLogger(FeatureEngineer.class);
    
    public List<EquityData> engineerFeatures(List<EquityData> data) {
        if (data.isEmpty()) {
            return data;
        }
        
        data.sort((a, b) -> Long.compare(a.getTimestamp(), b.getTimestamp()));
        
        Map<String, List<EquityData>> symbolGroups = new HashMap<>();
        for (EquityData eq : data) {
            symbolGroups.computeIfAbsent(eq.getSymbol(), k -> new ArrayList<>()).add(eq);
        }
        
        List<EquityData> processedData = new ArrayList<>();
        for (List<EquityData> symbolData : symbolGroups.values()) {
            calculateMovingAverages(symbolData);
            calculateRSI(symbolData);
            calculateMACD(symbolData);
            calculatePriceChange(symbolData);
            calculateVolatility(symbolData);
            processedData.addAll(symbolData);
        }
        
        logger.info("Feature engineering completed for {} records", processedData.size());
        return processedData;
    }
    
    private void calculateMovingAverages(List<EquityData> data) {
        for (int i = 4; i < data.size(); i++) {
            EquityData current = data.get(i);
            Map<String, Double> ma = new HashMap<>();
            
            ma.put("ma_5", calculateMA(data, i, 5));
            
            if (i >= 9) {
                ma.put("ma_10", calculateMA(data, i, 10));
            }
            if (i >= 19) {
                ma.put("ma_20", calculateMA(data, i, 20));
            }
            if (i >= 49) {
                ma.put("ma_50", calculateMA(data, i, 50));
            }
            
            current.setMovingAverages(ma);
        }
    }
    
    private double calculateMA(List<EquityData> data, int index, int period) {
        double sum = 0;
        for (int i = index - period + 1; i <= index; i++) {
            sum += data.get(i).getClose();
        }
        return sum / period;
    }
    
    private void calculateRSI(List<EquityData> data) {
        if (data.size() < 15) return;
        
        int period = 14;
        for (int i = period; i < data.size(); i++) {
            double gains = 0;
            double losses = 0;
            
            for (int j = i - period + 1; j <= i; j++) {
                double change = data.get(j).getClose() - data.get(j - 1).getClose();
                if (change > 0) {
                    gains += change;
                } else {
                    losses -= change;
                }
            }
            
            double avgGain = gains / period;
            double avgLoss = losses / period;
            
            if (avgLoss == 0) {
                data.get(i).setRsi(100.0);
            } else {
                double rs = avgGain / avgLoss;
                double rsi = 100 - (100 / (1 + rs));
                data.get(i).setRsi(rsi);
            }
        }
    }
    
    private void calculateMACD(List<EquityData> data) {
        if (data.size() < 26) return;
        
        double[] ema12 = new double[data.size()];
        double[] ema26 = new double[data.size()];
        
        ema12[11] = calculateSMA(data, 0, 12);
        for (int i = 12; i < data.size(); i++) {
            ema12[i] = (data.get(i).getClose() * (2.0 / 13.0)) + (ema12[i - 1] * (11.0 / 13.0));
        }
        
        ema26[25] = calculateSMA(data, 0, 26);
        for (int i = 26; i < data.size(); i++) {
            ema26[i] = (data.get(i).getClose() * (2.0 / 27.0)) + (ema26[i - 1] * (25.0 / 27.0));
        }
        
        for (int i = 26; i < data.size(); i++) {
            Map<String, Double> macd = new HashMap<>();
            double macdLine = ema12[i] - ema26[i];
            macd.put("macd", macdLine);
            
            if (i >= 34) {
                double signalLine = calculateSignalMACD(ema12, ema26, i, 9);
                macd.put("signal", signalLine);
                macd.put("histogram", macdLine - signalLine);
            }
            
            data.get(i).setMacd(macd);
        }
    }
    
    private double calculateSMA(List<EquityData> data, int start, int period) {
        double sum = 0;
        for (int i = start; i < start + period; i++) {
            sum += data.get(i).getClose();
        }
        return sum / period;
    }
    
    private double calculateSignalMACD(double[] ema12, double[] ema26, int index, int period) {
        double sum = 0;
        for (int i = index - period + 1; i <= index; i++) {
            sum += ema12[i] - ema26[i];
        }
        return sum / period;
    }
    
    private void calculatePriceChange(List<EquityData> data) {
        for (int i = 1; i < data.size(); i++) {
            double change = ((data.get(i).getClose() - data.get(i - 1).getClose()) / data.get(i - 1).getClose()) * 100;
        }
    }
    
    private void calculateVolatility(List<EquityData> data) {
        int period = 20;
        for (int i = period; i < data.size(); i++) {
            double sum = 0;
            for (int j = i - period + 1; j <= i; j++) {
                double change = data.get(j).getClose() - data.get(j - 1).getClose();
                sum += change * change;
            }
            double volatility = Math.sqrt(sum / period);
        }
    }
}
