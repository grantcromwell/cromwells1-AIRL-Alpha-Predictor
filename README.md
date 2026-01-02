**README: Financial Forecasting System - FinAlpha**

**1. Introduction**

FinAlpha is a scalable and robust backend system designed for analyzing equity data to identify promising alpha opportunities using Random Forests and Gaussian Processes. This system leverages a hybrid architecture, 
integrating data ingestion, model training, orchestration, and a user-friendly dashboard for visualizing and exploring investment opportunities.  It’s built for rapid prototyping and production-ready scalability.

**2. System Overview**

FinAlpha provides a comprehensive environment for financial forecasting, automating the process of feature engineering, model training, and risk assessment. It’s designed to be easily deployed and scaled, supporting both 
local development and production environments.  The core components include:

* **Data Layer:** Stores raw and processed data.
* **ML Models:** Implements Random Forests and Gaussian Copulas for predictive analysis.
* **Orchestration:**  Manages model training, data preparation, and pipeline orchestration.
* **UI:**  A web dashboard for interactive visualization and monitoring.

**3. Architecture**

The system adopts a hybrid architecture to balance speed and accuracy:

* **Data Layer:**
    * **Redis (Data Storage):** Low-latency data storage for real-time data retrieval and caching (5-7 seconds retention).  Crucial for rapid iteration.
    * **Java + Redis:**  Data ingestion, processing, and storage (2-week retention).  Handles large datasets efficiently.
    * **Rust (ML Models):**  Runtime environment for the Random Forest and Gaussian Copula models.
    * **Python (Orchestration):**  For workflow management, model training, and data preparation.
* **ML Models:**
    * **Random Forest:**  A classification and regression model suitable for predicting stock price movements.  Supports various feature engineering techniques.
    * **Gaussian Copula:**  A probabilistic model to model asset dependencies and identify risk correlations.  Provides statistical modeling and portfolio optimization capabilities.
    * **Search Algorithms (Betafish & Exa):**  For alpha discovery by identifying promising securities based on Sharpe ratio and confidence thresholds.
* **Orchestration:**
    * **Python (Workflow):** Automates the entire ML pipeline, including data preparation, model training, validation, and deployment. Uses `exec:java` for command-line tasks.
* **UI:**
    * **HTML/JavaScript (Dashboard):** A web-based dashboard for displaying charts, statistics, and historical data. Built with Chart.js.
    * **REST API (Application):** Accessible via a RESTful API.
* **Docker Services:** Provides containerized access to each component, simplifying deployment and management.

**4. Key Components - Detailed Breakdown**

* **4.1. Data Layer**
    * **Redis Configuration:** Stores relevant data such as historical prices, indicators, and technical stats.
    * **Data Source:**  Can be dynamically pulled from external data providers or built from data within the system (via a data pipeline).
* **4.2. ML Models**
    * **Random Forest:** Trained on historical data. Used for various predictions (e.g., price forecasting, anomaly detection).
    * **Gaussian Copula:** Used to model dependencies between different assets, improving risk management.
    * **Betafish Search:** Utilizes the Sharpe ratio to filter for superior alpha opportunities.
* **4.3. Orchestration**
    * **Python Script (Main):** Executes the entire model lifecycle.
    * **`build.sh`:**  A script to automatically set up the project environment.
    * **`mvn exec:java -Dexec.mainClass="com.financial.backend.Main" -Dexec.args="-Xmx2G" `:** Runs the Java backend to train the models.
* **4.4. UI**
    * **Chart.js:** Used for creating charts and visualizations of model results.
    * **Web Dashboard:** Provides interactive data exploration and monitoring.
* **4.5.  API**
    * **REST API:** Allows users to interact with the system through standard web requests.
* **4.6.  Model Details**
    * **Random Forest:** Selected due to its high accuracy and relatively fast training time.
    * **Gaussian Copula:** Leverages the ability to model complex, non-linear relationships between asset correlations.
    * **Betafish Search:** Measures and uses the Sharpe ratio of a portfolio.
    * **Exa Search:** Focuses on predicting portfolio performance based on confidence levels.

**5.  Features**

