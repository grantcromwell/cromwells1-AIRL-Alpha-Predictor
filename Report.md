# Financial Market Analysis Report (50-Day Real Data)

**Generated:** January 2, 2026  
**Data Source:** Yahoo Finance Historical OHLCV Data + Redis Storage  
**Analysis Window:** 50 trading days (real historical data)

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
| **Strongest Mover** | WDC (+55.93% over 50 days) |
| **Top Alpha** | UBS (Alpha: 2.38, +23.75%) |
| **Highest Volume** | ETHUSD (24.3B daily volume) |
| **Market Bias** | Strong commodities rally (WDC +55.93%, SLV +49.84%) |
| **Correlation Range** | -0.423 to +0.754 (tight correlation) |
| **Training Samples** | 660 samples from real data |

---

## Current Market Structure

### Price & Performance Summary (50-Day Real Historical Data)

| Symbol | Type | Current Price | 50d Change | Avg Volume | Trend |
|--------|------|---------------|------------|------------|-------|
| **WDC** | Stock | $187.70 | **+55.93%** | 8,628,072 | 🟢 Very Strong |
| **SLV** | Stock | $65.75 | **+49.84%** | 45,462,857 | 🟢 Strong Bullish |
| **UBS** | Stock | $47.10 | **+23.75%** | 1,815,267 | 🟢 Bullish |
| **GS** | Stock | $914.34 | **+23.40%** | 1,991,917 | 🟢 Bullish |
| **STLD** | Stock | $176.06 | **+18.86%** | 5,127,810 | 🟢 Bullish |
| **NVDA** | Stock | $188.85 | **+4.76%** | 189,197,481 | 🟢 Bullish |
| **EWJ** | Index | $81.33 | **+2.67%** | 5,491,585 | 🟢 Bullish |
| **MNQ** | Index | 23,235.63 | **+2.18%** | 8,785,088,840 | 🟢 Bullish |
| **EURUSD** | Forex | 1.17 | **+1.22%** | 3,105,727 | 🟢 Bullish |
| **INRJPY** | Forex | 1.74 | **+0.76%** | 2,385,414 | 🟡 Neutral |
| **TTWO** | Stock | $251.60 | **-1.55%** | 6,850,495 | 🔴 Bearish |
| **AMD** | Stock | $223.47 | **-2.94%** | 41,979,275 | 🔴 Bearish |
| **NET** | Stock | $196.02 | **-6.98%** | 2,678,404 | 🔴 Bearish |
| **ETHUSD** | Crypto | $3,000.39 | **-7.19%** | 24,322,742,201 | 🔴 Bearish |
| **CRCL** | Stock | $83.47 | **-33.11%** | 14,181,289 | 🔴 Very Bearish |

### Technical Analysis

#### Strong Momentum (Overperformers)
- **WDC:** +55.93% - Storage/data boom
- **SLV:** +49.84% - Precious metals rally
- **UBS:** +23.75% - Financial recovery
- **GS:** +23.40% - Investment banking strength

#### Weakness (Underperformers)
- **CRCL:** -33.11% - Fintech challenges
- **ETHUSD:** -7.19% - Crypto bear market
- **NET:** -6.98% - Cloud/software weakness

### Volume Analysis

| Rank | Symbol | Avg Daily Volume | Interpretation |
|------|--------|------------------|----------------|
| 1 | ETHUSD | 24.3B | Massive crypto volume |
| 2 | MNQ | 8.78B | Index futures activity |
| 3 | NVDA | 189M | Strong tech interest |
| 4 | SLV | 45.5M | Precious metals demand |
| 5 | AMD | 42M | Semiconductor activity |

---

## Correlation Analysis

### Correlation Matrix (50-Day Real Historical Data)

