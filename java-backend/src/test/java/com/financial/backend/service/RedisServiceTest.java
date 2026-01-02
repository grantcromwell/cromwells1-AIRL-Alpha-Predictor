package com.financial.backend.service;

import com.financial.backend.model.EquityData;
import org.junit.After;
import org.junit.Before;
import org.junit.Test;
import redis.clients.jedis.Jedis;

import java.util.ArrayList;
import java.util.List;

import static org.junit.Assert.*;

public class RedisServiceTest {
    private RedisService redisService;
    
    @Before
    public void setUp() {
        redisService = new RedisService("localhost", 6379);
    }
    
    @After
    public void tearDown() {
        redisService.flushAll();
        redisService.close();
    }
    
    @Test
    public void testStoreAndRetrieveEquityData() {
        EquityData data = new EquityData("TEST", System.currentTimeMillis(), 
                                         100.0, 105.0, 99.0, 104.0, 1000000.0, 104.0);
        
        redisService.storeEquityData(data);
        
        EquityData retrieved = redisService.getEquityData("TEST", data.getTimestamp());
        
        assertNotNull(retrieved);
        assertEquals("TEST", retrieved.getSymbol());
        assertEquals(104.0, retrieved.getClose(), 0.001);
        assertEquals(1000000.0, retrieved.getVolume(), 0.001);
    }
    
    @Test
    public void testStoreBatch() {
        List<EquityData> batch = new ArrayList<>();
        for (int i = 0; i < 10; i++) {
            batch.add(new EquityData("TEST", System.currentTimeMillis() + i, 
                                     100.0 + i, 105.0 + i, 99.0 + i, 104.0 + i, 
                                     1000000.0 + i * 100000, 104.0 + i));
        }
        
        redisService.storeBatch(batch);
        
        List<EquityData> retrieved = redisService.getEquityDataBySymbol("TEST", 10);
        assertEquals(10, retrieved.size());
    }
    
    @Test
    public void testGetEquityDataBySymbol() {
        long currentTime = System.currentTimeMillis();
        List<EquityData> data = new ArrayList<>();
        
        for (int i = 0; i < 20; i++) {
            data.add(new EquityData("SYMBOL", currentTime + i * 1000, 
                                    100.0 + i, 105.0 + i, 99.0 + i, 104.0 + i, 
                                    1000000.0, 104.0 + i));
        }
        
        redisService.storeBatch(data);
        
        List<EquityData> retrieved = redisService.getEquityDataBySymbol("SYMBOL", 10);
        assertEquals(10, retrieved.size());
    }
    
    @Test
    public void testFlushAll() {
        EquityData data = new EquityData("TEST", System.currentTimeMillis(), 
                                         100.0, 105.0, 99.0, 104.0, 1000000.0, 104.0);
        redisService.storeEquityData(data);
        
        redisService.flushAll();
        
        EquityData retrieved = redisService.getEquityData("TEST", data.getTimestamp());
        assertNull(retrieved);
    }
}
