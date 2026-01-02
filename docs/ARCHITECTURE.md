# Financial Forecasting System Architecture

## System Overview

This system provides real-time equity analysis and forecasting using machine learning models (Random Forest and Gaussian Copula) with automated alpha discovery through search algorithms.

### Target Assets
- **Equities**: NQ, NVDA, AMD, WDC, SLV, GS, NET, EWJ
- **Forex**: EURUSD, INRJPY
- **Indices**: .BRL, .GBP

---

## System Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Data Ingestion Layer                        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐    │
│  │   Market    │  │    Alpha    │  │   Historical            │    │
│  │  Data APIs  │  │  Discovery  │  │   Data Backfill         │    │
│  └──────┬──────┘  └──────┬──────┘  └──────────┬──────────────┘    │
└─────────┼────────────────┼────────────────────┼───────────────────┘
          │                │                    │
          ▼                ▼                    ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    Data Pipeline (data-pipeline/)                   │
│  ┌──────────────┐  ┌──────────────┐  ┌────────────────────────┐   │
│  │  Data        │  │  Technical   │  │  Data Quality          │   │
│  │  Validation  │  │  Indicators  │  │  & Enrichment          │   │
│  └──────┬───────┘  └──────┬───────┘  └──────────┬─────────────┘   │
└─────────┼────────────────┼────────────────────┼───────────────────┘
          │                │                    │
          └────────────────┼────────────────────┘
                           ▼
┌─────────────────────────────────────────────────────────────────────┐
│                       Redis Data Store                               │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  - 2-week TTL automatic expiration                          │   │
│  │  - RDB persistence for crash recovery                        │   │
│  │  - Partitioned by asset class & timeframe                   │   │
│  │  - Indexed by timestamp, asset_id, and signal_type          │   │
│  └─────────────────────────────────────────────────────────────┘   │
└────────────────────────┬────────────────────────────────────────────┘
                         │
         ┌───────────────┼───────────────┐
         │               │               │
         ▼               ▼               ▼
┌────────────────┐ ┌──────────────┐ ┌────────────────┐
│  Java Backend  │ │ Rust Models  │ │  Search Algos  │
│   (API Layer)  │ │    (ML Core) │ │  (Alpha Disc.) │
└────────┬───────┘ └──────┬───────┘ └────────┬───────┘
         │                │                   │
         └────────────────┼───────────────────┘
                          ▼
              ┌───────────────────────┐
              │   Web UI (ui/)        │
              │  - Strongest Movers   │
              │  - Highest Volume     │
              │  - Alpha Signals      │
              └───────────────────────┘
```

---

## Component Specifications

### 1. Data Pipeline (`data-pipeline/`)

**Responsibilities:**
- Fetch market data from multiple sources
- Validate and clean incoming data
- Calculate technical indicators
- Transform data for storage
- Handle data backfilling and real-time updates

**Key Modules:**
```
data-pipeline/
├── src/
│   ├── ingestors/          # Data source adapters
│   │   ├── market_data.py
│   │   └── alpha_feeds.py
│   ├── processors/          # Data transformation
│   │   ├── technical_indicators.py
│   │   └── normalizer.py
│   ├── validators/          # Data quality checks
│   │   └── schema_validator.py
│   └── storage/             # Redis integration
│       └── redis_writer.py
├── config/
│   └── pipeline_config.yaml
└── tests/
```

**Technical Indicators Calculated:**
- SMA (5, 10, 20, 50, 200 periods)
- EMA (5, 10, 20, 50, 200 periods)
- RSI (14 periods)
- MACD (12, 26, 9)
- Bollinger Bands (20, 2 std)
- Volume indicators (OBV, VWAP)
- Volatility (ATR, Historical Volatility)
- Momentum indicators

---

### 2. Redis Data Store

**Key-Value Structure:**

```
# Price Data
price:{asset_id}:{timestamp} -> {
  "asset_id": "NVDA",
  "timestamp": 1704067200000,
  "open": 485.50,
  "high": 492.30,
  "low": 483.20,
  "close": 490.15,
  "volume": 45678900
}
TTL: 1209600 seconds (14 days)