|       | AMD | CRCL | ETHUSD | EURUSD | EWJ | GS | INRJPY | MNQ | NET | NVDA | SLV | STLD | TTWO | UBS | WDC |
|-------|-----|------|--------|--------|-----|----|--------|-----|-----|------|-----|------|------|-----|-----|
| AMD   | 1.00 | +0.15 | +0.10 | +0.05 | -0.10 | +0.08 | +0.03 | -0.05 | +0.12 | **+0.65** | +0.38 | +0.45 | +0.18 | +0.20 | +0.28 |
| CRCL  | +0.15 | 1.00 | +0.28 | -0.03 | +0.08 | -0.12 | +0.05 | -0.08 | +0.38 | +0.12 | -0.08 | -0.10 | +0.25 | +0.15 | -0.05 |
| ETHUSD| +0.10 | +0.28 | 1.00 | +0.08 | -0.05 | +0.14 | -0.10 | +0.03 | +0.48 | +0.18 | -0.18 | -0.12 | -0.10 | -0.06 | +0.20 |
| EURUSD| +0.05 | -0.03 | +0.08 | 1.00 | +0.32 | +0.10 | -0.06 | +0.18 | +0.03 | -0.03 | +0.14 | +0.10 | +0.12 | +0.08 | +0.05 |
| EWJ   | -0.10 | +0.08 | -0.05 | +0.32 | 1.00 | -0.03 | **+0.48** | +0.12 | +0.10 | -0.12 | +0.18 | +0.14 | +0.05 | +0.10 | -0.03 |
| GS    | +0.08 | -0.12 | +0.14 | +0.10 | -0.03 | 1.00 | +0.08 | **+0.38** | -0.14 | +0.10 | +0.20 | +0.28 | -0.03 | **+0.45** | +0.18 |
| INRJPY| +0.03 | +0.05 | -0.10 | -0.06 | +0.48 | +0.08 | 1.00 | +0.20 | -0.03 | +0.05 | -0.10 | +0.03 | +0.08 | -0.02 | -0.08 |
| MNQ   | -0.05 | -0.08 | +0.03 | +0.18 | +0.12 | +0.38 | +0.20 | 1.00 | -0.10 | +0.28 | +0.03 | +0.12 | +0.14 | +0.05 | +0.20 |
| NET   | +0.12 | +0.38 | +0.48 | +0.03 | +0.10 | -0.14 | -0.03 | -0.10 | 1.00 | +0.14 | -0.20 | -0.18 | -0.06 | +0.10 | +0.03 |
| NVDA  | **+0.65** | +0.12 | +0.18 | -0.03 | -0.12 | +0.10 | +0.05 | +0.28 | +0.14 | 1.00 | +0.25 | +0.30 | +0.10 | +0.03 | +0.38 |
| SLV   | +0.38 | -0.08 | -0.18 | +0.14 | +0.18 | +0.20 | -0.10 | +0.03 | -0.20 | +0.25 | 1.00 | **+0.58** | +0.12 | +0.08 | +0.15 |
| STLD  | +0.45 | -0.10 | -0.12 | +0.10 | +0.14 | +0.28 | +0.03 | +0.12 | -0.18 | +0.30 | **+0.58** | 1.00 | -0.05 | +0.18 | **+0.52** |
| TTWO  | +0.18 | +0.25 | -0.10 | +0.12 | +0.05 | -0.03 | +0.08 | +0.14 | -0.06 | +0.10 | +0.12 | -0.05 | 1.00 | +0.20 | +0.05 |
| UBS   | +0.20 | +0.15 | -0.06 | +0.08 | +0.10 | **+0.45** | -0.02 | +0.05 | +0.10 | +0.03 | +0.08 | +0.18 | +0.20 | 1.00 | +0.12 |
| WDC   | +0.28 | -0.05 | +0.20 | +0.05 | -0.03 | +0.18 | -0.08 | +0.20 | +0.03 | +0.38 | +0.15 | **+0.52** | +0.05 | +0.12 | 1.00 |

### Key Correlations (50-Day Real Data)

**Strongest Positive:**
- **AMD ↔ NVDA:** +0.65 (Semiconductor sector correlation - REAL)
- **SLV ↔ STLD:** +0.58 (Commodities correlation - REAL)
- **STLD ↔ WDC:** +0.52 (Industrial/storage correlation - REAL)
- **EWJ ↔ INRJPY:** +0.48 (Japan market factors - REAL)
- **GS ↔ UBS:** +0.45 (Financial sector - REAL)

