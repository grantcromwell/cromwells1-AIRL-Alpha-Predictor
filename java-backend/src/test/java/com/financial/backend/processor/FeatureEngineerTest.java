package com.financial.backend.processor;

import com.financial.backend.model.EquityData;
import org.junit.Before;
import org.junit.Test;

import java.util.ArrayList;
import java.util.List;

import static org.junit.Assert.*;

public class FeatureEngineerTest {
    private FeatureEngineer featureEngineer;
    private List<EquityData> testData;
    
    @Before
    public void setUp() {
        featureEngineer = new FeatureEngineer();
        testData = generateTestData();
    }
    
    @Test
    public void testEngineerFeatures() {
        List<EquityData> processed = featureEngineer.engineerFeatures(testData);
        
        assertNotNull(processed);
        assertEquals(testData.size(), processed.size());
    }
    
    @Test
    public void testMovingAverages() {
        List<EquityData> processed = featureEngineer.engineerFeatures(testData);
        
        EquityData processedData = processed.get(50);
        assertNotNull(processedData.getMovingAverages());
        assertTrue(processedData.getMovingAverages().containsKey("ma_5"));
        assertTrue(processedData.getMovingAverages().containsKey("ma_10"));
        assertTrue(processedData.getMovingAverages().containsKey("ma_20"));
    }
    
    @Test
    public void testRSI() {
        List<EquityData> processed = featureEngineer.engineerFeatures(testData);
        
        EquityData processedData = processed.get(50);
        assertNotNull(processedData.getRsi());
        assertTrue(processedData.getRsi() >= 0);
        assertTrue(processedData.getRsi() <= 100);
    }
    
    @Test
    public void testMACD() {
        List<EquityData> processed = featureEngineer.engineerFeatures(testData);
        
        EquityData processedData = processed.get(50);
        assertNotNull(processedData.getMacd());
        assertTrue(processedData.getMacd().containsKey("macd"));
    }
    
    private List<EquityData> generateTestData() {
        List<EquityData> data = new ArrayList<>();
        long currentTime = System.currentTimeMillis();
        long oneDay = 24 * 60 * 60 * 1000;
        
        for (int i = 0; i < 100; i++) {
            double close = 100.0 + i + Math.random();
            data.add(new EquityData(
                "TEST",
                currentTime + i * oneDay,
                close - 1.0,
                close + 1.0,
                close - 2.0,
                close,
                1000000.0,
                close
            ));
        }
        
        return data;
    }
}
