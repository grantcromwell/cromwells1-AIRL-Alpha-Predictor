# Docker Installation

## Install Docker

### Ubuntu
```bash
# Install Docker
sudo apt-get update
sudo apt-get install -y apt-transport-https ca-certificates curl software-properties-common
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
sudo add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable"
sudo apt-get update
sudo apt-get install -y docker-ce docker-ce-cli containerd.io

# Start Docker
sudo systemctl start docker
sudo systemctl enable docker

# Add user to docker group
sudo usermod -aG docker $USER
```

### macOS
```bash
# Install Docker Desktop
brew install --cask docker
# Then open Docker Desktop from Applications
```

### Windows
```bash
# Install Docker Desktop for Windows
# Download from https://www.docker.com/products/docker-desktop
# Then run Docker Desktop
```

## Install Docker Compose

### Linux
```bash
sudo curl -L "https://github.com/docker/compose/releases/download/v2.24.0/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
```

### macOS/Windows
Docker Compose is included with Docker Desktop.

## Verify Installation

```bash
docker --version
docker-compose --version
```

## Quick Start with Docker

```bash
# Build all images
docker-compose build

# Start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop all services
docker-compose down
```

## Access Services

- **API**: http://localhost:8080
- **UI**: http://localhost:8081
- **Redis**: localhost:6379

## Files Created

```
sugi1/
├── docker-compose.yml         # Main Docker configuration
├── java-backend/
│   └── Dockerfile            # Java backend image
├── rust-model/
│   └── Dockerfile           # Rust models image
└── DOCKER_README.md         # Detailed Docker documentation
```