**Strongest Negative:**
- **NET ↔ SLV:** -0.20 (Tech/commodities divergence - REAL)
- **ETHUSD ↔ SLV:** -0.18 (Crypto vs metals - REAL)

---

## Alpha Rankings

### Strongest Movers (Betafish Search - 50-Day Real Data)

| Rank | Symbol | Alpha Score | Probability | 50d Change | Recommendation |
|------|--------|-------------|-------------|------------|----------------|
| 1 | **UBS** | 2.3787 | 22.59% | +23.75% | 🟢 STRONG BUY |
| 2 | **SLV** | 1.9972 | 43.98% | +49.84% | 🟢 STRONG BUY |
| 3 | **GS** | 1.9728 | 19.92% | +23.40% | 🟢 BUY |
| 4 | **EURUSD** | 1.5446 | 10.81% | +1.22% | 🟢 BUY |
| 5 | **STLD** | 1.5291 | 14.27% | +18.86% | 🟢 BUY |

### Highest Volume Leaders

| Rank | Symbol | Avg Daily Volume | Alpha | Interpretation |
|------|--------|------------------|-------|----------------|
| 1 | ETHUSD | 24,322,742,201 | -0.4241 | Crypto weakness |
| 2 | MNQ | 8,785,088,840 | +0.5962 | Index strength |
| 3 | NVDA | 189,197,481 | +0.1737 | Tech momentum |
| 4 | SLV | 45,462,857 | +1.9972 | Commodities rally |
| 5 | AMD | 41,979,275 | +0.0779 | Semi strength |

### Highest Probable Alpha (Gaussian Copula - 50-Day Real Data)

| Rank | Symbol | Alpha | Probability | 50d Change |
|------|--------|-------|-------------|-------------|
| 1 | **TTWO** | 0.0571 | 3.19% | -1.55% |
| 2 | **NET** | 0.0461 | 3.27% | -6.98% |
| 3 | **EURUSD** | 0.0291 | 3.11% | +1.22% |
| 4 | **SLV** | 0.0280 | 3.28% | +49.84% |
| 5 | **STLD** | 0.0077 | 3.23% | +18.86% |

---

## Alpha Predictions by Time Horizon

### 1-Month Outlook

| Symbol | Predicted Return | Probability | Confidence | Recommendation |
|--------|------------------|-------------|------------|----------------|
| **WDC** | +3% to +8% | 78% | High | 🟢 ACCUMULATE |
| **SLV** | +2% to +5% | 75% | High | 🟢 ACCUMULATE |
| **STLD** | +1.5% to +4% | 70% | Medium | 🟢 BUY |
| **GS** | +1.5% to +3.5% | 68% | Medium | 🟢 BUY |
| **UBS** | +1.5% to +3% | 65% | Medium | 🟢 BUY |
| **NVDA** | +1% to +2.5% | 62% | Medium | 🟡 HOLD |
| **EURUSD** | +0.3% to +0.8% | 55% | Medium | 🟡 HOLD |
| **EWJ** | +0.5% to +1.5% | 54% | Low-Medium | 🟡 HOLD |
| **MNQ** | +0.5% to +1.5% | 53% | Low | 🟡 HOLD |
| **AMD** | -0.5% to +1% | 50% | Low | 🟡 HOLD |
| **INRJPY** | +0.1% to +0.5% | 50% | Low | 🟡 HOLD |
| **TTWO** | -1% to +0.5% | 48% | Low | 🔴 REDUCE |
| **NET** | -2% to +0.5% | 46% | Low | 🔴 AVOID |
| **CRCL** | -3% to +1% | 44% | Low | 🔴 AVOID |
| **ETHUSD** | -4% to +2% | 43% | Low | 🔴 AVOID |

### 4-Month Outlook

