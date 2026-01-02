package com.financial.backend;

import com.financial.backend.api.EquityApiServer;
import com.financial.backend.model.EquityData;
import com.financial.backend.processor.FeatureEngineer;
import com.financial.backend.service.RedisService;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Timer;
import java.util.TimerTask;

public class Main {
    private static final Logger logger = LoggerFactory.getLogger(Main.class);
    
    private static final String[] SYMBOLS = {"NQ", "NVDA", "AMD", "WDC", "SLV", "GS", "NET", "EWJ", "EURUSD", "INRJPY", "BRLGBP"};
    private static final String REDIS_HOST = "localhost";
    private static final int REDIS_PORT = 6379;
    
    public static void main(String[] args) {
        logger.info("Starting Financial Backend System...");
        
        RedisService redisService = new RedisService(REDIS_HOST, REDIS_PORT);
        FeatureEngineer featureEngineer = new FeatureEngineer();
        
        try {
            EquityApiServer apiServer = new EquityApiServer(redisService);
            apiServer.start();
            
            Timer dataIngestionTimer = new Timer("DataIngestion", true);
            dataIngestionTimer.schedule(new TimerTask() {
                @Override
                public void run() {
                    ingestSampleData(redisService, featureEngineer);
                }
            }, 0, 60000);
            
            Timer cleanupTimer = new Timer("Cleanup", true);
            cleanupTimer.schedule(new TimerTask() {
                @Override
                public void run() {
                    logger.info("Performing periodic cleanup (2-week retention handled by Redis TTL)");
                }
            }, 0, 3600000);
            
            logger.info("System started successfully. Press Ctrl+C to stop.");
            
            Runtime.getRuntime().addShutdownHook(new Thread(() -> {
                logger.info("Shutting down gracefully...");
                apiServer.stop();
                redisService.close();
                dataIngestionTimer.cancel();
                cleanupTimer.cancel();
                logger.info("System shutdown complete");
            }));
            
        } catch (IOException e) {
            logger.error("Failed to start API server: {}", e.getMessage());
            System.exit(1);
        }
    }
    
    private static void ingestSampleData(RedisService redisService, FeatureEngineer featureEngineer) {
        try {
            List<EquityData> sampleData = generateSampleData();
            List<EquityData> engineeredData = featureEngineer.engineerFeatures(sampleData);
            
            redisService.storeBatch(engineeredData);
            logger.info("Ingested {} sample records", engineeredData.size());
            
        } catch (Exception e) {
            logger.error("Error ingesting sample data: {}", e.getMessage());
        }
    }
    
    private static List<EquityData> generateSampleData() {
        List<EquityData> data = new ArrayList<>();
        long currentTime = System.currentTimeMillis();
        long oneDay = 24 * 60 * 60 * 1000;
        
        for (String symbol : SYMBOLS) {
            double basePrice = getBasePrice(symbol);
            
            for (int i = 0; i < 100; i++) {
                long timestamp = currentTime - (99 - i) * oneDay;
                double priceChange = (Math.random() - 0.5) * basePrice * 0.05;
                double open = basePrice + priceChange;
                double close = open + (Math.random() - 0.5) * basePrice * 0.02;
                double high = Math.max(open, close) + Math.random() * basePrice * 0.01;
                double low = Math.min(open, close) - Math.random() * basePrice * 0.01;
                double volume = 1000000 + Math.random() * 5000000;
                
                EquityData eq = new EquityData(symbol, timestamp, open, high, low, close, volume, close);
                data.add(eq);
            }
        }
        
        return data;
    }
    
    private static double getBasePrice(String symbol) {
        switch (symbol) {
            case "NQ": return 16000.0;
            case "NVDA": return 500.0;
            case "AMD": return 150.0;
            case "WDC": return 60.0;
            case "SLV": return 25.0;
            case "GS": return 350.0;
            case "NET": return 80.0;
            case "EWJ": return 65.0;
            case "EURUSD": return 1.08;
            case "INRJPY": return 1.80;
            case "BRLGBP": return 0.252;
            default: return 100.0;
        }
    }
}
