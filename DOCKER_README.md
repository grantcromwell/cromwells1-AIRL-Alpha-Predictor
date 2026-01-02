# Financial Forecasting System - Docker Setup

## Quick Start

### Build and Start All Services

```bash
docker-compose up -d --build
```

### View Logs

```bash
# All logs
docker-compose logs -f

# Specific service
docker-compose logs -f java-backend
docker-compose logs -f rust-models
docker-compose logs -f redis
```

### Stop All Services

```bash
docker-compose down

# Stop and remove volumes
docker-compose down -v
```

## Services

| Service | Port | Description |
|---------|------|-------------|
| redis | 6379 | Data storage (2-week retention) |
| java-backend | 8080 | REST API, data ingestion |
| rust-models | - | ML models (batch processing) |
| ui | 8081 | Web dashboard |

## Access Points

- **API**: http://localhost:8080
- **Health**: http://localhost:8080/api/health
- **UI Dashboard**: http://localhost:8081
- **Redis**: localhost:6379

## API Endpoints

```bash
# Health check
curl http://localhost:8080/api/health

# Get all data
curl http://localhost:8080/api/equity/all

# Get symbol data
curl "http://localhost:8080/api/equity/symbol?symbol=NVDA&limit=50"
```

## Configuration

Edit `docker-compose.yml` to modify:
- Port mappings
- Environment variables
- Volume mounts
- Resource limits

### Java Backend Environment

```yaml
environment:
  - REDIS_HOST=redis
  - REDIS_PORT=6379
  - JAVA_OPTS=-Xmx512m
```

### Rust Models Environment

```yaml
environment:
  - JAVA_API_URL=http://java-backend:8080
  - RUST_LOG=info
```

## Troubleshooting

### Redis Connection Failed

```bash
# Check Redis status
docker-compose exec redis redis-cli ping

# View Redis logs
docker-compose logs redis
```

### Java Backend Not Starting

```bash
# Check Java logs
docker-compose logs java-backend

# Check if Redis is healthy
docker-compose ps
```

### Rust Models Failing

```bash
# Check Rust logs
docker-compose logs rust-models

# Rebuild Rust service
docker-compose build rust-models
docker-compose up -d rust-models
```

## Production Deployment

### Build Production Images

```bash
docker-compose -f docker-compose.yml -f docker-compose.prod.yml build
```

### Scale Services

```bash
# Scale Java backend
docker-compose up -d --scale java-backend=2

# Scale with custom config
docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d
```

### Monitoring

```bash
# Resource usage
docker stats

# Container health
docker-compose ps
```

## Logs

Logs are stored in `./logs/` directory:
- `redis/redis.log` - Redis operations
- `java-backend/app.log` - Java application logs
- `rust-models/` - Rust model output

## Cleanup

```bash
# Remove all containers, networks, and volumes
docker-compose down -v

# Remove images
docker-compose down --rmi all

# Complete cleanup
docker system prune -a
```