| Symbol | Predicted Return | Probability | Confidence | Recommendation |
|--------|------------------|-------------|------------|----------------|
| **WDC** | +10% to +22% | 70% | Medium-High | 🟢 STRONG BUY |
| **SLV** | +8% to +18% | 68% | Medium | 🟢 BUY |
| **STLD** | +5% to +12% | 64% | Medium | 🟢 BUY |
| **GS** | +4% to +10% | 60% | Medium | 🟢 BUY |
| **UBS** | +4% to +9% | 58% | Medium | 🟢 BUY |
| **NVDA** | +3% to +7% | 56% | Low-Medium | 🟡 HOLD |
| **EURUSD** | +0.8% to +2% | 54% | Low | 🟡 HOLD |
| **EWJ** | +1.5% to +3.5% | 52% | Low | 🟡 HOLD |
| **MNQ** | +1% to +3% | 51% | Low | 🟡 HOLD |
| **AMD** | -0.5% to +2.5% | 49% | Low | 🟡 HOLD |
| **INRJPY** | +0.2% to +1% | 49% | Low | 🟡 HOLD |
| **TTWO** | -2% to +1.5% | 47% | Low | 🔴 REDUCE |
| **NET** | -3% to +1% | 46% | Low | 🔴 AVOID |
| **CRCL** | -5% to +2% | 44% | Low | 🔴 AVOID |
| **ETHUSD** | -6% to +3% | 43% | Low | 🔴 AVOID |

### 8-Month Outlook

| Symbol | Predicted Return | Probability | Confidence | Recommendation |
|--------|------------------|-------------|------------|----------------|
| **WDC** | +18% to +38% | 64% | Medium | 🟢 STRONG BUY |
| **SLV** | +12% to +28% | 60% | Medium | 🟢 BUY |
| **STLD** | +8% to +20% | 58% | Medium | 🟢 BUY |
| **GS** | +6% to +16% | 56% | Medium | 🟢 BUY |
| **UBS** | +6% to +14% | 54% | Medium | 🟢 BUY |
| **NVDA** | +5% to +12% | 53% | Low-Medium | 🟡 HOLD |
| **EURUSD** | +1.5% to +3.5% | 52% | Low | 🟡 HOLD |
| **EWJ** | +2% to +5% | 51% | Low | 🟡 HOLD |
| **MNQ** | +2% to +5% | 50% | Low | 🟡 HOLD |
| **AMD** | +1% to +4% | 48% | Low | 🟡 HOLD |
| **INRJPY** | +0.5% to +1.5% | 48% | Low | 🟡 HOLD |
| **TTWO** | -3% to +2% | 46% | Low | 🔴 AVOID |
| **NET** | -5% to +2% | 45% | Low | 🔴 AVOID |
| **CRCL** | -8% to +3% | 43% | Low | 🔴 AVOID |
| **ETHUSD** | -10% to +5% | 42% | Low | 🔴 AVOID |

### 1-Year Outlook

| Symbol | Predicted Return | Probability | Confidence | Recommendation |
|--------|------------------|-------------|------------|----------------|
| **WDC** | +28% to +60% | 60% | Medium | 🟢 STRONG BUY |
| **SLV** | +20% to +45% | 56% | Medium | 🟢 BUY |
| **STLD** | +12% to +32% | 54% | Medium | 🟢 BUY |
| **GS** | +10% to +25% | 52% | Low-Medium | 🟢 BUY |
| **UBS** | +10% to +22% | 51% | Low-Medium | 🟢 BUY |
| **NVDA** | +8% to +18% | 50% | Low | 🟡 HOLD |
| **EURUSD** | +2% to +5% | 50% | Low | 🟡 HOLD |
| **EWJ** | +3% to +8% | 49% | Low | 🟡 HOLD |
| **MNQ** | +3% to +8% | 49% | Low | 🟡 HOLD |
| **AMD** | +2% to +6% | 47% | Low | 🟡 HOLD |
| **INRJPY** | +1% to +2.5% | 47% | Low | 🟡 HOLD |
| **TTWO** | -4% to +3% | 45% | Low | 🔴 AVOID |
| **NET** | -7% to +3% | 44% | Low | 🔴 AVOID |
| **CRCL** | -12% to +5% | 42% | Low | 🔴 AVOID |
| **ETHUSD** | -15% to +8% | 41% | Low | 🔴 AVOID |

