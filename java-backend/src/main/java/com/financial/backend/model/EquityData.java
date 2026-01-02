package com.financial.backend.model;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.List;
import java.util.Map;

public class EquityData {
    @JsonProperty("symbol")
    private String symbol;
    
    @JsonProperty("timestamp")
    private long timestamp;
    
    @JsonProperty("open")
    private double open;
    
    @JsonProperty("high")
    private double high;
    
    @JsonProperty("low")
    private double low;
    
    @JsonProperty("close")
    private double close;
    
    @JsonProperty("volume")
    private double volume;
    
    @JsonProperty("adjusted_close")
    private double adjustedClose;
    
    @JsonProperty("moving_averages")
    private Map<String, Double> movingAverages;
    
    @JsonProperty("rsi")
    private Double rsi;
    
    @JsonProperty("macd")
    private Map<String, Double> macd;
    
    public EquityData() {}
    
    public EquityData(String symbol, long timestamp, double open, double high, double low, 
                     double close, double volume, double adjustedClose) {
        this.symbol = symbol;
        this.timestamp = timestamp;
        this.open = open;
        this.high = high;
        this.low = low;
        this.close = close;
        this.volume = volume;
        this.adjustedClose = adjustedClose;
    }
    
    public String getSymbol() { return symbol; }
    public void setSymbol(String symbol) { this.symbol = symbol; }
    
    public long getTimestamp() { return timestamp; }
    public void setTimestamp(long timestamp) { this.timestamp = timestamp; }
    
    public double getOpen() { return open; }
    public void setOpen(double open) { this.open = open; }
    
    public double getHigh() { return high; }
    public void setHigh(double high) { this.high = high; }
    
    public double getLow() { return low; }
    public void setLow(double low) { this.low = low; }
    
    public double getClose() { return close; }
    public void setClose(double close) { this.close = close; }
    
    public double getVolume() { return volume; }
    public void setVolume(double volume) { this.volume = volume; }
    
    public double getAdjustedClose() { return adjustedClose; }
    public void setAdjustedClose(double adjustedClose) { this.adjustedClose = adjustedClose; }
    
    public Map<String, Double> getMovingAverages() { return movingAverages; }
    public void setMovingAverages(Map<String, Double> movingAverages) { this.movingAverages = movingAverages; }
    
    public Double getRsi() { return rsi; }
    public void setRsi(Double rsi) { this.rsi = rsi; }
    
    public Map<String, Double> getMacd() { return macd; }
    public void setMacd(Map<String, Double> macd) { this.macd = macd; }
}