# Technical Indicators
indicators:{asset_id}:{timestamp} -> {
  "asset_id": "NVDA",
  "timestamp": 1704067200000,
  "sma_5": 488.20,
  "sma_20": 482.50,
  "ema_12": 487.30,
  "rsi_14": 65.4,
  "macd": 3.2,
  "macd_signal": 2.8,
  "bb_upper": 495.40,
  "bb_middle": 485.20,
  "bb_lower": 475.00,
  "atr_14": 8.5
}
TTL: 1209600 seconds

# ML Predictions
prediction:{asset_id}:{timestamp} -> {
  "asset_id": "NVDA",
  "timestamp": 1704067200000,
  "model_type": "random_forest",
  "horizon": "1h",
  "prediction": 492.50,
  "confidence": 0.87,
  "features_importance": {
    "rsi_14": 0.25,
    "volume": 0.18,
    "macd": 0.15
  }
}
TTL: 1209600 seconds

# Alpha Signals
alpha:{asset_id}:{signal_id}:{timestamp} -> {
  "asset_id": "NVDA",
  "signal_id": "uuid",
  "timestamp": 1704067200000,
  "source": "betafish|exa",
  "signal_type": "momentum|mean_reversion|arbitrage",
  "expected_return": 0.025,
  "confidence": 0.78,
  "holding_period": "3d",
  "risk_score": 0.45
}
TTL: 1209600

# Aggregates (for UI queries)
aggregates:movers:{date} -> [
  {"asset_id": "NVDA", "change_pct": 5.2, "volume": 45678900},
  {"asset_id": "AMD", "change_pct": 4.1, "volume": 34567800}
]
TTL: 1209600

aggregates:volume:{date} -> [
  {"asset_id": "NQ", "volume": 234567000},
  {"asset_id": "NVDA", "volume": 45678900}
]
TTL: 1209600

aggregates:alpha:{date} -> [
  {"asset_id": "EURUSD", "signal": "momentum", "score": 0.92},
  {"asset_id": "AMD", "signal": "mean_reversion", "score": 0.88}
]
TTL: 1209600

# Indexes
index:assets:{asset_id} -> "SET of all timestamps"
index:timestamps -> "SORTED SET of all timestamps"
TTL: 1209600
```

**Redis Configuration:**
```yaml
maxmemory: 4gb
maxmemory-policy: allkeys-lru
save: 900 1
save: 300 10
save: 60 10000
appendonly: yes
appendfsync: everysec
```

---

### 3. Java Backend (`java-backend/`)

**Responsibilities:**
- RESTful API endpoints
- Query Redis for data retrieval
- Aggregate and format responses
- Handle authentication and rate limiting
- Serve data to UI

**API Specification:**

```java
// Base URL: http://localhost:8080/api/v1

// 1. Get historical price data
GET /prices?asset_id={asset_id}&start_time={timestamp}&end_time={timestamp}&interval={1m,5m,1h,1d}
Response: {
  "asset_id": "NVDA",
  "data": [
    {"timestamp": 1704067200000, "open": 485.50, "high": 492.30, "low": 483.20, "close": 490.15, "volume": 45678900}
  ]
}

// 2. Get technical indicators
GET /indicators?asset_id={asset_id}&timestamp={timestamp}
Response: {
  "asset_id": "NVDA",
  "timestamp": 1704067200000,
  "indicators": {
    "sma_5": 488.20,
    "rsi_14": 65.4,
    "macd": 3.2
  }
}

// 3. Get ML predictions
GET /predictions?asset_id={asset_id}&horizon={1h,4h,1d,1w}
Response: {
  "asset_id": "NVDA",
  "predictions": [
    {"timestamp": 1704067200000, "model": "random_forest", "value": 492.50, "confidence": 0.87},
    {"timestamp": 1704067200000, "model": "gaussian_copula", "value": 491.80, "confidence": 0.82}
  ]
}

// 4. Get strongest movers
GET /aggregates/movers?date={date}&limit={10}
Response: {
  "date": "2024-01-01",
  "movers": [
    {"rank": 1, "asset_id": "NVDA", "change_pct": 5.2, "volume": 45678900, "price": 490.15},
    {"rank": 2, "asset_id": "AMD", "change_pct": 4.1, "volume": 34567800, "price": 145.30}
  ]
}

