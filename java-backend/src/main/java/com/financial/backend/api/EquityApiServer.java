package com.financial.backend.api;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.financial.backend.model.AlphaResult;
import com.financial.backend.model.EquityData;
import com.financial.backend.service.RedisService;
import com.sun.net.httpserver.HttpExchange;
import com.sun.net.httpserver.HttpHandler;
import com.sun.net.httpserver.HttpServer;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.InetSocketAddress;
import java.nio.charset.StandardCharsets;
import java.util.List;

public class EquityApiServer {
    private static final Logger logger = LoggerFactory.getLogger(EquityApiServer.class);
    private static final int PORT = 8080;
    
    private final HttpServer server;
    private final RedisService redisService;
    private final ObjectMapper objectMapper;
    
    public EquityApiServer(RedisService redisService) throws IOException {
        this.redisService = redisService;
        this.objectMapper = new ObjectMapper();
        this.server = HttpServer.create(new InetSocketAddress(PORT), 0);
        
        server.createContext("/api/equity", new EquityHandler());
        server.createContext("/api/equity/symbol", new SymbolHandler());
        server.createContext("/api/equity/all", new AllDataHandler());
        server.createContext("/api/health", new HealthHandler());
        
        server.setExecutor(null);
        logger.info("API server started on port {}", PORT);
    }
    
    public void start() {
        server.start();
        logger.info("API server is running");
    }
    
    public void stop() {
        server.stop(0);
        logger.info("API server stopped");
    }
    
    private class EquityHandler implements HttpHandler {
        @Override
        public void handle(HttpExchange exchange) throws IOException {
            if (!"GET".equals(exchange.getRequestMethod())) {
                sendResponse(exchange, 405, "Method Not Allowed");
                return;
            }
            
            String query = exchange.getRequestURI().getQuery();
            if (query == null) {
                sendResponse(exchange, 400, "Missing query parameters");
                return;
            }
            
            try {
                String symbol = getQueryParam(query, "symbol");
                String timestampStr = getQueryParam(query, "timestamp");
                
                if (symbol == null || timestampStr == null) {
                    sendResponse(exchange, 400, "Missing symbol or timestamp parameter");
                    return;
                }
                
                long timestamp = Long.parseLong(timestampStr);
                EquityData data = redisService.getEquityData(symbol, timestamp);
                
                if (data == null) {
                    sendResponse(exchange, 404, "Data not found");
                    return;
                }
                
                String response = objectMapper.writeValueAsString(data);
                sendResponse(exchange, 200, response);
                
            } catch (Exception e) {
                logger.error("Error processing request: {}", e.getMessage());
                sendResponse(exchange, 500, "Internal Server Error");
            }
        }
    }
    
    private class SymbolHandler implements HttpHandler {
        @Override
        public void handle(HttpExchange exchange) throws IOException {
            if (!"GET".equals(exchange.getRequestMethod())) {
                sendResponse(exchange, 405, "Method Not Allowed");
                return;
            }
            
            String query = exchange.getRequestURI().getQuery();
            if (query == null) {
                sendResponse(exchange, 400, "Missing query parameters");
                return;
            }
            
            try {
                String symbol = getQueryParam(query, "symbol");
                String limitStr = getQueryParam(query, "limit");
                
                if (symbol == null) {
                    sendResponse(exchange, 400, "Missing symbol parameter");
                    return;
                }
                
                int limit = limitStr != null ? Integer.parseInt(limitStr) : 100;
                List<EquityData> data = redisService.getEquityDataBySymbol(symbol, limit);
                
                String response = objectMapper.writeValueAsString(data);
                sendResponse(exchange, 200, response);
                
            } catch (Exception e) {
                logger.error("Error processing request: {}", e.getMessage());
                sendResponse(exchange, 500, "Internal Server Error");
            }
        }
    }
    
    private class AllDataHandler implements HttpHandler {
        @Override
        public void handle(HttpExchange exchange) throws IOException {
            if (!"GET".equals(exchange.getRequestMethod())) {
                sendResponse(exchange, 405, "Method Not Allowed");
                return;
            }
            
            try {
                List<EquityData> data = redisService.getAllRecentData();
                String response = objectMapper.writeValueAsString(data);
                sendResponse(exchange, 200, response);
                
            } catch (Exception e) {
                logger.error("Error processing request: {}", e.getMessage());
                sendResponse(exchange, 500, "Internal Server Error");
            }
        }
    }
    
    private class HealthHandler implements HttpHandler {
        @Override
        public void handle(HttpExchange exchange) throws IOException {
            String response = "{\"status\":\"healthy\",\"timestamp\":" + System.currentTimeMillis() + "}";
            sendResponse(exchange, 200, response);
        }
    }
    
    private void sendResponse(HttpExchange exchange, int statusCode, String response) throws IOException {
        exchange.getResponseHeaders().set("Content-Type", "application/json");
        exchange.sendResponseHeaders(statusCode, response.getBytes(StandardCharsets.UTF_8).length);
        
        try (OutputStream os = exchange.getResponseBody()) {
            os.write(response.getBytes(StandardCharsets.UTF_8));
        }
    }
    
    private String getQueryParam(String query, String param) {
        String[] pairs = query.split("&");
        for (String pair : pairs) {
            String[] keyValue = pair.split("=");
            if (keyValue.length == 2 && keyValue[0].equals(param)) {
                return keyValue[1];
            }
        }
        return null;
    }
}