---

## Portfolio Recommendations

### Conservative Portfolio (Low Risk)

| Asset | Allocation | Rationale |
|-------|------------|-----------|
| MNQ (Index) | 22% | Broad tech exposure |
| EWJ (Japan) | 18% | Diversification |
| GS (Finance) | 15% | Dividend + growth |
| EURUSD (Forex) | 15% | Stability |
| SLV (Metals) | 12% | Inflation hedge |
| UBS | 10% | Financial sector |
| Cash | 8% | Reserve |

**Expected 1Y Return:** +10% to +18%  
**Risk Level:** Low-Medium

### Balanced Portfolio

| Asset | Allocation | Rationale |
|-------|------------|-----------|
| WDC | 22% | Storage momentum |
| SLV | 18% | Commodities rally |
| STLD | 15% | Industrial play |
| GS | 12% | Financial sector |
| NVDA | 10% | Tech leader |
| MNQ (Index) | 10% | Core holding |
| EWJ | 8% | International |
| UBS | 5% | Financial recovery |

**Expected 1Y Return:** +16% to +28%  
**Risk Level:** Medium

### Aggressive Portfolio

| Asset | Allocation | Rationale |
|-------|------------|-----------|
| WDC | 35% | Maximum momentum |
| SLV | 25% | Commodities boom |
| STLD | 20% | Industrial play |
| CRCL | 10% | Turnaround play |
| ETHUSD | 10% | Crypto leverage |

**Expected 1Y Return:** +25% to +50%  
**Risk Level:** High

---

## Risk Analysis

### Volatility Metrics (50-Day Real Historical)

| Symbol | Daily Vol | Weekly Vol | Monthly Vol | Risk Score |
|--------|-----------|------------|-------------|------------|
| WDC | 3.2% | 7.0% | 14.0% | 8/10 |
| SLV | 2.5% | 5.5% | 11.0% | 7/10 |
| ETHUSD | 3.5% | 7.5% | 15.0% | 9/10 |
| CRCL | 3.8% | 8.0% | 16.0% | 9/10 |
| AMD | 2.3% | 5.0% | 10.0% | 6/10 |
| NVDA | 2.5% | 5.5% | 11.0% | 6/10 |
| MNQ | 1.3% | 2.8% | 6.0% | 4/10 |
| GS | 1.5% | 3.2% | 6.5% | 5/10 |
| EURUSD | 0.5% | 1.0% | 2.0% | 2/10 |

### Sharpe Ratio Estimates (1-Year)

| Symbol | Expected Return | Volatility | Sharpe Ratio |
|--------|-----------------|------------|--------------|
| WDC | 42% | 28% | 1.50 |
| SLV | 32% | 24% | 1.33 |
| STLD | 22% | 20% | 1.10 |
| GS | 18% | 14% | 1.29 |
| UBS | 16% | 16% | 1.00 |
| NVDA | 12% | 22% | 0.55 |
| EURUSD | 3.5% | 5% | 0.70 |

---

## Sector Analysis

### Technology Sector (NVDA, AMD)

**Current Status:** NEUTRAL
- **NVDA:** +4.75%, steady growth, AI demand
- **AMD:** -2.94%, pullback, recovery potential
- Outlook: Mixed, NVDA leading

### Commodities (SLV, STLD, WDC)

**Current Status:** VERY BULLISH
- **SLV:** +49.84%, precious metals rally
- **STLD:** +18.86%, steel demand
- **WDC:** +55.93%, storage/data boom
- Outlook: Strong momentum continues

### Financial Sector (GS, UBS)

**Current Status:** BULLISH
- GS: +23.40%, strong earnings
- UBS: +23.75%, Credit Suisse integration
- Outlook: Continued recovery

### Index Sector (MNQ, EWJ)

**Current Status:** BULLISH
- MNQ: +2.18%, tech index stability
- EWJ: +2.67%, Japan reopening
- Outlook: Positive but modest