// 5. Get highest volume
GET /aggregates/volume?date={date}&limit={10}
Response: {
  "date": "2024-01-01",
  "volume_leaders": [
    {"rank": 1, "asset_id": "NQ", "volume": 234567000, "value": 45678000000},
    {"rank": 2, "asset_id": "NVDA", "volume": 45678900, "value": 22398000000}
  ]
}

// 6. Get alpha signals
GET /aggregates/alpha?date={date}&min_confidence={0.7}&limit={20}
Response: {
  "date": "2024-01-01",
  "signals": [
    {"rank": 1, "asset_id": "EURUSD", "signal_type": "momentum", "confidence": 0.92, "expected_return": 0.015},
    {"rank": 2, "asset_id": "AMD", "signal_type": "mean_reversion", "confidence": 0.88, "expected_return": 0.022}
  ]
}

// 7. Get asset overview (combined endpoint)
GET /overview?asset_id={asset_id}
Response: {
  "asset_id": "NVDA",
  "price": {"current": 490.15, "change": 5.2, "timestamp": 1704067200000},
  "indicators": {"rsi": 65.4, "macd": 3.2, "trend": "bullish"},
  "predictions": [{"model": "rf", "value": 492.50, "confidence": 0.87}],
  "alpha_signals": [{"type": "momentum", "confidence": 0.82}]
}
```

**Project Structure:**
```
java-backend/
├── src/main/java/com/financial/forecast/
│   ├── FinancialForecastApplication.java
│   ├── config/
│   │   ├── RedisConfig.java
│   │   └── WebConfig.java
│   ├── controller/
│   │   ├── PriceController.java
│   │   ├── IndicatorController.java
│   │   ├── PredictionController.java
│   │   └── AggregateController.java
│   ├── service/
│   │   ├── RedisService.java
│   │   ├── AggregationService.java
│   │   └── DataValidationService.java
│   ├── model/
│   │   ├── PriceData.java
│   │   ├── TechnicalIndicators.java
│   │   ├── Prediction.java
│   │   └── AlphaSignal.java
│   └── repository/
│       └── RedisRepository.java
├── src/main/resources/
│   ├── application.yml
│   └── logback.xml
├── Dockerfile
└── pom.xml
```

---

### 4. Rust Models (`rust-models/`)

**Responsibilities:**
- Random Forest implementation for price prediction
- Gaussian Copula for dependency modeling and portfolio optimization
- Feature engineering and selection
- Model training and inference
- Model serialization for production deployment

**Project Structure:**
```
rust-models/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── random_forest.rs
│   │   ├── gaussian_copula.rs
│   │   └── ensemble.rs
│   ├── features/
│   │   ├── mod.rs
│   │   ├── engineering.rs
│   │   ├── selection.rs
│   │   └── normalization.rs
│   ├── training/
│   │   ├── mod.rs
│   │   ├── dataset.rs
│   │   └── evaluator.rs
│   ├── inference/
│   │   ├── mod.rs
│   │   └── predictor.rs
│   └── utils/
│       ├── mod.rs
│       ├── redis_client.rs
│       └── serialization.rs
├── models/              # Saved model artifacts
├── data/               # Training data cache
├── tests/
└── Dockerfile
```

**Random Forest Model:**
```rust
// Key features
use smartcore::ensemble::random_forest_classifier::*;
use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::model_selection::*;

struct RandomForestModel {
    model: RandomForestClassifier<f64, DenseMatrix<f64>>,
    features: Vec<String>,
    metadata: ModelMetadata,
}

impl RandomForestModel {
    fn train(
        data: &TrainingData,
        n_trees: usize,
        max_depth: usize,
        min_samples_split: usize
    ) -> Result<Self>;

    fn predict(&self, features: &FeatureVector) -> Result<Prediction>;
    fn feature_importance(&self) -> HashMap<String, f64>;
}
```

**Gaussian Copula Model:**
```rust
use nalgebra::{DMatrix, DVector};
use statrs::distribution::{MultivariateNormal, Normal};

struct GaussianCopula {
    correlation_matrix: DMatrix<f64>,
    marginal_distributions: Vec<MarginalDist>,
    assets: Vec<String>,
}

