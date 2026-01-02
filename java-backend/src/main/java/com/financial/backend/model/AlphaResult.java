package com.financial.backend.model;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.List;

public class AlphaResult {
    @JsonProperty("symbol")
    private String symbol;
    
    @JsonProperty("alpha")
    private double alpha;
    
    @JsonProperty("probability")
    private double probability;
    
    @JsonProperty("volume")
    private double volume;
    
    @JsonProperty("change")
    private double change;
    
    public AlphaResult() {}
    
    public AlphaResult(String symbol, double alpha, double probability, double volume, double change) {
        this.symbol = symbol;
        this.alpha = alpha;
        this.probability = probability;
        this.volume = volume;
        this.change = change;
    }
    
    public String getSymbol() { return symbol; }
    public void setSymbol(String symbol) { this.symbol = symbol; }
    
    public double getAlpha() { return alpha; }
    public void setAlpha(double alpha) { this.alpha = alpha; }
    
    public double getProbability() { return probability; }
    public void setProbability(double probability) { this.probability = probability; }
    
    public double getVolume() { return volume; }
    public void setVolume(double volume) { this.volume = volume; }
    
    public double getChange() { return change; }
    public void setChange(double change) { this.change = change; }
}
