# Zklear Payment System - Docker Deployment

This document provides comprehensive instructions for deploying the Zklear Payment System using Docker.

## 🐳 Overview

The Zklear Payment System is containerized using a multi-stage Docker build that includes:
- **Rust Backend**: Zero-knowledge payment processing with RISC Zero
- **React Frontend**: Modern web interface for interacting with the system
- **API Server**: RESTful API for account and transaction management

## 📋 Prerequisites

- **Docker**: Version 20.10 or higher
- **Docker Compose**: Version 2.0 or higher
- **Git**: For cloning the repository

## 🚀 Quick Start

### Option 1: Automated Deployment (Recommended)

```bash
# Clone the repository (if not already done)
git clone <your-repo-url>
cd ocl

# Run the automated deployment script
./deploy.sh
```

This script will:
- Check Docker installation
- Create necessary directories and files
- Build and start the application
- Verify the deployment
- Display access URLs

### Option 2: Manual Deployment

```bash
# Build and start the application
docker-compose up --build -d

# Check the status
docker-compose ps

# View logs
docker-compose logs -f
```

## 🌐 Access Points

Once deployed, you can access the application at:

- **React Frontend**: http://localhost:3000
- **API Server**: http://localhost:8081
- **API Documentation**: http://localhost:3000/api-spec

## 📁 Project Structure

```
ocl/
├── Dockerfile              # Multi-stage Docker build
├── docker-compose.yml      # Container orchestration
├── deploy.sh              # Automated deployment script
├── src/                   # Rust backend source
├── methods/               # RISC Zero guest code
├── zklear-frontend/      # React frontend
├── accounts.json         # Account data (persistent)
├── transactions.json     # Transaction data (persistent)
└── data/                # Additional persistent data
```

## 🔧 Configuration

### Environment Variables

The following environment variables can be configured in `docker-compose.yml`:

- `RUST_LOG`: Logging level (default: `info`)
- `DATA_DIR`: Data directory path (default: `/app/data`)

### Port Configuration

Default ports (can be changed in `docker-compose.yml`):
- **8081**: API server
- **3000**: React frontend

### Volume Mounts

The following directories are mounted for data persistence:
- `./data:/app/data`: General application data
- `./accounts.json:/app/accounts.json`: Account information
- `./transactions.json:/app/transactions.json`: Transaction history

## 🛠️ Management Commands

### View Application Status
```bash
docker-compose ps
```

### View Logs
```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f zklear
```

### Stop Application
```bash
docker-compose down
```

### Restart Application
```bash
docker-compose restart
```

### Update Application
```bash
# Pull latest changes and rebuild
git pull
docker-compose up --build -d
```

### Access Container Shell
```bash
docker-compose exec zklear bash
```

## 🔍 Troubleshooting

### Common Issues

1. **Port Already in Use**
   ```bash
   # Check what's using the ports
   lsof -i :8081
   lsof -i :3000
   
   # Stop conflicting services
   sudo pkill -f "cargo run"
   ```

2. **Build Failures**
   ```bash
   # Clean Docker cache
   docker system prune -a
   
   # Rebuild without cache
   docker-compose build --no-cache
   ```

3. **Permission Issues**
   ```bash
   # Fix file permissions
   sudo chown -R $USER:$USER data/
   chmod 644 accounts.json transactions.json
   ```

### Health Checks

The application includes health checks that monitor:
- API server availability
- Container status
- Service responsiveness

### Log Analysis

```bash
# View recent logs
docker-compose logs --tail=100

# Search for errors
docker-compose logs | grep -i error

# Monitor real-time logs
docker-compose logs -f --tail=50
```

## 🔒 Security Considerations

- **Non-root User**: The container runs as a non-root user (`appuser`)
- **Minimal Base Image**: Uses `debian:bookworm-slim` for smaller attack surface
- **Health Checks**: Regular health monitoring for service availability
- **Volume Permissions**: Proper file permissions for data persistence

## 📊 Monitoring

### Container Metrics
```bash
# View resource usage
docker stats

# Check container health
docker-compose ps
```

### Application Health
```bash
# API health check
curl http://localhost:8081/api/system-info

# Frontend availability
curl http://localhost:3000
```

## 🚀 Production Deployment

For production deployment, consider:

1. **Reverse Proxy**: Use nginx or traefik for SSL termination
2. **Load Balancing**: Multiple container instances
3. **Database**: External database for data persistence
4. **Monitoring**: Prometheus/Grafana for metrics
5. **Logging**: Centralized logging with ELK stack

### Example Production docker-compose.yml

```yaml
version: '3.8'
services:
  zklear:
    build: .
    restart: always
    environment:
      - RUST_LOG=warn
    volumes:
      - zklear-data:/app/data
    networks:
      - zklear-network
    deploy:
      replicas: 2
      resources:
        limits:
          memory: 1G
          cpus: '0.5'

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
    depends_on:
      - zklear

volumes:
  zklear-data:

networks:
  zklear-network:
    driver: bridge
```

## 📚 Additional Resources

- [Docker Documentation](https://docs.docker.com/)
- [Docker Compose Documentation](https://docs.docker.com/compose/)
- [RISC Zero Documentation](https://dev.risczero.com/)
- [React Documentation](https://reactjs.org/docs/)

## 🤝 Support

For issues or questions:
1. Check the troubleshooting section above
2. Review application logs: `docker-compose logs`
3. Verify Docker installation: `docker --version`
4. Check system resources: `docker system df` 