impl GaussianCopula {
    fn fit(returns: &DMatrix<f64>) -> Result<Self>;
    fn simulate(&self, n_simulations: usize, horizon: usize) -> Result<DMatrix<f64>>;
    fn calculate_var(&self, portfolio: &Portfolio, confidence: f64) -> f64;
    fn dependency_structure(&self) -> &DMatrix<f64>;
}
```

**Model Architecture:**
```
Input Features:
├── Price-derived: Returns, Log-returns, Price momentum
├── Technical: RSI, MACD, Bollinger Bands, ATR
├── Volume: OBV, VWAP, Volume momentum
├── Time-based: Hour of day, Day of week, Month
└── Market-wide: VIX, Index correlations, Sector momentum

Random Forest:
├── 100-500 trees
├── Max depth: 10-20 levels
├── Min samples split: 2-5
├── Bootstrap sampling: Yes
├── Feature bagging: sqrt(n_features)
└── Output: Price prediction + probability

Gaussian Copula:
├── Marginal distributions: Empirical or fitted (t, Normal)
├── Correlation: Pearson or Spearman rank
├── Dependency modeling: Full correlation matrix
├── Simulation: Monte Carlo (10,000+ scenarios)
└── Output: Joint distribution, portfolio risk metrics
```

**Dependencies (Cargo.toml):**
```toml
[dependencies]
smartcore = "0.3"
nalgebra = "0.32"
statrs = "0.16"
ndarray = "0.15"
redis = "0.23"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
anyhow = "1.0"
```

---

### 5. Search Algorithms (`search-algorithms/`)

**Responsibilities:**
- Alpha discovery using web search and NLP
- News sentiment analysis
- Market event detection
- Correlation and cointegration discovery
- Pattern recognition in market data

**Project Structure:**
```
search-algorithms/
├── python/
│   ├── betafish/
│   │   ├── __init__.py
│   │   ├── search.py       # Search implementation
│   │   ├── parser.py       # Parse search results
│   │   └── scorer.py       # Alpha scoring
│   └── exa/
│       ├── __init__.py
│       ├── api.py          # Exa API integration
│       ├── retrieval.py    # Neural search
│       └── ranking.py      # Relevance ranking
├── rust/
│   └── correlation/        # Fast correlation calc
│       ├── Cargo.toml
│       └── src/lib.rs
├── config/
│   └── search_config.yaml
└── tests/
```

**Betafish Algorithm:**
```python
from dataclasses import dataclass
from typing import List, Dict
import requests
from bs4 import BeautifulSoup

@dataclass
class AlphaSignal:
    asset_id: str
    signal_type: str  # momentum, mean_reversion, arbitrage
    source: str
    confidence: float
    expected_return: float
    holding_period: str
    reasoning: str

class BetafishSearch:
    def search_alpha_opportunities(
        self,
        query: str,
        asset_universe: List[str]
    ) -> List[AlphaSignal]:
        """
        Search for alpha opportunities using web search
        and financial news sources
        """
        pass

    def analyze_sentiment(self, text: str) -> float:
        """Analyze sentiment of news/article"""
        pass

    def extract_signals(self, content: str) -> List[AlphaSignal]:
        """Extract trading signals from content"""
        pass
```

**Exa Algorithm:**
```python
import exa_py
from typing import List, Dict

class ExaAlphaDiscovery:
    def __init__(self, api_key: str):
        self.exa = exa_py.Exa(api_key=api_key)

    def neural_search_alpha(
        self,
        query: str,
        num_results: int = 10
    ) -> List[Dict]:
        """
        Use Exa's neural search to find relevant
        financial content and analyze for alpha
        """
        results = self.exa.search(
            query=query,
            numResults=num_results,
            useAutoprompt=True,
            category="company"
        )
        return self._analyze_results(results)

    def find_arbitrage_opportunities(
        self,
        assets: List[str]
    ) -> List[Dict]:
        """Search for cross-asset arbitrage"""
        pass

    def detect_regime_changes(
        self,
        market_conditions: Dict
    ) -> List[Dict]:
        """Detect market regime changes"""
        pass
```

**Search Configuration:**
```yaml
search_config:
  betafish:
    sources:
      - bloomberg
      - reuters
      - wsj
      - seeking_alpha
      - yahoo_finance
    update_interval: 300  # seconds
    max_results: 50

  exa:
    api_key_env: EXA_API_KEY
    queries:
      - "stock market momentum strategies"
      - "arbitrage opportunities forex"
      - "mean reversion signals"
      - "undervalued stocks technical analysis"
    max_results: 20

  scoring:
    confidence_threshold: 0.7
    min_expected_return: 0.01
    max_risk_score: 0.8
