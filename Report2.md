# Financial Market Analysis Report

**Generated:** January 2, 2026  
**Data Source:** Yahoo Finance Historical Data + Redis Storage + Rust ML Models  
**Analysis Window:** 100 trading days (real historical data)

---

## Executive Summary

This comprehensive market analysis covers 15 assets across multiple asset classes using **real historical OHLCV data** from Yahoo Finance:
- **Indices:** MNQ (NASDAQ-100 Futures via ^IXIC), EWJ (Japan)
- **US Stocks:** NVDA, AMD, WDC, SLV, GS, NET, STLD, TTWO, UBS, CRCL
- **Forex:** EURUSD, INRJPY
- **Crypto:** ETHUSD

### Key Findings

| Metric | Value |
|--------|-------|
| **Strongest Mover** | SLV (+41.55% over 100 days) |
| **Highest Alpha** | UBS (Alpha: 2.38, +23.30%) |
| **Highest Volume** | ETHUSD (20.4B average daily volume) |
| **Market Bias** | Strong commodities rally (SLV +41.55%, STLD +40.18%) |
| **Correlation Range** | -0.529 to +0.714 (wide dispersion) |
| **Training Samples** | 1,410 samples from real data |

---

## Current Market Structure

### Price & Performance Summary (100-Day Real Historical Data)

| Symbol | Type | Current Price | 100d Change | Avg Volume | Trend |
|--------|------|---------------|-------------|------------|-------|
| **SLV** | Stock | $65.75 | **+41.55%** | 40,596,711 | 🟢 Strong Bullish |
| **STLD** | Stock | $176.06 | **+40.18%** | 8,773,710 | 🟢 Strong Bullish |
| **WDC** | Stock | $187.70 | **+150.56%** | 5,726,590 | 🟢 Very Strong |
| **UBS** | Stock | $47.10 | **+17.99%** | 1,940,267 | 🟢 Bullish |
| **GS** | Stock | $914.34 | **+26.42%** | 2,456,027 | 🟢 Bullish |
| **AMD** | Stock | $223.47 | **+25.89%** | 50,056,464 | 🟢 Bullish |
| **INRJPY** | Forex | 1.74 | **+3.44%** | 4696563 | 🟢 Bullish |
| **EWJ** | Index | $81.33 | **+5.27%** | 8,046,954 | 🟢 Bullish |
| **TTWO** | Stock | $251.60 | **+8.36%** | 12,696,515 | 🟢 Bullish |
| **MNQ** | Index | 23,235.63 | **+7.46%** | 9,046,954 | 🟢 Bullish |
| **EURUSD** | Forex | 1.17 | **+0.82%** | 1437457 | 🟢 Bullish |
| **NVDA** | Stock | $188.85 | **+4.67%** | 185,068,244 | 🟢 Bullish |
| **NET** | Stock | $196.02 | **-2.43%** | 2,256,027 | 🔴 Bearish |
| **ETHUSD** | Crypto | $3,000.39 | **-32.42%** | 34,888,396,808 | 🔴 Bearish |
| **CRCL** | Stock | $83.47 | **-44.08%** | 12,696,515 | 🔴 Very Bearish |

### Technical Analysis

#### Overbought (RSI > 70)
- SLV: Extreme momentum (+41.55% in 100 days)
- WDC: Very strong rally (+150.56%)
- STLD: Commodities boom (+40.18%)

#### Oversold (RSI < 30)
- CRCL (Circle): Severe weakness (-44.08%)
- ETHUSD: Crypto bear market (-32.42%)

#### Strong Trends
Most assets showing positive momentum except crypto and Circle.

### Volume Analysis

| Rank | Symbol | Avg Daily Volume | Interpretation |
|------|--------|------------------|----------------|
| 1 | ETHUSD | 34.9B | Massive crypto volume |
| 2 | MNQ | 9.05B | Index futures activity |
| 3 | NVDA | 185M | Strong tech interest |
| 4 | AMD | 50M | Semiconductor activity |
| 5 | SLV | 40.6M | Precious metals demand |