* **5.1. Data Management:** Automatic data retention with configurable TTLs.
* **5.2. Feature Engineering:**  Includes moving averages, RSI, MACD, volatility, and recent price data.
* **5.3. Random Forest:** 100 trees, 15 engineered features, ensemble learning.
* **5.4. Gaussian Copula:** Asset dependency modeling, probability distributions.
* **5.5. Search Algorithms (Betafish & Exa):** Sharpe ratio-based alpha discovery.
* **5.6. REST API:** Full CRUD operations for equity data.
* **5.7. Web Dashboard:** Real-time charts, statistical summaries, historical data, and model monitoring.
* **5.8.  Historical Data Integration:** Future improvements will include the ability to import data from external sources.

**6. Prerequisites**

* **Java 17+ / Maven 3.6+:** Required for the Java backend.
* **Rust 1.75+ / Cargo:** Required for the Rust models.
* **Redis 6.0+:**  Required for data storage.
* **Python 3.8+ (for orchestration script):**  Required for the orchestration script.
* **Docker Engine 20.10+:** Required for containerization.

**7.  Testing**

* **Java Tests:**  Run the Java backend's test suite.
* **Rust Tests:** Run the Rust test suite.
* **Build All:** Run the build script to create the project environment.

**8.  Monitoring**

* **Docker Monitoring:** Track container status and resource usage.
* **Redis Monitoring:** Check Redis connection and health.
* **API Health:** Monitor the API endpoint.
* **Logging:**  Utilize a logging library (e.g., `loguru`) to centralize and manage log files.

**9.  Security**

* **API Authentication:** Implement authentication to protect API endpoints.
* **Redis AUTH:**  Enable the Redis AUTH mechanism to restrict access to data.
* **HTTPS:**  Enable HTTPS for the API to secure data transmission.
* **Validation:** Validate all inputs at the API level.

**10.  Documentation**

* **README:**  Provide an overview of the project, its components, and how to get started.
* **API Documentation:**  For the REST API, document endpoints, request/response formats, and authentication.
* **Code Comments:**  Add clear and concise comments to the code to improve readability and maintainability.

**11.  License**

*   MIT License

---


## Testing

### Java Tests

```bash
cd java-backend
mvn test
```

### Rust Tests

```bash
cd rust-model
cargo test
```

### Build All

```bash
./build.sh
```

## Monitoring

### Docker Monitoring

```bash
# Container status
docker-compose ps

# Resource usage
docker stats

# Service logs
docker-compose logs -f <service>
```

### API Health

```bash
curl http://localhost:8080/api/health
```

### Redis Monitoring

```bash
redis-cli ping
redis-cli INFO
```

## Configuration

### Environment Variables (Docker)

```yaml
environment:
  - REDIS_HOST=redis
  - REDIS_PORT=6379
  - JAVA_API_URL=http://java-backend:8080
```

### Redis TTL

- Default: 2 weeks (1,209,600 seconds)
- Automatic cleanup via TTL expiration

## Troubleshooting

### Redis Connection Failed

```bash
# Check Redis status
docker-compose exec redis redis-cli ping

# Or native
redis-cli ping
```

### Port Already in Use

```bash
# Find process using port
lsof -i :8080

# Change port in docker-compose.yml or EquityApiServer.java
```

### Out of Memory

```bash
# Increase JVM heap
mvn exec:java -Dexec.mainClass="com.financial.backend.Main" -Dexec.args="-Xmx2G"
```

### Rust Build Failed

```bash
# Update Rust
rustup update

# Clean build
cd rust-model
cargo clean
cargo build --release
```

### Docker Issues

```bash
# Check Docker daemon
docker version

# View Docker logs
docker logs <container_name>

# Restart Docker
sudo systemctl restart docker
```

## Performance

- Redis TTL ensures automatic 2-week data cleanup
- Batch operations for efficient storage
- Random Forest uses parallel processing
- Gaussian Copula runs 10,000 simulations
- Nginx serves static UI files

## Security

- API endpoints lack authentication (add for production)
- Enable Redis AUTH for production
- Enable HTTPS for production API
- Validate all inputs in production

## License

MIT License

## Acknowledgments

- Random Forest inspired by scikit-learn
- Gaussian Copula based on financial modeling literature
- UI built with Chart.js
- Redis for fast data storage
