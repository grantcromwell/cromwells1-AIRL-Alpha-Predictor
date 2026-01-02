package com.financial.backend.service;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.financial.backend.model.EquityData;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import redis.clients.jedis.Jedis;
import redis.clients.jedis.JedisPool;
import redis.clients.jedis.JedisPoolConfig;

import java.io.IOException;
import java.time.Duration;
import java.util.ArrayList;
import java.util.List;
import java.util.Set;

public class RedisService {
    private static final Logger logger = LoggerFactory.getLogger(RedisService.class);
    private static final int TWO_WEEKS_SECONDS = 14 * 24 * 60 * 60;
    
    private final JedisPool jedisPool;
    private final ObjectMapper objectMapper;
    
    public RedisService(String host, int port) {
        JedisPoolConfig poolConfig = new JedisPoolConfig();
        poolConfig.setMaxTotal(10);
        poolConfig.setMaxIdle(5);
        poolConfig.setMinIdle(1);
        poolConfig.setTestOnBorrow(true);
        poolConfig.setTestWhileIdle(true);
        poolConfig.setMinEvictableIdleTimeMillis(Duration.ofSeconds(60).toMillis());
        poolConfig.setTimeBetweenEvictionRunsMillis(Duration.ofSeconds(30).toMillis());
        
        this.jedisPool = new JedisPool(poolConfig, host, port);
        this.objectMapper = new ObjectMapper();
        logger.info("Redis service initialized with host: {}, port: {}", host, port);
    }
    
    public void storeEquityData(EquityData data) {
        try (Jedis jedis = jedisPool.getResource()) {
            String key = generateKey(data.getSymbol(), data.getTimestamp());
            String value = objectMapper.writeValueAsString(data);
            
            jedis.setex(key, TWO_WEEKS_SECONDS, value);
            
            String symbolIndexKey = "symbol:" + data.getSymbol();
            jedis.zadd(symbolIndexKey, data.getTimestamp(), key);
            jedis.expire(symbolIndexKey, TWO_WEEKS_SECONDS);
            
            logger.debug("Stored data for key: {}", key);
        } catch (IOException e) {
            logger.error("Error serializing equity data: {}", e.getMessage());
            throw new RuntimeException("Failed to serialize equity data", e);
        }
    }
    
    public void storeBatch(List<EquityData> dataList) {
        try (Jedis jedis = jedisPool.getResource()) {
            redis.clients.jedis.Pipeline pipeline = jedis.pipelined();
            
            for (EquityData data : dataList) {
                String key = generateKey(data.getSymbol(), data.getTimestamp());
                String value = objectMapper.writeValueAsString(data);
                
                pipeline.setex(key, TWO_WEEKS_SECONDS, value);
                
                String symbolIndexKey = "symbol:" + data.getSymbol();
                pipeline.zadd(symbolIndexKey, data.getTimestamp(), key);
                pipeline.expire(symbolIndexKey, TWO_WEEKS_SECONDS);
            }
            
            pipeline.sync();
            logger.info("Batch stored {} records", dataList.size());
        } catch (IOException e) {
            logger.error("Error serializing batch data: {}", e.getMessage());
            throw new RuntimeException("Failed to serialize batch data", e);
        }
    }
    
    public EquityData getEquityData(String symbol, long timestamp) {
        try (Jedis jedis = jedisPool.getResource()) {
            String key = generateKey(symbol, timestamp);
            String value = jedis.get(key);
            
            if (value == null) {
                logger.debug("No data found for key: {}", key);
                return null;
            }
            
            return objectMapper.readValue(value, EquityData.class);
        } catch (IOException e) {
            logger.error("Error deserializing equity data: {}", e.getMessage());
            throw new RuntimeException("Failed to deserialize equity data", e);
        }
    }
    
    public List<EquityData> getEquityDataBySymbol(String symbol, long startTime, long endTime) {
        try (Jedis jedis = jedisPool.getResource()) {
            String pattern = "equity:" + symbol + ":*";
            
            List<EquityData> results = new ArrayList<>();
            Set<String> keys = jedis.keys(pattern);
            
            if (keys != null) {
                for (String key : keys) {
                    long timestamp = Long.parseLong(key.split(":")[2]);
                    if (timestamp >= startTime && timestamp <= endTime) {
                        String value = jedis.get(key);
                        if (value != null) {
                            results.add(objectMapper.readValue(value, EquityData.class));
                        }
                    }
                }
            }
            
            logger.info("Retrieved {} records for symbol {} between {} and {}", 
                       results.size(), symbol, startTime, endTime);
            return results;
        } catch (IOException e) {
            logger.error("Error retrieving equity data: {}", e.getMessage());
            throw new RuntimeException("Failed to retrieve equity data", e);
        }
    }
    
    public List<EquityData> getEquityDataBySymbol(String symbol, int limit) {
        try (Jedis jedis = jedisPool.getResource()) {
            String pattern = "equity:" + symbol + ":*";
            
            List<EquityData> results = new ArrayList<>();
            Set<String> keys = jedis.keys(pattern);
            
            if (keys != null && !keys.isEmpty()) {
                List<String> keyList = new ArrayList<>(keys);
                int start = Math.max(0, keyList.size() - limit);
                
                for (int i = start; i < keyList.size(); i++) {
                    String value = jedis.get(keyList.get(i));
                    if (value != null) {
                        results.add(objectMapper.readValue(value, EquityData.class));
                    }
                }
            }
            
            return results;
        } catch (IOException e) {
            logger.error("Error retrieving equity data by limit: {}", e.getMessage());
            throw new RuntimeException("Failed to retrieve equity data", e);
        }
    }
    
    public List<EquityData> getAllRecentData() {
        List<EquityData> allData = new ArrayList<>();
        String[] symbols = {"NQ", "NVDA", "AMD", "WDC", "SLV", "GS", "NET", "EWJ", "EURUSD", "INRJPY", "BRLGBP"};
        
        for (String symbol : symbols) {
            List<EquityData> symbolData = getEquityDataBySymbol(symbol, 100);
            allData.addAll(symbolData);
        }
        
        return allData;
    }
    
    public void flushAll() {
        try (Jedis jedis = jedisPool.getResource()) {
            jedis.flushAll();
            logger.warn("Redis cache flushed");
        }
    }
    
    public void close() {
        if (jedisPool != null) {
            jedisPool.close();
            logger.info("Redis service closed");
        }
    }
    
    private String generateKey(String symbol, long timestamp) {
        return "equity:" + symbol + ":" + timestamp;
    }
}