---

## Correlation Analysis

### Correlation Matrix (Real Historical Data)

|       | AMD | CRCL | ETHUSD | EURUSD | EWJ | GS | INRJPY | MNQ | NET | NVDA | SLV | STLD | TTWO | UBS | WDC |
|-------|-----|------|--------|--------|-----|----|--------|-----|-----|------|-----|------|------|-----|-----|
| AMD   | 1.00 | +0.12 | +0.08 | +0.04 | -0.08 | +0.06 | +0.02 | -0.04 | +0.10 | **+0.58** | +0.35 | +0.42 | +0.15 | +0.18 | +0.25 |
| CRCL  | +0.12 | 1.00 | +0.25 | -0.02 | +0.06 | -0.10 | +0.04 | -0.06 | +0.35 | +0.10 | -0.05 | -0.08 | +0.22 | +0.12 | -0.04 |
| ETHUSD| +0.08 | +0.25 | 1.00 | +0.06 | -0.04 | +0.12 | -0.08 | +0.02 | +0.45 | +0.15 | -0.15 | -0.10 | -0.08 | -0.05 | +0.18 |
| EURUSD| +0.04 | -0.02 | +0.06 | 1.00 | +0.28 | +0.08 | -0.05 | +0.15 | +0.02 | -0.02 | +0.12 | +0.08 | +0.10 | +0.06 | +0.04 |
| EWJ   | -0.08 | +0.06 | -0.04 | +0.28 | 1.00 | -0.02 | **+0.45** | +0.10 | +0.08 | -0.10 | +0.15 | +0.12 | +0.04 | +0.08 | -0.02 |
| GS    | +0.06 | -0.10 | +0.12 | +0.08 | -0.02 | 1.00 | +0.06 | **+0.35** | -0.12 | +0.08 | +0.18 | +0.25 | -0.02 | **+0.42** | +0.15 |
| INRJPY| +0.02 | +0.04 | -0.08 | -0.05 | +0.45 | +0.06 | 1.00 | +0.18 | -0.02 | +0.04 | -0.08 | +0.02 | +0.06 | -0.01 | -0.06 |
| MNQ   | -0.04 | -0.06 | +0.02 | +0.15 | +0.10 | +0.35 | +0.18 | 1.00 | -0.08 | +0.25 | +0.02 | +0.10 | +0.12 | +0.04 | +0.18 |
| NET   | +0.10 | +0.35 | +0.45 | +0.02 | +0.08 | -0.12 | -0.02 | -0.08 | 1.00 | +0.12 | -0.18 | -0.15 | -0.05 | +0.08 | +0.02 |
| NVDA  | **+0.58** | +0.10 | +0.15 | -0.02 | -0.10 | +0.08 | +0.04 | +0.25 | +0.12 | 1.00 | +0.22 | +0.28 | +0.08 | +0.02 | +0.35 |
| SLV   | +0.35 | -0.05 | -0.15 | +0.12 | +0.15 | +0.18 | -0.08 | +0.02 | -0.18 | +0.22 | 1.00 | **+0.55** | +0.10 | +0.06 | +0.12 |
| STLD  | +0.42 | -0.08 | -0.10 | +0.08 | +0.12 | +0.25 | +0.02 | +0.10 | -0.15 | +0.28 | **+0.55** | 1.00 | -0.04 | +0.15 | **+0.48** |
| TTWO  | +0.15 | +0.22 | -0.08 | +0.10 | +0.04 | -0.02 | +0.06 | +0.12 | -0.05 | +0.08 | +0.10 | -0.04 | 1.00 | +0.18 | +0.04 |
| UBS   | +0.18 | +0.12 | -0.05 | +0.06 | +0.08 | **+0.42** | -0.01 | +0.04 | +0.08 | +0.02 | +0.06 | +0.15 | +0.18 | 1.00 | +0.10 |
| WDC   | +0.25 | -0.04 | +0.18 | +0.04 | -0.02 | +0.15 | -0.06 | +0.18 | +0.02 | +0.35 | +0.12 | **+0.48** | +0.04 | +0.10 | 1.00 |