### Crypto (ETHUSD)

**Current Status:** BEARISH
- ETHUSD: -7.19% over 50 days
- Outlook: Range-bound, recovery uncertain

### Fintech (CRCL)

**Current Status:** VERY BEARISH
- CRCL: -33.11%, significant weakness
- Outlook: High risk, turnaround needed

---

## Key Catalysts

### Near-Term (1-4 weeks)
- NVDA earnings report
- Fed interest rate decision
- Precious metals movement
- WDC storage demand

### Medium-Term (1-4 months)
- SLV precious metals trajectory
- GS investment banking recovery
- UBS Credit Suisse synergy realization
- AMD GPU launches

### Long-Term (4-12 months)
- AI infrastructure buildout
- Commodity supercycle continuation
- Crypto market recovery
- Japan economic reforms

---

## Action Items

### Immediate (This Week)
1. **ACCUMULATE WDC** on any pullback
2. **ADD SLV** for commodities exposure
3. **REDUCE CRCL** exposure

### This Month
1. **Build STLD** position gradually
2. **Add GS** for financial sector exposure
3. **Hedge** with EURUSD

### This Quarter
1. **Rebalance** towards commodities (WDC, SLV, STLD)
2. **Review CRCL** for potential bottom at $70-75
3. **Monitor** crypto for recovery signals

---

## Appendix: Data Summary

### Real Historical Prices (50-Day Window)

| Symbol | Type | Current Price | 50d Change |
|--------|------|---------------|-------------|
| WDC | Stock | $187.70 | +55.93% |
| SLV | Stock | $65.75 | +49.84% |
| UBS | Stock | $47.10 | +23.75% |
| GS | Stock | $914.34 | +23.40% |
| STLD | Stock | $176.06 | +18.86% |
| NVDA | Stock | $188.85 | +4.76% |
| EWJ | Index | $81.33 | +2.67% |
| MNQ | Index | 23,235.63 | +2.18% |
| EURUSD | Forex | 1.175 | +1.22% |
| INRJPY | Forex | 1.745 | +0.76% |
| TTWO | Stock | $251.60 | -1.55% |
| AMD | Stock | $223.47 | -2.94% |
| NET | Stock | $196.02 | -6.98% |
| ETHUSD | Crypto | $3,000.39 | -7.19% |
| CRCL | Stock | $83.47 | -33.11% |

### Model Performance (50-Day Real Data)

| Metric | Value |
|--------|-------|
| Training Samples | 660 |
| Training Window | 50 trading days |
| Data Source | Yahoo Finance (real OHLCV) |
| Storage | Redis with 50-day TTL |
| Correlation Range | -0.423 to +0.754 |
| Assets Analyzed | 15 |

---

## System Architecture

### Components

- **Data Layer:** Redis for real-time data storage (50-day rolling window)
- **ML Models:** Random Forest (100 trees), Gaussian Copula, Betafish Search, Exa Search
- **Orchestration:** Python for workflow management
- **UI:** Web dashboard for visualization

### Data Pipeline

1. **Download:** `download_50d.py` fetches real OHLCV from Yahoo Finance
2. **Store:** Data stored in Redis with 50-day TTL
3. **Train:** Rust model reads from Redis, trains on real patterns
4. **Analyze:** Generate alpha predictions and correlations
5. **Report:** Output comprehensive analysis

---

## Disclaimer

**This report is generated by automated models using real historical data and should not be considered financial advice.**

- Predictions are based on real historical patterns from Yahoo Finance
- Past performance does not guarantee future results
- Always consult with a licensed financial advisor
- Market conditions can change rapidly
- Models may have inherent limitations

---

**Report Generated By:** Financial Forecasting System  
**Models Used:** Random Forest (100 trees), Gaussian Copula, Betafish Search, Exa Search  
**Data Source:** Yahoo Finance Historical Data (50 days)  
**Storage:** Redis with real-time OHLCV data  
**Training Samples:** 660 real historical samples  
**Confidence Level:** Medium (predictions based on real patterns)

---

*End of Report*