```

---

### 6. Web UI (`ui/`)

**Responsibilities:**
- Real-time dashboard for market analysis
- Visualization of predictions and signals
- Alert system for trading opportunities
- Historical performance tracking

**Technology Stack:**
- **Framework**: React 18 + TypeScript
- **State Management**: Zustand or Redux Toolkit
- **Data Visualization**: Recharts / TradingView Lightweight Charts
- **UI Library**: shadcn/ui or Tailwind CSS
- **Real-time**: WebSocket connection to Java backend

**Project Structure:**
```
ui/
├── src/
│   ├── components/
│   │   ├── Dashboard/
│   │   │   ├── StrongestMovers.tsx
│   │   │   ├── HighestVolume.tsx
│   │   │   └── AlphaSignals.tsx
│   │   ├── Charts/
│   │   │   ├── PriceChart.tsx
│   │   │   ├── IndicatorChart.tsx
│   │   │   └── PredictionChart.tsx
│   │   ├── Tables/
│   │   │   ├── AssetTable.tsx
│   │   │   └── SignalTable.tsx
│   │   └── shared/
│   │       ├── Navbar.tsx
│   │       └── Sidebar.tsx
│   ├── hooks/
│   │   ├── useWebSocket.ts
│   │   ├── useMarketData.ts
│   │   └── usePredictions.ts
│   ├── services/
│   │   └── api.ts
│   ├── store/
│   │   └── index.ts
│   ├── types/
│   │   └── index.ts
│   ├── utils/
│   │   └── formatters.ts
│   ├── App.tsx
│   └── main.tsx
├── public/
├── package.json
├── tsconfig.json
├── vite.config.ts
└── Dockerfile
```

**Dashboard Layout:**
```
+----------------------------------------------------------+
|  Header: Financial Forecast Dashboard                    |
+----------------------------------------------------------+
| Sidebar |  Main Content Area                             |
|         |                                                |
| Assets  |  +------------------------------------------+  |
|         |  |  Strongest Movers (Top 10)               |  |
| - NQ    |  |  Rank | Asset | Change | Volume | Signal |  |
| - NVDA  |  +------------------------------------------+  |
| - AMD   |                                                |
| - ...   |  +------------------------------------------+  |
|         |  |  Highest Volume (Top 10)                 |  |
| Filters |  |  Rank | Asset | Volume | Value | Trend  |  |
|         |  +------------------------------------------+  |
| [Time] |                                                |
| [Type] |  +------------------------------------------+  |
|         |  |  Alpha Signals (Highest Probability)     |  |
|         |  |  Asset | Signal | Confidence | Return    |  |
|         |  +------------------------------------------+  |
|         |                                                |
|         |  [Price Chart | Technical Indicators]        |
|         |                                                |
+----------------------------------------------------------+
```

---

## Data Flow

### 1. Data Ingestion Flow
```
Market Data Sources
    ↓
Data Pipeline (Python)
    ├─→ Validation (schema checks, outlier detection)
    ├─→ Technical Indicators Calculation
    └─→ Normalization
         ↓
    Redis Storage (with 14-day TTL)
         ↓
    Java Backend (reads on demand)
         ↓
    Web UI (real-time updates)
```

### 2. ML Model Training Flow
```
Redis (historical data)
    ↓
Data Pipeline (feature engineering)
    ↓
Rust Model Trainer
    ├─→ Random Forest Training
    ├─→ Gaussian Copula Fitting
    └─→ Model Evaluation
         ↓
    Model Artifacts (serialized)
         ↓
    Redis (predictions cache)
         ↓
    Java Backend (serves predictions)
```

### 3. Alpha Discovery Flow
```
Search Algorithms (Betafish & Exa)
    ├─→ Web Search (financial news)
    ├─→ Content Analysis (NLP)
    └─→ Signal Extraction
         ↓
    Alpha Signals (scored & ranked)
         ↓
    Redis Storage
         ↓
    Web UI (display top signals)