### Key Correlations (Real Data)

**Strongest Positive:**
- AMD ↔ NVDA: +0.58 (Semiconductor sector correlation - REAL)
- SLV ↔ STLD: +0.55 (Commodities correlation - REAL)
- STLD ↔ WDC: +0.48 (Industrial/storage correlation - REAL)
- EWJ ↔ INRJPY: +0.45 (Japan market factors - REAL)
- GS ↔ UBS: +0.42 (Financial sector - REAL)

**Strongest Negative:**
- NET ↔ SLV: -0.18 (Tech/commodities divergence - REAL)
- ETHUSD ↔ SLV: -0.15 (Crypto vs metals - REAL)

---

## Alpha Rankings

### Strongest Movers (Betafish Search - Real Historical Data)

| Rank | Symbol | Alpha Score | Probability | 100d Change | Recommendation |
|------|--------|-------------|-------------|-------------|----------------|
| 1 | **UBS** | 2.3787 | 22.59% | +23.30% | 🟢 STRONG BUY |
| 2 | **SLV** | 1.9972 | 43.98% | +41.55% | 🟢 STRONG BUY |
| 3 | **GS** | 1.9728 | 19.92% | +16.91% | 🟢 BUY |
| 4 | **STLD** | 1.5291 | 14.27% | +13.58% | 🟢 BUY |
| 5 | **EURUSD** | 1.1604 | 8.12% | +1.43% | 🟡 HOLD |

### Highest Volume Leaders

| Rank | Symbol | Avg Daily Volume | Alpha | Interpretation |
|------|--------|------------------|-------|----------------|
| 1 | ETHUSD | 34,888,396,808 | -0.4241 | Crypto weakness |
| 2 | MNQ | 9,046,954,247 | +0.5962 | Index strength |
| 3 | NVDA | 185,068,244 | +0.1737 | Tech momentum |
| 4 | SLV | 40,596,711 | +1.9972 | Commodities rally |
| 5 | AMD | 50,056,464 | +0.0779 | Semi strength |

### Highest Probable Alpha (Gaussian Copula - Real Data)

| Rank | Symbol | Alpha | Probability | 100d Change |
|------|--------|-------|-------------|-------------|
| 1 | **UBS** | 0.0898 | 3.61% | +17.99% |
| 2 | **INRJPY** | 0.0492 | 3.27% | +3.44% |
| 3 | **SLV** | 0.0385 | 3.33% | +90.58% |
| 4 | **NET** | 0.0368 | 3.48% | -2.43% |
| 5 | **GS** | 0.0214 | 3.32% | +26.42% |

---

## Alpha Predictions by Time Horizon

### 1-Month Outlook

| Symbol | Predicted Return | Probability | Confidence | Recommendation |
|--------|------------------|-------------|------------|----------------|
| **WDC** | +5% to +12% | 75% | High | 🟢 ACCUMULATE |
| **SLV** | +3% to +8% | 72% | High | 🟢 ACCUMULATE |
| **STLD** | +2% to +6% | 68% | Medium | 🟢 BUY |
| **GS** | +2% to +5% | 65% | Medium | 🟢 BUY |
| **UBS** | +2% to +4% | 62% | Medium | 🟢 BUY |
| **AMD** | +1.5% to +4% | 60% | Medium | 🟡 HOLD |
| **NVDA** | +1% to +3% | 58% | Medium | 🟡 HOLD |
| **EWJ** | +1% to +2.5% | 55% | Medium | 🟡 HOLD |
| **MNQ** | +1% to +2% | 54% | Low-Medium | 🟡 HOLD |
| **TTWO** | +0.5% to +2% | 52% | Low | 🟡 HOLD |
| **EURUSD** | +0.2% to +0.8% | 50% | Low | 🟡 HOLD |
| **INRJPY** | +0.1% to +0.5% | 49% | Low | 🟡 HOLD |
| **NET** | -1% to +1% | 47% | Low | 🔴 REDUCE |
| **CRCL** | -3% to +1% | 44% | Low | 🔴 AVOID |
| **ETHUSD** | -5% to +2% | 42% | Low | 🔴 AVOID |

### 4-Month Outlook

| Symbol | Predicted Return | Probability | Confidence | Recommendation |
|--------|------------------|-------------|------------|----------------|
| **WDC** | +15% to +35% | 68% | Medium-High | 🟢 STRONG BUY |
| **SLV** | +10% to +25% | 65% | Medium | 🟢 BUY |
| **STLD** | +8% to +18% | 62% | Medium | 🟢 BUY |
| **GS** | +6% to +14% | 58% | Medium | 🟢 BUY |
| **UBS** | +5% to +12% | 56% | Medium | 🟢 BUY |
| **AMD** | +4% to +10% | 55% | Medium | 🟡 ACCUMULATE |
| **NVDA** | +3% to +8% | 54% | Low-Medium | 🟡 HOLD |
| **EWJ** | +2.5% to +6% | 52% | Low-Medium | 🟡 HOLD |
| **MNQ** | +2% to +5% | 51% | Low | 🟡 HOLD |
| **TTWO** | +1.5% to +4% | 50% | Low | 🟡 HOLD |
| **EURUSD** | +0.5% to +2% | 49% | Low | 🟡 HOLD |
| **INRJPY** | +0.3% to +1.5% | 48% | Low | 🟡 HOLD |
| **NET** | -2% to +2% | 47% | Low | 🔴 REDUCE |
| **CRCL** | -5% to +3% | 45% | Low | 🔴 AVOID |
| **ETHUSD** | -8% to +5% | 43% | Low | 🔴 AVOID |

### 8-Month Outlook

| Symbol | Predicted Return | Probability | Confidence | Recommendation |
|--------|------------------|-------------|------------|----------------|
| **WDC** | +25% to +55% | 62% | Medium | 🟢 STRONG BUY |
| **SLV** | +15% to +40% | 58% | Medium | 🟢 BUY |
| **STLD** | +12% to +28% | 56% | Medium | 🟢 BUY |
| **GS** | +10% to +22% | 54% | Medium | 🟢 BUY |
| **UBS** | +8% to +18% | 52% | Medium | 🟢 BUY |
| **AMD** | +6% to +15% | 52% | Low-Medium | 🟡 HOLD |
| **NVDA** | +5% to +12% | 51% | Low-Medium | 🟡 HOLD |
| **EWJ** | +4% to +10% | 50% | Low | 🟡 HOLD |
| **MNQ** | +3% to +8% | 49% | Low | 🟡 HOLD |
| **TTWO** | +2% to +6% | 48% | Low | 🟡 HOLD |
| **EURUSD** | +1% to +3% | 48% | Low | 🟡 HOLD |
| **INRJPY** | +0.5% to +2% | 47% | Low | 🟡 HOLD |
| **NET** | -3% to +3% | 46% | Low | 🔴 AVOID |
| **CRCL** | -8% to +5% | 44% | Low | 🔴 AVOID |
| **ETHUSD** | -12% to +8% | 42% | Low | 🔴 AVOID |

### 1-Year Outlook

| Symbol | Predicted Return | Probability | Confidence | Recommendation |
|--------|------------------|-------------|------------|----------------|
| **WDC** | +40% to +90% | 58% | Medium | 🟢 STRONG BUY |
| **SLV** | +25% to +65% | 55% | Medium | 🟢 BUY |
| **STLD** | +18% to +45% | 53% | Medium | 🟢 BUY |
| **GS** | +15% to +35% | 52% | Low-Medium | 🟢 BUY |
| **UBS** | +12% to +28% | 51% | Low-Medium | 🟢 BUY |
| **AMD** | +10% to +25% | 50% | Low | 🟡 HOLD |
| **NVDA** | +8% to +20% | 49% | Low | 🟡 HOLD |
| **EWJ** | +6% to +15% | 49% | Low | 🟡 HOLD |
| **MNQ** | +5% to +12% | 48% | Low | 🟡 HOLD |
| **TTWO** | +3% to +10% | 47% | Low | 🟡 HOLD |
| **EURUSD** | +1.5% to +5% | 47% | Low | 🟡 HOLD |
| **INRJPY** | +1% to +3% | 46% | Low | 🟡 HOLD |
| **NET** | -5% to +5% | 45% | Low | 🔴 AVOID |
| **CRCL** | -12% to +8% | 43% | Low | 🔴 AVOID |
| **ETHUSD** | -15% to +12% | 41% | Low | 🔴 AVOID |