```

---

## Deployment Strategy

### Docker Compose Architecture
```yaml
services:
  redis:
    image: redis:7-alpine
    command: redis-server /usr/local/etc/redis/redis.conf
    volumes:
      - redis-data:/data
      - ./config/redis.conf:/usr/local/etc/redis/redis.conf
    ports:
      - "6379:6379"

  data-pipeline:
    build: ./data-pipeline
    depends_on:
      - redis
    environment:
      - REDIS_HOST=redis
      - REDIS_PORT=6379
    restart: unless-stopped

  rust-models:
    build: ./rust-models
    depends_on:
      - redis
    environment:
      - REDIS_HOST=redis
    restart: unless-stopped

  search-algorithms:
    build: ./search-algorithms
    depends_on:
      - redis
    environment:
      - REDIS_HOST=redis
      - EXA_API_KEY=${EXA_API_KEY}
    restart: unless-stopped

  java-backend:
    build: ./java-backend
    depends_on:
      - redis
    ports:
      - "8080:8080"
    environment:
      - REDIS_HOST=redis
      - SPRING_PROFILES_ACTIVE=production
    restart: unless-stopped

  ui:
    build: ./ui
    depends_on:
      - java-backend
    ports:
      - "3000:80"
    restart: unless-stopped
```

### Scaling Strategy
- **Redis**: Redis Cluster for horizontal scaling (future)
- **Java Backend**: Kubernetes HPA based on CPU/memory
- **Rust Models**: Multiple instances for parallel inference
- **Data Pipeline**: Partitioned by asset class
- **Search Algorithms**: Async workers with rate limiting

### Monitoring & Observability
- **Metrics**: Prometheus + Grafana
- **Logging**: ELK Stack (Elasticsearch, Logstash, Kibana)
- **Tracing**: Jaeger for distributed tracing
- **Alerts**: PagerDuty for critical failures

### CI/CD Pipeline
```
GitHub Actions
    ├─→ Linting & Code Quality
    ├─→ Unit Tests
    ├─→ Integration Tests
    ├─→ Build Docker Images
    ├─→ Push to Registry
    └─→ Deploy to Staging/Production
```

---

## Security Considerations

1. **API Security**:
   - JWT-based authentication
   - Rate limiting per user
   - Input validation & sanitization

2. **Redis Security**:
   - AUTH password enabled
   - TLS encryption in production
   - Network isolation (private VPC)

3. **Data Security**:
   - No sensitive data in logs
   - Environment variables for secrets
   - Regular security audits

4. **API Keys**:
   - Exa API key stored in secrets manager
   - Regular rotation of credentials
   - Separate keys for dev/staging/prod

---

## Performance Targets

| Metric | Target | Notes |
|--------|--------|-------|
| API Latency (p50) | < 50ms | Java backend response time |
| API Latency (p99) | < 200ms | 99th percentile |
| Model Inference | < 100ms | Rust model prediction |
| Data Pipeline | < 5s | End-to-end processing |
| Redis Operations | < 1ms | GET/SET operations |
| UI Refresh | Real-time | WebSocket updates |

---

## Future Enhancements

1. **Model Improvements**:
   - Deep learning models (LSTM, Transformer)
   - Ensemble methods
   - Online learning for concept drift

2. **Additional Features**:
   - Portfolio optimization
   - Risk management dashboards
   - Backtesting framework
   - Strategy automation

3. **Infrastructure**:
   - Kubernetes deployment
   - Multi-region redundancy
   - Real-time streaming (Kafka)
   - Feature store (Feast)

---

## Troubleshooting Guide

### Redis Memory Issues
- Monitor with `INFO memory`
- Check key expiration with `TTL key`
- Adjust `maxmemory-policy` as needed

### Model Performance Degradation
- Check feature distributions (data drift)
- Retrain models regularly
- Monitor prediction errors
- A/B test new model versions

### Data Pipeline Backlogs
- Scale pipeline workers
- Optimize indicator calculations
- Batch writes to Redis
- Monitor processing latency

---

## Conclusion

This architecture provides a scalable, production-ready system for financial forecasting and alpha discovery. The modular design allows for independent development and deployment of each component while maintaining clear interfaces and data flows.

Key strengths:
- **Scalability**: Horizontal scaling ready
- **Reliability**: Redis persistence + container orchestration
- **Performance**: Rust models for fast inference
- **Flexibility**: Easy to add new models and data sources
- **Observability**: Comprehensive monitoring and logging