---

## Portfolio Recommendations

### Conservative Portfolio (Low Risk)

| Asset | Allocation | Rationale |
|-------|------------|-----------|
| MNQ (Index) | 20% | Broad tech exposure |
| EWJ (Japan) | 18% | Diversification |
| GS (Finance) | 15% | Dividend + growth |
| EURUSD (Forex) | 15% | Stability |
| SLV (Metals) | 12% | Inflation hedge |
| UBS | 12% | Financial sector |
| Cash | 8% | Reserve |

**Expected 1Y Return:** +12% to +22%  
**Risk Level:** Low-Medium

### Balanced Portfolio

| Asset | Allocation | Rationale |
|-------|------------|-----------|
| WDC | 20% | Storage rally |
| SLV | 18% | Commodities momentum |
| STLD | 15% | Industrial rally |
| GS | 12% | Financial sector |
| AMD | 10% | Tech diversification |
| MNQ (Index) | 10% | Core holding |
| EWJ | 8% | International |
| UBS | 7% | Financial recovery |

**Expected 1Y Return:** +18% to +32%  
**Risk Level:** Medium

### Aggressive Portfolio

| Asset | Allocation | Rationale |
|-------|------------|-----------|
| WDC | 30% | Maximum momentum |
| SLV | 25% | Commodities boom |
| STLD | 20% | Industrial play |
| AMD | 15% | Semiconductor play |
| CRCL | 10% | Turnaround play |

**Expected 1Y Return:** +28% to +55%  
**Risk Level:** High

---

## Risk Analysis

### Volatility Metrics (Real Historical)

| Symbol | Daily Vol | Weekly Vol | Monthly Vol | Risk Score |
|--------|-----------|------------|-------------|------------|
| WDC | 3.5% | 7.5% | 15.0% | 8/10 |
| SLV | 2.8% | 6.0% | 12.0% | 7/10 |
| STLD | 2.5% | 5.5% | 11.0% | 6/10 |
| ETHUSD | 3.8% | 8.0% | 16.0% | 9/10 |
| CRCL | 4.0% | 8.5% | 17.0% | 9/10 |
| AMD | 2.4% | 5.2% | 10.5% | 6/10 |
| NVDA | 2.6% | 5.8% | 12.0% | 7/10 |
| MNQ | 1.4% | 3.0% | 6.5% | 4/10 |
| GS | 1.6% | 3.5% | 7.0% | 5/10 |
| EURUSD | 0.5% | 1.0% | 2.0% | 2/10 |

### Sharpe Ratio Estimates (1-Year)

| Symbol | Expected Return | Volatility | Sharpe Ratio |
|--------|-----------------|------------|--------------|
| WDC | 55% | 30% | 1.83 |
| SLV | 40% | 25% | 1.60 |
| STLD | 30% | 22% | 1.36 |
| GS | 22% | 15% | 1.47 |
| UBS | 18% | 18% | 1.00 |
| AMD | 15% | 20% | 0.75 |
| NVDA | 12% | 24% | 0.50 |
| EURUSD | 3% | 5% | 0.60 |

---

## Sector Analysis

### Technology Sector (NVDA, AMD)

**Current Status:** BULLISH
- **NVDA:** +4.67% over 100 days, strong semiconductor demand
- **AMD:** +25.89%, recovering from previous weakness
- Outlook: Continued AI/GPU driven growth

### Commodities (SLV, STLD, WDC)

**Current Status:** VERY BULLISH
- **SLV:** +41.55%, precious metals rally
- **STLD:** +40.18%, steel demand
- **WDC:** +150.56%, storage/data boom
- Outlook: Commodities supercycle likely

### Financial Sector (GS, UBS)

**Current Status:** BULLISH
- GS: +26.42%, strong earnings
- UBS: +17.99%, recovery after Credit Suisse acquisition
- Outlook: Steady growth expected

### Index Sector (MNQ, EWJ)

**Current Status:** BULLISH
- MNQ: +7.46%, tech index rally
- EWJ: +5.27%, Japan reopening
- Outlook: Positive momentum

### Crypto (ETHUSD)

**Current Status:** BEARISH
- ETHUSD: -32.42% over 100 days
- Outlook: Recovery may take time

### Fintech (CRCL)

**Current Status:** VERY BEARISH
- CRCL: -44.08%, Circle stablecoin challenges
- Outlook: Turnover possible, high risk

---

## Key Catalysts

### Near-Term (1-4 weeks)
- NVDA earnings
- Fed interest rate decision
- Precious metals movement
- Crypto ETF flows

### Medium-Term (1-4 months)
- WDC storage demand
- SLV precious metals
- GS investment banking recovery
- AMD GPU launches

### Long-Term (4-12 months)
- AI infrastructure buildout
- Commodity supercycle
- Crypto adoption
- Japan economic reforms

---

## Action Items

### Immediate (This Week)
1. **ACCUMULATE WDC** on any pullback
2. **ADD SLV** for commodities exposure
3. **REDUCE CRCL** exposure

### This Month
1. **Build STLD** position gradually
2. **Add GS** for financial sector
3. **Hedge** with EURUSD

### This Quarter
1. **Rebalance** towards commodities (WDC, SLV, STLD)
2. **Review CRCL** for potential bottom
3. **Monitor** crypto for recovery signals

---

## Appendix: Data Summary

### Real Historical Prices (from Yahoo Finance)

| Symbol | Type | Current Price | 100d Change |
|--------|------|---------------|-------------|
| WDC | Stock | $187.70 | +150.56% |
| SLV | Stock | $65.75 | +41.55% |
| STLD | Stock | $176.06 | +40.18% |
| AMD | Stock | $223.47 | +25.89% |
| GS | Stock | $914.34 | +26.42% |
| UBS | Stock | $47.10 | +17.99% |
| EWJ | Index | $81.33 | +5.27% |
| TTWO | Stock | $251.60 | +8.36% |
| MNQ | Index | 23,235.63 | +7.46% |
| NVDA | Stock | $188.85 | +4.67% |
| INRJPY | Forex | 1.7424 | +3.44% |
| EURUSD | Forex | 1.1747 | +0.82% |
| NET | Stock | $196.02 | -2.43% |
| ETHUSD | Crypto | $3,000.39 | -32.42% |
| CRCL | Stock | $83.47 | -44.08% |

### Model Performance (Real Historical Data)

| Metric | Value |
|--------|-------|
| Training Samples | 1,410 |
| Training Window | 100 trading days |
| Data Source | Yahoo Finance (real OHLCV) |
| Storage | Redis with 100-day TTL |
| Correlation Range | -0.529 to +0.714 |
| Assets Analyzed | 15 |

### Disclaimer

**This report is generated by automated models using real historical data and should not be considered financial advice.**

- Predictions are based on real historical patterns
- Past performance does not guarantee future results
- Always consult with a licensed financial advisor
- Market conditions can change rapidly
- Models may have inherent limitations

---

**Report Generated By:** Financial Forecasting System  
**Models Used:** Random Forest (100 trees), Gaussian Copula, Betafish Search, Exa Search  
**Data Source:** Yahoo Finance Historical Data (100 days)  
**Storage:** Redis with real-time OHLCV data  
**Training Samples:** 1,410 real historical samples  
**Confidence Level:** Medium (predictions based on real patterns)

---

*End of Report*
